// Copyright (C) 2016 Matti Hänninen
//
// This file is part of Umpteenth Anion.
//
// Umpteenth Anion is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Foobar is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with Foobar.  If not, see <http://www.gnu.org/licenses/>.

use ua::space::{Pos, Space};

#[derive(Clone, Debug)]
pub struct Site {
    pub owner: u8,
    pub strength: u8,
    pub production: u8,
}

impl Site {
    pub fn blank() -> Self {
        Site {
            owner: 0,
            strength: 0,
            production: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub space: Space,
    pub sites: Vec<Site>,
}

impl Map {
    pub fn from_space(space: Space) -> Self {
        let sites = vec![Site::blank(); space.len()];
        Map {
            space: space,
            sites: sites,
        }
    }

    pub fn site(&self, pos: &Pos) -> &Site {
        &self.sites[self.space.ix(pos)]
    }
}
