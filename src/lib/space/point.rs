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

use std::cmp::Ordering;

use dir::Dir;
use space::Space;
use space::frame::Frame;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point<'a>
{
    space: &'a Space,
    ix: u16,
}

impl<'a> Frame for Point<'a>
{
    #[inline]
    fn space(&self) -> &Space
    {
        self.space
    }

    #[inline]
    fn ix(&self) -> usize
    {
        self.ix as usize
    }
}

impl<'a> Point<'a>
{
    // XXX: This is a hack.  Many of the objects here maintain a reference to
    // the containing space either directly or indirectly.  When the reference
    // is indirect the life-time is often constrained by the life-time of the
    // intermediating object which often is also very transient.  This hack
    // enables me to code around these life-time problems but there is
    // something really awkward about this.  I need to think about this
    // critically.
    #[inline]
    pub fn hack<'b>(&self, space: &'b Space) -> Point<'b>
    {
        Point {
            space: space,
            ix: self.ix,
        }
    }

    #[inline]
    pub fn new(space: &'a Space, ix: usize) -> Self
    {
        Point {
            space: space,
            ix: ix as u16,
        }
    }

    #[inline]
    pub fn adjacent_in(&self, dir: &Dir) -> Point<'a>
    {
        Point { ix: self.space.adjacent_ix(self.ix(), dir) as u16, ..*self }
    }
}

impl<'a> PartialOrd for Point<'a>
{
    #[inline]
    fn partial_cmp(&self, other: &Point<'a>) -> Option<Ordering>
    {
        self.ix().partial_cmp(&other.ix())
    }
}

impl<'a> Ord for Point<'a>
{
    #[inline]
    fn cmp(&self, other: &Point<'a>) -> Ordering
    {
        self.ix().cmp(&other.ix())
    }
}

#[derive(Clone, Debug)]
pub struct Points<'a>
{
    space: &'a Space,
    start: u16,
    stop: u16,
}

impl<'a> Iterator for Points<'a>
{
    type Item = Point<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.start != self.stop {
            let f = Point {
                space: self.space,
                ix: self.start,
            };
            self.start += 1;
            Some(f)
        } else {
            None
        }
    }
}

impl Space
{
    #[inline]
    pub fn point<'a>(&'a self, ix: usize) -> Point<'a>
    {
        debug_assert!((self.sz as usize) < ix);
        Point {
            space: self,
            ix: ix as u16,
        }
    }

    #[inline]
    pub fn points<'a>(&'a self) -> Points<'a>
    {
        Points {
            space: &self,
            start: 0,
            stop: self.sz,
        }
    }
}
