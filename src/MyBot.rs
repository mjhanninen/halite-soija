#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate rand;

#[macro_use]
extern crate text_io;

mod hlt;
mod ua;

use std::collections::{HashSet,HashMap};

use hlt::networking;
use hlt::types::Location;
use ua::map::{Map, Site};
use ua::space::{Pos, Dir, Space};

fn calc_occupations(map: &Map, who: u8) -> HashSet<Pos> {
    map.space.sweep()
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
            edge: seed.iter().map(|p| (p.clone(), Vec::new())).collect::<HashMap<_, _>>(),
            generation: 0,
            space: space
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
                        },
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

fn tick(map: &Map, me: u8) -> HashMap<Location, u8> {
    let occupations = calc_occupations(map, me);
    let gen0 = Onion::from_set(&map.space, &occupations);
    let gen1 = gen0.expand();
    let mut moves = HashMap::new();
    for p in map.space.sweep() {
        let source = map.site(&p);
        if source.owner == me {
            let loc = p.to_hlt_location();
            if let Some(dir) = tick_site(&p, source, map, me) {
                moves.insert(loc, dir.to_code());
            }
        }
    }
    moves
}

fn main() {
    let (me, mut game_map) = networking::get_init();
    networking::send_init(format!("UmpteenthAnion_{}", me.to_string()));
    loop {
        networking::get_frame(&mut game_map);
        let map = Map::from_hlt_game_map(&game_map);
        let moves = tick(&map, me);
        networking::send_frame(moves);
    }
}
