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

use space::frame::Frame;

pub trait Map<'a, T>
{
    fn at(&'a self, ix: usize) -> T;
}

pub trait RefMap<T>
{
    fn ref_at(&self, ix: usize) -> &T;
}

pub trait MutMap<T>
{
    fn mut_at(&mut self, ix: usize) -> &mut T;
}

impl<'a, T> Map<'a, T> for Vec<T>
    where T: Clone
{
    #[inline]
    fn at(&'a self, ix: usize) -> T
    {
        self[ix].clone()
    }
}

impl<T> RefMap<T> for Vec<T>
{
    #[inline]
    fn ref_at(&self, ix: usize) -> &T
    {
        &self[ix]
    }
}

impl<T> MutMap<T> for Vec<T>
{
    #[inline]
    fn mut_at(&mut self, ix: usize) -> &mut T
    {
        &mut self[ix]
    }
}
