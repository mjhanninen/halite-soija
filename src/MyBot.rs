// This line helps Emacs not to mix the following attributes with shebang.

#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate rand;

#[macro_use]
extern crate text_io;

mod ua;

use std::collections::{HashSet, HashMap};

use ua::map::{Map, Site};
use ua::space::{Pos, Dir, Space};
use ua::io;
use ua::world::{State};

fn calc_occupations(map: &Map, who: u8) -> HashSet<Pos> {
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

struct Onion<'a> {
    body: HashSet<Pos>,
    edge: HashMap<Pos, Vec<Pos>>,
    generation: i32,
    space: &'a Space,
}

use std::collections::hash_map::Entry;

impl<'a> Onion<'a> {
    pub fn from_set(space: &'a Space, seed: &HashSet<Pos>) -> Self {
        Onion {
            body: seed.clone(),
            edge: seed.iter()
                .map(|p| (p.clone(), Vec::new()))
                .collect::<HashMap<_, _>>(),
            generation: 0,
            space: space,
        }
    }

    pub fn expand(&self) -> Onion {
        let mut next_edge: HashMap<Pos, Vec<Pos>> = HashMap::new();
        for (p, _) in &self.edge {
            for n in p.neighbors() {
                let n = self.space.normalize(&n);
                if !self.body.contains(&n) {
                    match next_edge.entry(n) {
                        Entry::Occupied(value) => {
                            value.into_mut().push(p.clone());
                        }
                        Entry::Vacant(value) => {
                            value.insert(vec![p.clone()]);
                        }
                    }
                }
            }
        }
        let mut next_body = self.body.clone();
        next_body.extend(next_edge.iter().map(|(p, _)| p.clone()));
        Onion {
            body: next_body,
            edge: next_edge,
            generation: self.generation + 1,
            space: self.space,
        }
    }
}

fn tick_site(pos: &Pos, src: &Site, map: &Map, me: u8) -> Option<Dir> {
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

fn tick(map: &Map, me: u8) -> HashMap<Pos, Dir> {
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

use std::fmt::Debug;
use std::io::Write;
use std::fs::File;

trait LoggedUnwrap<T> {
    fn unwrap_or_log(self, log: &mut Write) -> T;
}

impl<T, E> LoggedUnwrap<T> for Result<T, E>
    where E: Debug
{
    fn unwrap_or_log(self, log: &mut Write) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                if write!(log, "unwrapping due to: {:?}\n", e).is_ok() {
                    log.flush().unwrap();
                }
                panic!();
            }
        }
    }
}

fn main() {
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
            .map(|(pos, dir)| (pos.to_world(), dir.to_world()))
            .collect::<Vec<_>>();
        connection.send_moves(moves.iter())
            .unwrap_or_log(&mut log_file);
    }
}
