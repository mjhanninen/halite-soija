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

// XXX: Remove when you come back fixing this
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use coord::Coord;
use space::Space;

pub struct Onion<'a>
{
    body: HashSet<Coord>,
    edge: HashMap<Coord, Vec<Coord>>,
    generation: i32,
    space: &'a Space,
}


impl<'a> Onion<'a>
{
    pub fn from_set(space: &'a Space, seed: &HashSet<Coord>) -> Self
    {
        Onion {
            body: seed.clone(),
            edge: seed.iter()
                      .map(|p| (p.clone(), Vec::new()))
                      .collect::<HashMap<_, _>>(),
            generation: 0,
            space: space,
        }
    }

    pub fn expand(&self) -> Onion
    {
        let mut next_edge: HashMap<Coord, Vec<Coord>> = HashMap::new();
        for (p, _) in &self.edge {
            // XXX: Broken after refactoring
            // for n in p.neighbors() {
            //     let n = self.space.normalize(&n);
            //     if !self.body.contains(&n) {
            //         match next_edge.entry(n) {
            //             Entry::Occupied(value) => {
            //                 value.into_mut().push(p.clone());
            //             }
            //             Entry::Vacant(value) => {
            //                 value.insert(vec![p.clone()]);
            //             }
            //         }
            //     }
            // }
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
