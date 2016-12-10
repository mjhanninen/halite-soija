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

use coord::Coord;
use dir::Dir;
use map::Map;
use space::Space;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame<'a>
{
    space: &'a Space,
    origin: u16,
}

impl<'a> Frame<'a>
{
    // XXX: This is a hack.  Many of the objects here maintain a reference to
    // the containing space either directly or indirectly.  When the reference
    // is indirect the life-time is often constrained by the life-time of the
    // intermediating object which often is also very transient.  This hack
    // enables me to code around these life-time problems but there is
    // something really awkward about this.  I need to think about this
    // critically.
    #[inline]
    pub fn hack<'b>(&self, space: &'b Space) -> Frame<'b>
    {
        Frame {
            space: space,
            origin: self.origin,
        }
    }

    #[inline]
    pub fn new(space: &'a Space, ix: usize) -> Self
    {
        Frame {
            space: space,
            origin: ix as u16,
        }
    }

    #[inline]
    pub fn space(&self) -> &Space
    {
        self.space
    }

    #[inline]
    pub fn ix(&self) -> usize
    {
        self.origin as usize
    }

    #[inline]
    pub fn on<'b, T, M>(&self, map: &'b M) -> &'b T
        where M: Map<T>
    {
        map.at(self.ix())
    }

    #[inline]
    pub fn on_mut<'b, T, M>(&self, map: &'b mut M) -> &'b mut T
        where M: Map<T>
    {
        map.at_mut(self.ix())
    }

    #[inline]
    pub fn coord(&self) -> Coord
    {
        self.space.coord_of(self.ix())
    }

    #[inline]
    pub fn adjacent_in(&self, dir: &Dir) -> Frame<'a>
    {
        Frame { origin: self.space.adjacent_ix(self.ix(), dir) as u16, ..*self }
    }

    #[inline]
    pub fn l1_norm(&self, other: &Frame) -> i16
    {
        debug_assert!(self.space == other.space);
        self.space.l1_norm(self.ix(), other.ix())
    }
}

impl<'a> PartialOrd for Frame<'a>
{
    #[inline]
    fn partial_cmp(&self, other: &Frame<'a>) -> Option<Ordering>
    {
        self.ix().partial_cmp(&other.ix())
    }
}

impl<'a> Ord for Frame<'a>
{
    #[inline]
    fn cmp(&self, other: &Frame<'a>) -> Ordering
    {
        self.ix().cmp(&other.ix())
    }
}

#[derive(Clone, Debug)]
pub struct Frames<'a>
{
    space: &'a Space,
    start: u16,
    stop: u16,
}

impl<'a> Iterator for Frames<'a>
{
    type Item = Frame<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.start != self.stop {
            let f = Frame {
                space: self.space,
                origin: self.start,
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
    pub fn frames<'a>(&'a self) -> Frames<'a>
    {
        Frames {
            space: &self,
            start: 0,
            stop: self.sz,
        }
    }
}
