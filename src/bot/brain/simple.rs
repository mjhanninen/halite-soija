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

use std::fs::File;

use ua::action::Action;
use ua::dir::Dir;
use ua::io;
use ua::map::Map;
use ua::space::Frame;
use ua::util::LoggedUnwrap;
use ua::world::{Environment, Occupation, State};

use params::Params;

fn tick_site(origin: &Frame,
             src: &Occupation,
             occupations: &Map<Occupation>,
             me: u8)
    -> Option<Dir>
{
    for d in Dir::dirs() {
        let f = origin.adjacent_in(d);
        let tgt = &occupations[f.ix()];
        if tgt.tag != me && tgt.strength < src.strength {
            return Some(d);
        }
    }
    for d in Dir::dirs() {
        let f = origin.adjacent_in(d);
        let tgt = &occupations[f.ix()];
        if tgt.tag == me && 2 * (tgt.strength as i16) < src.strength as i16 {
            return Some(d);
        }
    }
    None
}

fn tick(environment: &Environment, state: &State) -> Vec<Action>
{
    let me = environment.my_tag;
    let mut actions = vec![];
    for f in environment.space.frames() {
        let source = &state.occupation_map[f.ix()];
        if source.tag == me {
            if let Some(dir) = tick_site(&f,
                                         source,
                                         &state.occupation_map,
                                         me) {
                actions.push((f.coord(), Some(dir)));
            }
        }
    }
    actions
}

pub fn run(_: &Params)
{
    let mut log_file = File::create("runtime.log").unwrap();
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment()
                                .unwrap_or_log(&mut log_file);
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&mut state_frame)
              .unwrap_or_log(&mut log_file);
    // You've got 15 seconds to spare on this line. Use it well.
    connection.send_ready(&environment.my_tag, "UmpteenthAnion")
              .unwrap_or_log(&mut log_file);
    loop {
        connection.recv_state(&mut state_frame)
                  .unwrap_or_log(&mut log_file);
        let actions = tick(&environment, &state_frame);
        connection.send_actions(actions.iter())
                  .unwrap_or_log(&mut log_file);
    }
}
