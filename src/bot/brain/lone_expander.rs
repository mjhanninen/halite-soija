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
use ua::space::Pos;
use ua::world::{Environment, Move, State};

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

    fn init(&mut self, _environment: &Environment, _state: &State)
    {
        // To be done
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
