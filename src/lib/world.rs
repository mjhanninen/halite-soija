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

pub type Map<T> = Vec<T>;
pub type Production = i16;

#[derive(Clone, Debug)]
pub struct Space {
    pub width: i16,
    pub height: i16,
}

impl Space {
    pub fn with_dims(width: i16, height: i16) -> Self {
        Space {
            width: width,
            height: height,
        }
    }
    pub fn size(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }
}

#[derive(Debug)]
pub struct Environment {
    pub my_tag: Tag,
    pub space: Space,
    pub production_map: Map<Production>,
}

impl Environment {
    pub fn create(my_tag: Tag, width: i16, height: i16) -> Self {
        let space = Space::with_dims(width, height);
        let production_map = Vec::with_capacity(space.size());
        Environment {
            my_tag: my_tag,
            space: space,
            production_map: production_map,
        }
    }
}

pub type Tag = u8;
pub type Strength = i16;

#[derive(Clone, Debug)]
pub struct Occupation {
    pub tag: Tag,
    pub strength: Strength,
}

#[derive(Debug)]
pub struct State {
    pub occupation_map: Map<Occupation>,
}

impl State {
    pub fn for_environment(environment: &Environment) -> Self {
        State {
            occupation_map: vec![Occupation {tag: 0, strength: 0};
                 environment.space.size()],
        }
    }
}

pub type Pos = (i16, i16);

#[derive(Debug)]
pub enum Dir {
    Still,
    North,
    East,
    South,
    West,
}

pub type Move = (Pos, Dir);
