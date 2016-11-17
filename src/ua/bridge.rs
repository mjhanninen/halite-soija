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

use ua::map::{Map};
use ua::space::{Space, Pos, Dir};
use ua::world;

impl Dir {
    #[inline]
    pub fn to_world(&self) -> world::Dir {
        match *self {
            Dir::North => world::Dir::North,
            Dir::East => world::Dir::East,
            Dir::South => world::Dir::South,
            Dir::West => world::Dir::West,
        }
    }
}

impl Pos {
    #[inline]
    pub fn to_world(&self) -> world::Pos {
        (self.x, self.y)
    }
}

impl Map {
    pub fn from_world(environment: &world::Environment,
                      state: &world::State) -> Map {
        let w = environment.space.width;
        let h = environment.space.height;
        let space = Space::with_dims(w as i16, h as i16);
        let mut map = Map::from_space(space);
        for i in 0..map.sites.len() {
            let site = &mut map.sites[i];
            site.owner = state.occupation_map[i].tag;
            site.strength = state.occupation_map[i].strength as u8;
            site.production = environment.production_map[i] as u8;
        }
        map
    }
}
