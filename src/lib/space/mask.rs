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

use map::Map;
use space::{Frame, Space};

pub struct Mask<'a>
{
    // XXXX: Make this private again
    pub space: &'a Space,
    mask: Vec<bool>,
}

impl<'a> Map<bool> for Mask<'a>
{
    #[inline]
    fn at(&self, z: usize) -> &bool
    {
        &self.mask[z]
    }

    #[inline]
    fn at_mut(&mut self, z: usize) -> &mut bool
    {
        &mut self.mask[z]
    }
}

impl<'a> Mask<'a>
{
    pub fn new(space: &'a Space) -> Self
    {
        Mask {
            space: space,
            mask: vec![false; space.len()],
        }
    }

    pub fn singleton(frame: &'a Frame) -> Self
    {
        let mut mask = vec![false; frame.space.len()];
        mask[frame.origin as usize] = true;
        Mask {
            space: frame.space,
            mask: mask,
        }
    }

    pub fn create<F>(space: &'a Space, f: F) -> Self
        where F: Fn(&Frame) -> bool
    {
        let mut mask = Vec::with_capacity(space.len());
        let mut p = Frame {
            space: space,
            origin: 0,
        };
        for i in 0..space.len() {
            p.origin = i as u16;
            mask.push(f(&p));
        }
        Mask {
            space: space,
            mask: mask,
        }
    }
}
