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

use std::f32;

use ua::action::Action;
use ua::dir::Dir;
use ua::io;
use ua::map::Map;
use ua::space::Frame;
use ua::util::f32_cmp;
use ua::world::{Environment, Occupation, State, Tag};

use params::Params;

const DEFAULT_DISCOUNT_FACTOR: f32 = 0.55;
const DEFAULT_EXPANSION_WEIGHT: f32 = 80.0;
const DEFAULT_AGGRESSION_WEIGHT: f32 = 50.0;
const DEFAULT_DENSITY_WEIGHT: f32 = 70.0;
const DEFAULT_MINIMUM_MOVABLE_STRENGTH: f32 = 40.0;

const MAX_STR: f32 = 255.0;

#[inline]
fn f32_max(a: f32, b: f32) -> f32
{
    if a >= b {
        a
    } else {
        b
    }
}

struct Mold;

struct Brain
{
    // Static environment
    environment: Environment,
    // Parameters
    aggression_weight: f32,
    density_weight: f32,
    discount_factor: f32,
    expansion_weight: f32,
    minimum_movable_strength: f32,
}

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

    fn reanimate(&self,
                 params: &Params,
                 environment: Environment,
                 _: &State)
        -> Brain
    {
        Brain {
            environment: environment,
            aggression_weight: *params.get("aggression_weight")
                                      .unwrap_or(&DEFAULT_AGGRESSION_WEIGHT),
            density_weight: *params.get("density_weight")
                                   .unwrap_or(&DEFAULT_DENSITY_WEIGHT),
            discount_factor: *params.get("discount_factor")
                                    .unwrap_or(&DEFAULT_DISCOUNT_FACTOR),
            expansion_weight: *params.get("expansion_weight")
                                     .unwrap_or(&DEFAULT_EXPANSION_WEIGHT),
            minimum_movable_strength:
                *params.get("minimum_movable_strength")
                       .unwrap_or(&DEFAULT_MINIMUM_MOVABLE_STRENGTH),
        }
    }
}

fn perpetuity(discount_factor: f32) -> f32
{
    assert!(discount_factor < 1.0);
    1.0 / (1.0 - discount_factor)
}

#[allow(dead_code)]
struct Choice<'a>
{
    source: Frame<'a>,
    target: Frame<'a>,
    utility: f32,
}

impl Brain
{
    fn calc_density_map(&self,
                        who: Tag,
                        occupations: &Map<Occupation>)
        -> Map<f32>
    {
        let mut densities = vec![0.0; occupations.len()];
        for f in self.environment.space.frames() {
            let mut df_mass = 0.0;
            let mut pop_mass = 0.0;
            for g in self.environment.space.frames() {
                let d = f.l1_norm(&g);
                let df = self.discount_factor.powi(d as i32);
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

    fn calc_ownership_map(&self,
                          who: Tag,
                          discount_factor: f32,
                          occupations: &Map<Occupation>)
        -> Map<f32>
    {
        let space = &self.environment.space;
        let productions = &self.environment.production_map;
        let ln_df = discount_factor.ln();
        let mut ownerships = vec![0.0; occupations.len()];
        for f in space.frames() {
            let mass = space.frames()
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

    fn calc_blood_map(&self,
                      who: Tag,
                      discount_factor: f32,
                      occupations: &Map<Occupation>)
        -> Map<f32>
    {
        let space = &self.environment.space;
        let ln_df = discount_factor.ln();
        let mut blood = vec![0.0; occupations.len()];
        for f in space.frames() {
            let mass = space.frames()
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

    fn select_cell_action(&self,
                          who: Tag,
                          loc: Frame,
                          state: &State,
                          densities: &Map<f32>,
                          ownerships: &Map<f32>,
                          blood: &Map<f32>)
        -> Action
    {
        let productions = &self.environment.production_map;
        let occupations = &state.occupation_map;
        let o_src = loc.on(&occupations);
        let d_src = *loc.on(densities);
        let e_src = *loc.on(ownerships);
        let b_src = *loc.on(blood);
        let str_src = o_src.strength as f32;
        let prod_src = *loc.on(productions) as f32;
        let mut utilities = Vec::with_capacity(5);
        assert!(d_src.is_finite());
        // Utility for staying put
        {
            let utility = 10.0 * prod_src *
                          ((MAX_STR - str_src) / MAX_STR).powi(4);
            utilities.push((utility, None));
        }
        // Utilities for moving
        for d in Dir::dirs() {
            let p = loc.adjacent_in(d);
            let o_tgt = p.on(&occupations);
            let d_tgt = *p.on(densities);
            let e_tgt = *p.on(ownerships);
            let b_tgt = *p.on(blood);
            let str_tgt = o_tgt.strength as f32;
            let density_value = -self.density_weight *
                                (d_tgt.powi(4) - d_src.powi(4));
            let prospect_value = self.expansion_weight * (e_tgt - e_src);
            let aggression_change = self.aggression_weight * (b_tgt - b_src);
            let u = if o_tgt.tag == who {
                if str_src < self.minimum_movable_strength {
                    f32::NEG_INFINITY
                } else {
                    prospect_value + density_value + aggression_change -
                    f32_max(0.0, str_tgt + str_src - MAX_STR)
                }
            } else {
                if o_tgt.strength < o_src.strength {
                    let acquisition_value = *p.on(productions) as f32 *
                                            self.discount_factor *
                                            perpetuity(self.discount_factor);
                    let acquisition_cost =
                        0.5 * (o_src.strength - o_tgt.strength) as f32;
                    let territory_reward = self.expansion_weight;
                    let aggression_reward = if o_tgt.tag != 0 {
                        self.aggression_weight
                    } else {
                        0.0
                    };
                    acquisition_value - acquisition_cost + territory_reward +
                    prospect_value +
                    density_value + aggression_change +
                    aggression_reward
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

    #[inline]
    fn me(&self) -> Tag
    {
        self.environment.my_tag
    }

    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        let densities = self.calc_density_map(self.me(), &state.occupation_map);
        let ownerships = self.calc_ownership_map(self.me(),
                                                 self.discount_factor,
                                                 &state.occupation_map);
        let blood = self.calc_blood_map(self.me(),
                                        self.discount_factor,
                                        &state.occupation_map);
        let mut actions = vec![];
        for f in self.environment
                     .space
                     .frames() {
            let source = f.on(&state.occupation_map);
            if source.tag == self.me() {
                actions.push(self.select_cell_action(self.me(),
                                                     f,
                                                     state,
                                                     &densities,
                                                     &ownerships,
                                                     &blood));
            }
        }
        actions
    }
}

pub fn run(params: &Params)
{
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment().unwrap();
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&mut state_frame).unwrap();
    let mold = Mold::new();
    let my_tag = environment.my_tag;
    let mut brain = mold.reanimate(params, environment, &state_frame);
    connection.send_ready(&my_tag, &format!("UA_{}", mold.name()))
              .unwrap();
    loop {
        connection.recv_state(&mut state_frame).unwrap();
        let actions = brain.tick(&state_frame);
        connection.send_actions(actions.iter()).unwrap();
    }
}
