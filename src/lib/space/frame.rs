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
use map::Map;
use space::Space;

pub trait Frame
{
    fn space(&self) -> &Space;

    fn ix(&self) -> usize;

    #[inline]
    fn on<'b, T, M>(&self, map: &'b M) -> &'b T
        where M: Map<T>
    {
        map.at(self.ix())
    }

    #[inline]
    fn on_mut<'b, T, M>(&self, map: &'b mut M) -> &'b mut T
        where M: Map<T>
    {
        map.at_mut(self.ix())
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
}
