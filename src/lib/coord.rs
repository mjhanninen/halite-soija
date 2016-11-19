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

use dir::{Dir, Dirs};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord
{
    pub x: i16,
    pub y: i16,
}

impl Coord
{
    #[inline]
    pub fn neighbor(&self, dir: Dir) -> Coord
    {
        match dir {
            Dir::North => {
                Coord {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            Dir::East => {
                Coord {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            Dir::South => {
                Coord {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            Dir::West => {
                Coord {
                    x: self.x - 1,
                    y: self.y,
                }
            }
        }
    }

    #[inline]
    pub fn neighbors(&self) -> Neighbors
    {
        Neighbors {
            p: self.clone(),
            ds: Dir::dirs(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Neighbors
{
    p: Coord,
    ds: Dirs,
}

impl Iterator for Neighbors
{
    type Item = Coord;
    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        if let Some(dir) = self.ds.next() {
            Some(self.p.neighbor(dir))
        } else {
            None
        }
    }
}
