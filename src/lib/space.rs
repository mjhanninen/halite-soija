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
use dir::Dir;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Space
{
    sz: u16,
    w: u16,
    h: u16,
    // These are going out of the door; don't increase their use.
    pub width: i16,
    pub height: i16,
}

#[derive(Clone, Debug)]
pub struct Frame<'a>
{
    space: &'a Space,
    origin: u16,
}

#[inline]
fn modular_dist(a: u16, b: u16, m: u16) -> u16
{
    assert!(a < m && b < m);
    let d1 = if a < b {
        b - a
    } else {
        a - b
    };
    let d2 = m - d1;
    if d1 < d2 {
        d1
    } else {
        d2
    }
}

impl<'a> Frame<'a>
{
    pub fn frames(&self) -> Frames
    {
        self.space.frames()
    }

    #[inline]
    pub fn ix(&self) -> usize
    {
        self.origin as usize
    }

    #[inline]
    pub fn coord(&self) -> Coord
    {
        let y = self.origin / self.space.w;
        let x = self.origin - y * self.space.w;
        Coord {
            x: x as i16,
            y: y as i16,
        }
    }

    #[inline]
    pub fn adjacent_in(&self, dir: Dir) -> Frame<'a>
    {
        let origin = match dir {
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
        assert!(origin < self.space.sz);
        Frame { origin: origin, ..*self }
    }

    #[inline]
    pub fn l1_norm(&self, other: &Frame) -> i16
    {
        assert!(self.space == other.space);
        let w = self.space.w;
        let h = self.space.h;
        let y1 = self.origin / w;
        let x1 = self.origin - y1 * w;
        let y2 = other.origin / w;
        let x2 = other.origin - y2 * w;
        let dx = modular_dist(x1, x2, w);
        let dy = modular_dist(y1, y2, h);
        (dx + dy) as i16
    }
}

impl Space
{
    pub fn with_dims(width: i16, height: i16) -> Self
    {
        assert!(width > 0 && height > 0);
        Space {
            sz: width as u16 * height as u16,
            w: width as u16,
            h: height as u16,
            width: width,
            height: height,
        }
    }

    // Weird to call it "len" but, hey, that's conisitent with Rust's
    // containers.
    #[inline]
    pub fn len(&self) -> usize
    {
        self.width as usize * self.height as usize
    }

    pub fn frames<'a>(&'a self) -> Frames<'a>
    {
        Frames {
            space: &self,
            start: 0,
            stop: self.sz,
        }
    }
}

#[cfg(test)]
mod test { }
