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

use std::num::Wrapping;

use rand::{self, Rng};
use ua::coord::Coord;
use ua::io;
use ua::world::State;

use params::Params;

const ALPHA: f64 = 1000.0;
const BETA: f64 = 0.125;

fn tick<R: Rng>(iteration: i32, rng: &mut R) -> bool
{
    let work_load = (ALPHA * (iteration as f64 * BETA).exp()) as usize;
    let mut path = Wrapping(0);
    for _ in 0..work_load {
        path += Wrapping(rng.gen::<u64>())
    }
    path.0 & 1 == 0
}

pub fn run(_: &Params)
{
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment().unwrap();
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&mut state_frame).unwrap();
    connection.send_ready(&environment.my_tag, "UA_Probe").unwrap();
    let red_actions = vec![(Coord { x: 0, y: 0 }, None)];
    let black_actions = vec![(Coord { x: 1, y: 0 }, None)];
    let mut rng = rand::thread_rng();
    let mut iteration = 0;
    loop {
        iteration += 1;
        connection.recv_state(&mut state_frame).unwrap();
        let actions = if tick(iteration, &mut rng) {
            &red_actions
        } else {
            &black_actions
        };
        connection.send_actions(actions.iter()).unwrap();
    }
}
