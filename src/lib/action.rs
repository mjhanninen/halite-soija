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

use dir::Dir;
use coord::Coord;

pub type Action = (Coord, Option<Dir>);

pub struct Choice<T>
{
    still: Option<T>,
    north: Option<T>,
    east: Option<T>,
    south: Option<T>,
    west: Option<T>,
}

impl<T> Choice<T>
{
    pub fn new() -> Self
    {
        Choice {
            still: None,
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }

    pub fn get(&self, key: &Option<Dir>) -> Option<&T>
    {
        match *key {
            None => self.still.as_ref(),
            Some(Dir::North) => self.north.as_ref(),
            Some(Dir::East) => self.east.as_ref(),
            Some(Dir::South) => self.south.as_ref(),
            Some(Dir::West) => self.west.as_ref(),
        }
    }

    #[inline]
    fn gt(a: &Option<T>, b: &Option<T>) -> bool
        where T: PartialOrd
    {
        if let Some(ref a) = *a {
            if let Some(ref b) = *b {
                a > b
            } else {
                true
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn find_max_action(&self) -> Option<Dir>
        where T: PartialOrd
    {
        let mut action = None;
        let mut max = &self.still;
        if Choice::gt(&self.north, max) {
            action = Some(Dir::North);
            max = &self.north;
        }
        if Choice::gt(&self.east, max) {
            action = Some(Dir::East);
            max = &self.east;
        }
        if Choice::gt(&self.south, max) {
            action = Some(Dir::South);
            max = &self.south;
        }
        if Choice::gt(&self.west, max) {
            action = Some(Dir::West);
        }
        action
    }
}

#[cfg(test)]
mod test {

    use super::Choice;

    #[test]
    fn test_lt_helper()
    {
        assert!(Choice::gt(&Some(1.0_f32), &None));
        assert!(Choice::gt(&Some(1.0_f32), &Some(0.0)));
        assert!(!Choice::gt(&Some(1.0_f32), &Some(1.0)));
        assert!(!Choice::gt(&None, &Some(1.0_f32)));
        assert!(!Choice::<f32>::gt(&None, &None));
    }
}
