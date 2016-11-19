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

struct Brain;

use ua::io;
use ua::space::{Pos, Space};
use ua::world::{Environment, Map, Move, Production, State};

fn discounting_convolution(discount_factor: f32,
                           space: &Space,
                           production_map: &Map<Production>)
    -> Map<f32>
{
    assert!(0.0 <= discount_factor && discount_factor <= 1.0);
    let m = production_map;
    let mut r = vec![0.0; m.len()];
    if discount_factor == 0.0 {
        for p in space.frames() {
            r[p.ix()] = m[p.ix()] as f32;
        }
    } else if discount_factor == 1.0 {
        let mut s = 0.0;
        for o in space.frames() {
            s += m[o.ix()] as f32;
        }
        for p in space.frames() {
            r[p.ix()] = s;
        }
    } else {
        let ln_df = discount_factor.ln();
        for p in space.frames() {
            let mut s = 0.0;
            for o in space.frames() {
                s += m[o.ix()] as f32 * (ln_df * p.l1_norm(&o) as f32).exp();
            }
            r[p.ix()] = s;
        }
    }
    r
}

impl Brain
{
    fn new() -> Self
    {
        Brain
    }

    fn name(&self) -> &'static str
    {
        "LoneExpander"
    }

    fn init(&mut self, environment: &Environment, _state: &State)
    {
        let _exploration_values =
            discounting_convolution(0.5,
                                    &environment.space,
                                    &environment.production_map);
    }

    fn tick(&mut self, _environment: &Environment, _state: &State) -> Vec<Move>
    {
        // To be done
        vec![(Pos { x: 0, y: 0 }, None)]
    }
}

pub fn run()
{
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment().unwrap();
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&environment, &mut state_frame).unwrap();
    let mut brain = Brain::new();
    brain.init(&environment, &state_frame);
    connection.send_ready(&environment, &format!("UA_{}", brain.name()))
              .unwrap();
    loop {
        connection.recv_state(&environment, &mut state_frame).unwrap();
        let moves = brain.tick(&environment, &state_frame);
        connection.send_moves(moves.iter()).unwrap();
    }
}
