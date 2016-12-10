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
    still: T,
    north: T,
    east: T,
    south: T,
    west: T,
}

impl<T> Choice<T>
{
    pub fn with_init(init: T) -> Self
        where T: Clone
    {
        Choice {
            still: init.clone(),
            north: init.clone(),
            east: init.clone(),
            south: init.clone(),
            west: init,
        }
    }

    pub fn reset(&mut self, value: T)
        where T: Clone
    {
        self.still = value.clone();
        self.north = value.clone();
        self.east = value.clone();
        self.south = value.clone();
        self.west = value;
    }

    pub fn get(&self, key: &Option<Dir>) -> &T
    {
        match *key {
            None => &self.still,
            Some(Dir::North) => &self.north,
            Some(Dir::East) => &self.east,
            Some(Dir::South) => &self.south,
            Some(Dir::West) => &self.west,
        }
    }

    pub fn get_mut(&mut self, key: &Option<Dir>) -> &mut T
    {
        match *key {
            None => &mut self.still,
            Some(Dir::North) => &mut self.north,
            Some(Dir::East) => &mut self.east,
            Some(Dir::South) => &mut self.south,
            Some(Dir::West) => &mut self.west,
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
        if &self.north > max {
            action = Some(Dir::North);
            max = &self.north;
        }
        if &self.east > max {
            action = Some(Dir::East);
            max = &self.east;
        }
        if &self.south > max {
            action = Some(Dir::South);
            max = &self.south;
        }
        if &self.west > max {
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
