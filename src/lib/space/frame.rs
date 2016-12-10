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

use coord::Coord;
use map::{Map, MutMap, RefMap};
use space::Space;
use space::point::Point;

pub trait Frame
{
    fn space(&self) -> &Space;

    fn ix(&self) -> usize;

    #[inline]
    fn on<'a, M, T>(&self, map: &'a M) -> T
        where M: Map<'a, T>
    {
        map.at(self.ix())
    }

    #[inline]
    fn ref_on<'a, T, M>(&self, map: &'a M) -> &'a T
        where M: RefMap<T>
    {
        map.ref_at(self.ix())
    }

    #[inline]
    fn mut_on<'a, T, M>(&self, map: &'a mut M) -> &'a mut T
        where M: MutMap<T>
    {
        map.mut_at(self.ix())
    }

    #[inline]
    fn coord(&self) -> Coord
    {
        self.space().coord_of(self.ix())
    }

    #[inline]
    fn l1_norm<F: Frame>(&self, other: &F) -> i16
    {
        debug_assert!(self.space() == other.space());
        self.space().l1_norm(self.ix(), other.ix())
    }

    // NB: An implementation of Into<Point<'a>> for &'a Frame turned out to be
    // an ownership hell. Gave up.
    #[inline]
    fn to_point<'a>(&'a self) -> Point<'a>
    {
        self.space().point(self.ix())
    }
}
