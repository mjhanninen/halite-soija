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
use math::modular;

pub mod dijkstra;
pub mod mask;
pub mod wave;

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

    pub fn frames<'a>(&'a self) -> Frames<'a>
    {
        Frames {
            space: &self,
            start: 0,
            stop: self.sz,
        }
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
}

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
        let origin = match *dir {
            Dir::North => {
                if self.origin < self.space.w {
                    self.origin + self.space.sz - self.space.w
                } else {
                    self.origin - self.space.w
                }
            }
            Dir::East => {
                let origin = self.origin + 1;
                if origin == self.space.sz {
                    0
                } else {
                    origin
                }
            }
            Dir::South => {
                let origin = self.origin + self.space.w;
                if origin >= self.space.sz {
                    origin - self.space.sz
                } else {
                    origin
                }
            }
            Dir::West => {
                if self.origin == 0 {
                    self.space.sz - 1
                } else {
                    self.origin - 1
                }
            }
        };
        debug_assert!(origin < self.space.sz);
        Frame { origin: origin, ..*self }
    }

    pub fn l0_neighbors(&self, radius: usize) -> L0_Neighbors<'a>
    {
        let y = self.origin / self.space.w;
        let x = self.origin - y * self.space.w;
        let r = radius as u16;
        if r > 0 {
            L0_Neighbors {
                space: self.space,
                sz: 2 * r - 1,
                x0: modular::sub(x, r - 1, self.space.w),
                y0: modular::sub(y, r - 1, self.space.h),
                dx: 0,
                dy: 0,
            }
        } else {
            L0_Neighbors {
                space: self.space,
                sz: 0,
                x0: modular::sub(x, r - 1, self.space.w),
                y0: modular::sub(y, r - 1, self.space.h),
                dx: 0,
                dy: 0,
            }
        }
    }

    #[inline]
    pub fn l1_norm(&self, other: &Frame) -> i16
    {
        debug_assert!(self.space == other.space);
        let w = self.space.w;
        let h = self.space.h;
        let y1 = self.origin / w;
        let x1 = self.origin - y1 * w;
        let y2 = other.origin / w;
        let x2 = other.origin - y2 * w;
        let dx = modular::dist(x1, x2, w);
        let dy = modular::dist(y1, y2, h);
        (dx + dy) as i16
    }
}

impl<'a> PartialOrd for Frame<'a>
{
    #[inline]
    fn partial_cmp(&self, other: &Frame<'a>) -> Option<Ordering>
    {
        self.origin.partial_cmp(&other.origin)
    }
}

impl<'a> Ord for Frame<'a>
{
    #[inline]
    fn cmp(&self, other: &Frame<'a>) -> Ordering
    {
        self.origin.cmp(&other.origin)
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

#[allow(non_camel_case_types)]
pub struct L0_Neighbors<'a>
{
    space: &'a Space,
    sz: u16,
    x0: u16,
    y0: u16,
    dx: u16,
    dy: u16,
}

impl<'a> Iterator for L0_Neighbors<'a>
{
    type Item = Frame<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.sz == 0 {
            None
        } else if self.dy == self.sz {
            None
        } else {
            let x = modular::add(self.x0, self.dx, self.space.w);
            let y = modular::add(self.y0, self.dy, self.space.h);
            self.dx += 1;
            if self.dx == self.sz {
                self.dx = 0;
                self.dy += 1;
            }
            Some(Frame {
                space: self.space,
                origin: x + self.space.w * y,
            })
        }

    }
}
