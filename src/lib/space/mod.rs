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

pub mod dijkstra;
pub mod frame;
pub mod mask;
pub mod point;
pub mod wave;

use coord::Coord;
use dir::Dir;
use math::modular;

pub use self::dijkstra::DijsktraScan;
pub use self::wave::Wave;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Space
{
    sz: u16,
    w: u16,
    h: u16,
}

impl Space
{
    pub fn with_dims(width: i16, height: i16) -> Self
    {
        debug_assert!(width > 0 && height > 0);
        Space {
            sz: width as u16 * height as u16,
            w: width as u16,
            h: height as u16,
        }
    }

    // Weird to call it "len" but, hey, that's conisitent with Rust's
    // containers.
    #[inline]
    pub fn len(&self) -> usize
    {
        self.w as usize * self.h as usize
    }

    #[inline]
    pub fn width(&self) -> i16
    {
        self.w as i16
    }

    #[inline]
    pub fn height(&self) -> i16
    {
        self.h as i16
    }

    #[inline]
    pub fn coord_of(&self, ix: usize) -> Coord
    {
        let y = ix / self.w as usize;
        let x = ix - y * self.w as usize;
        Coord {
            x: x as i16,
            y: y as i16,
        }
    }

    #[inline]
    pub fn adjacent_ix(&self, ix: usize, dir: &Dir) -> usize
    {
        debug_assert!(ix < self.sz as usize);
        match *dir {
            Dir::North => {
                if ix < self.w as usize {
                    ix + self.sz as usize - self.w as usize
                } else {
                    ix - self.w as usize
                }
            }
            Dir::East => {
                let adj = ix + 1;
                if adj < self.sz as usize {
                    adj
                } else {
                    0
                }
            }
            Dir::South => {
                let adj = ix + self.w as usize;
                if adj < self.sz as usize {
                    adj
                } else {
                    adj - self.sz as usize
                }
            }
            Dir::West => {
                if ix == 0 {
                    self.sz as usize - 1
                } else {
                    ix - 1
                }
            }
        }
    }

    #[inline]
    pub fn l1_norm(&self, a: usize, b: usize) -> i16
    {
        debug_assert!((a as u16) < self.sz);
        debug_assert!((b as u16) < self.sz);
        let y_a = a as u16 / self.w;
        let x_a = a as u16 - y_a * self.w;
        let y_b = b as u16 / self.w;
        let x_b = b as u16 - y_b * self.w;
        let dx = modular::dist(x_a, x_b, self.w);
        let dy = modular::dist(y_a, y_b, self.h);
        (dx + dy) as i16
    }
}
