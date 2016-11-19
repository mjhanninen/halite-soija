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

use std::collections::{HashMap, HashSet};
use std::fs::File;

use ua::io;
use ua::map::{Map, Site};
use ua::space::{Dir, Pos};
use ua::util::LoggedUnwrap;
use ua::world::State;

#[allow(dead_code)]
fn calc_occupations(map: &Map, who: u8) -> HashSet<Pos>
{
    map.space
       .sweep()
       .filter_map(|pos| {
           let site = map.site(&pos);
           if site.owner == who {
               Some(map.space.normalize(&pos))
           } else {
               None
           }
       })
       .collect::<HashSet<_>>()
}

fn tick_site(pos: &Pos, src: &Site, map: &Map, me: u8) -> Option<Dir>
{
    for n in pos.neighbors() {
        let tgt = map.site(&n);
        if tgt.owner != me && tgt.strength < src.strength {
            return Some(map.space.direction(pos, &n));
        }
    }
    for n in pos.neighbors() {
        let tgt = map.site(&n);
        if tgt.owner == me && 2 * (tgt.strength as i16) < src.strength as i16 {
            return Some(map.space.direction(pos, &n));
        }
    }
    None
}

fn tick(map: &Map, me: u8) -> HashMap<Pos, Dir>
{
    // let occupations = calc_occupations(map, me);
    // let gen0 = Onion::from_set(&map.space, &occupations);
    // let gen1 = gen0.expand();
    let mut moves = HashMap::new();
    for p in map.space.sweep() {
        let source = map.site(&p);
        if source.owner == me {
            if let Some(dir) = tick_site(&p, source, map, me) {
                moves.insert(p, dir);
            }
        }
    }
    moves
}

pub fn run()
{
    let mut log_file = File::create("runtime.log").unwrap();
    let mut connection = io::Connection::new();
    let environment = connection.recv_environment()
                                .unwrap_or_log(&mut log_file);
    let mut state_frame = State::for_environment(&environment);
    connection.recv_state(&environment, &mut state_frame)
              .unwrap_or_log(&mut log_file);
    // You've got 15 seconds to spare on this line. Use it well.
    connection.send_ready(&environment, "UmpteenthAnion")
              .unwrap_or_log(&mut log_file);
    loop {
        connection.recv_state(&environment, &mut state_frame)
                  .unwrap_or_log(&mut log_file);
        let map = Map::from_world(&environment, &state_frame);
        let moves = tick(&map, environment.my_tag)
            .iter()
            .map(|(pos, dir)| (pos.clone(), Some(dir.clone())))
            .collect::<Vec<_>>();
        connection.send_moves(moves.iter())
                  .unwrap_or_log(&mut log_file);
    }
}
