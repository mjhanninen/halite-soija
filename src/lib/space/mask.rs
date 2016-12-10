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
use space::Space;
use space::frame::Frame;
use space::point::Point;

pub struct Mask<'a>
{
    // XXXX: Make this private again
    pub space: &'a Space,
    mask: Vec<bool>,
}

impl<'a> Map<bool> for Mask<'a>
{
    #[inline]
    fn at(&self, ix: usize) -> &bool
    {
        &self.mask[ix]
    }

    #[inline]
    fn at_mut(&mut self, ix: usize) -> &mut bool
    {
        &mut self.mask[ix]
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

    pub fn singleton(frame: &'a Point) -> Self
    {
        let mut mask = vec![false; frame.space().len()];
        mask[frame.ix()] = true;
        Mask {
            space: frame.space(),
            mask: mask,
        }
    }

    pub fn create<F>(space: &'a Space, f: F) -> Self
        where F: Fn(&Point) -> bool
    {
        let mut mask = Vec::with_capacity(space.len());
        for ix in 0..space.len() {
            mask.push(f(&Point::new(space, ix)));
        }
        Mask {
            space: space,
            mask: mask,
        }
    }
}
