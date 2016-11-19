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

#[derive(Clone, Copy, Debug)]
pub enum Dir
{
    North,
    East,
    South,
    West,
}

impl Dir
{
    pub fn dirs() -> Dirs
    {
        Dirs { d: Some(Dir::North) }
    }
}

#[derive(Clone, Debug)]
pub struct Dirs
{
    d: Option<Dir>,
}

impl Iterator for Dirs
{
    type Item = Dir;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        let d = self.d;
        match d {
            Some(Dir::North) => {
                self.d = Some(Dir::East);
            }
            Some(Dir::East) => {
                self.d = Some(Dir::South);
            }
            Some(Dir::South) => {
                self.d = Some(Dir::West);
            }
            Some(Dir::West) => {
                self.d = None;
            }
            None => {}
        }
        d
    }
}
