// Copyright (C) 2016 Matti Hänninen
//
// This file is part of Umpteenth Anion.
//
// Umpteenth Anion is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Umpteenth Anion is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
// for more details.
//
// You should have received a copy of the GNU General Public License along
// with Umpteenth Anion.  If not, see <http://www.gnu.org/licenses/>.

use std::cmp::Ordering;
use std::f32;

use ua::action::Action;
use ua::dir::Dir;
use ua::io;
use ua::map::Map;
use ua::space::{Frame, Space};
use ua::world::{Environment, Occupation, Production, State, Tag};

const DISCOUNT_FACTOR: f32 = 0.5;
const TERRITORY_WEIGHT: f32 = 10.0;
const DENSITY_WEIGHT: f32 = 10.0;
const AGGRESSIVITY: f32 = 10.0;

fn calc_density_map(who: Tag,
                    space: &Space,
                    occupations: &Map<Occupation>)
    -> Map<f32>
{
    let radius = 10;
    let mut densities = vec![0.0; occupations.len()];
    for f in space.frames() {
        let mut df_mass = 0.0;
        let mut pop_mass = 0.0;
        for g in f.l0_neighbors(radius) {
            let d = f.l1_norm(&g);
            let df = DISCOUNT_FACTOR.powi(d as i32);
            df_mass += df;
            let o = g.on(occupations);
            if o.tag == who {
                pop_mass += df * o.strength as f32 / 255.0;
            }
        }
        *f.on_mut(&mut densities) = pop_mass / df_mass;
    }
    densities
}

fn calc_ownership_map(who: Tag,
                      radius: usize,
                      discount_factor: f32,
                      space: &Space,
                      productions: &Map<Production>,
                      occupations: &Map<Occupation>)
    -> Map<f32>
{
    let ln_df = discount_factor.ln();
    let mut ownerships = vec![0.0; occupations.len()];
    for f in space.frames() {
        let mass = f.l0_neighbors(radius)
                    .map(|g| {
                        let o = g.on(occupations);
                        if o.tag != who {
                            *g.on(productions) as f32 *
                            (f.l1_norm(&g) as f32 * ln_df).exp() *
                            perpetuity(discount_factor)
                        } else {
                            0.0
                        }
                    })
                    .sum();
        *f.on_mut(&mut ownerships) = mass;
    }
    ownerships
}

fn calc_blood_map(who: Tag,
                  radius: usize,
                  discount_factor: f32,
                  space: &Space,
                  occupations: &Map<Occupation>)
    -> Map<f32>
{
    let ln_df = discount_factor.ln();
    let mut blood = vec![0.0; occupations.len()];
    for f in space.frames() {
        let mass = f.l0_neighbors(radius)
                    .map(|g| {
                        let o = g.on(occupations);
                        if o.tag != who && o.tag != 0 {
                            (f.l1_norm(&g) as f32 * ln_df).exp()
                        } else {
                            0.0
                        }
                    })
                    .sum();
        *f.on_mut(&mut blood) = mass;
    }
    blood
}


struct Mold;

struct Brain;

impl Mold
{
    fn new() -> Self
    {
        Mold
    }

    fn name(&self) -> &'static str
    {
        "LoneExpander"
    }

    fn reanimate(&self, _: &Environment, _: &State) -> Brain
    {
        Brain
    }
}

#[inline]
fn f32_cmp(a: &f32, b: &f32) -> Ordering
{
    if a < b {
        Ordering::Less
    } else if b < a {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn perpetuity(discount_factor: f32) -> f32
{
    assert!(discount_factor < 1.0);
    1.0 / (1.0 - discount_factor)
}

fn select_cell_action(me: Tag,
                      loc: Frame,
                      state: &State,
                      productions: &Map<Production>,
                      densities: &Map<f32>,
                      ownerships: &Map<f32>,
                      blood: &Map<f32>)
    -> Action
{
    let occupations = &state.occupation_map;
    let o_src = loc.on(&occupations);
    let d_src = *loc.on(densities);
    let e_src = *loc.on(ownerships);
    let b_src = *loc.on(blood);
    let prod_src = *loc.on(productions) as f32;
    let mut utilities = Vec::with_capacity(5);
    assert!(d_src.is_finite());
    utilities.push((prod_src, None));
    for d in Dir::dirs() {
        let p = loc.adjacent_in(d);
        let o_tgt = p.on(&occupations);
        let d_tgt = *p.on(densities);
        let e_tgt = *p.on(ownerships);
        let b_tgt = *p.on(blood);
        let density_value = -DENSITY_WEIGHT * (d_tgt.powi(4) - d_src.powi(4));
        let prospect_value = TERRITORY_WEIGHT * (e_tgt - e_src);
        let aggression_change = AGGRESSIVITY * (b_tgt - b_src);
        let u = if o_tgt.tag == me {
            if o_src.strength < 5 || o_tgt.strength + o_src.strength > 255 {
                f32::NEG_INFINITY
            } else {
                prospect_value + density_value + aggression_change
            }
        } else {
            if o_tgt.strength < o_src.strength {
                let acquisition_value = *p.on(productions) as f32 *
                                        DISCOUNT_FACTOR *
                                        perpetuity(DISCOUNT_FACTOR);
                let acquisition_cost = 0.5 *
                                       (o_src.strength - o_tgt.strength) as f32;
                let territory_reward = TERRITORY_WEIGHT;
                let aggression_reward = if o_tgt.tag != 0 {
                    AGGRESSIVITY
                } else {
                    0.0
                };
                acquisition_value - acquisition_cost + territory_reward +
                prospect_value + density_value +
                aggression_change + aggression_reward
            } else {
                f32::NEG_INFINITY
            }
        };
        utilities.push((u, Some(d)));
    }
    utilities.sort_by(|a, b| f32_cmp(&b.0, &a.0));
    assert!(utilities[0].0.is_finite());
    (loc.coord(), utilities[0].1)
}

impl Brain
{
    fn tick(&mut self, environment: &Environment, state: &State) -> Vec<Action>
    {
        let me = environment.my_tag;
        let densities =
            calc_density_map(me, &environment.space, &state.occupation_map);
        let ownerships = calc_ownership_map(me,
                                            10,
                                            DISCOUNT_FACTOR,
                                            &environment.space,
                                            &environment.production_map,
                                            &state.occupation_map);
        let blood = calc_blood_map(me,
                                   10,
                                   DISCOUNT_FACTOR,
                                   &environment.space,
                                   &state.occupation_map);
        let mut actions = vec![];
        for f in environment.space.frames() {
            let source = f.on(&state.occupation_map);
            if source.tag == me {
                actions.push(select_cell_action(me,
                                                f,
                                                state,
                                                &environment.production_map,
                                                &densities,
                                                &ownerships,
                                                &blood));
            }
        }
        actions
    }
}

pub fn run()
{
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment().unwrap();
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&environment, &mut state_frame).unwrap();
    let mold = Mold::new();
    let mut brain = mold.reanimate(&environment, &state_frame);
    connection.send_ready(&environment, &format!("UA_{}", mold.name()))
              .unwrap();
    loop {
        connection.recv_state(&environment, &mut state_frame).unwrap();
        let actions = brain.tick(&environment, &state_frame);
        connection.send_actions(actions.iter()).unwrap();
    }
}
