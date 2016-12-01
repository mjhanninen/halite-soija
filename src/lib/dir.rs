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

use std::slice;

#[derive(Clone, Copy, Debug)]
pub enum Dir
{
    North,
    East,
    South,
    West,
}

pub static DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

pub type Dirs = slice::Iter<'static, Dir>;

impl Dir
{
    #[inline]
    pub fn dirs() -> Dirs
    {
        DIRS.iter()
    }
}
