#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate rand;

#[macro_use]
extern crate text_io;

mod hlt;
mod ua;

use std::collections::HashMap;

use hlt::networking;
use hlt::types::Location;
use ua::map::{Map, Site};
use ua::space::{Pos, Dir};


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
