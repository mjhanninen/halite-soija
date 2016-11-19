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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos
{
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug)]
pub enum Dir
{
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
pub struct Neighbors
{
    pos: Pos,
    dir: Option<Dir>,
}

impl Pos
{
    #[inline]
    pub fn neighbor(&self, dir: Dir) -> Pos
    {
        match dir {
            Dir::North => {
                Pos {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            Dir::East => {
                Pos {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            Dir::South => {
                Pos {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            Dir::West => {
                Pos {
                    x: self.x - 1,
                    y: self.y,
                }
            }
        }
    }

    #[inline]
    pub fn neighbors(&self) -> Neighbors
    {
        Neighbors {
            pos: self.clone(),
            dir: Some(Dir::North),
        }
    }
}

impl Iterator for Neighbors
{
    type Item = Pos;
    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.dir {
            Some(Dir::North) => {
                self.dir = Some(Dir::East);
                Some(self.pos.neighbor(Dir::North))
            }
            Some(Dir::East) => {
                self.dir = Some(Dir::South);
                Some(self.pos.neighbor(Dir::East))
            }
            Some(Dir::South) => {
                self.dir = Some(Dir::West);
                Some(self.pos.neighbor(Dir::South))
            }
            Some(Dir::West) => {
                self.dir = None;
                Some(self.pos.neighbor(Dir::West))
            }
            None => None,
        }
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

    pub fn ix(&self) -> usize
    {
        self.origin as usize
    }

    pub fn coord(&self) -> Pos
    {
        let y = self.origin / self.space.w;
        let x = self.origin - y * self.space.w;
        Pos {
            x: x as i16,
            y: y as i16,
        }
    }

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

#[inline]
fn modulo(n: i16, m: i16) -> i16
{
    assert!(m > 0);
    if n < 0 {
        n % m + m
    } else if n < m {
        n
    } else {
        n % m
    }
}

#[inline]
fn shortest_diff(a: i16, b: i16, m: i16) -> i16
{
    let d = modulo(b, m) - modulo(a, m);
    if d < -m / 2 {
        d + m
    } else if d > m / 2 {
        d - m
    } else {
        d
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

    #[inline]
    pub fn normalize(&self, p: &Pos) -> Pos
    {
        Pos {
            x: modulo(p.x, self.width),
            y: modulo(p.y, self.height),
        }
    }

    #[inline]
    pub fn ix(&self, p: &Pos) -> usize
    {
        let x = modulo(p.x, self.width);
        let y = modulo(p.y, self.height);
        x as usize + (y as usize) * (self.width as usize)
    }

    // Weird to call it "len" but, hey, that's conisitent with Rust's
    // containers.
    #[inline]
    pub fn len(&self) -> usize
    {
        self.width as usize * self.height as usize
    }

    #[inline]
    pub fn sweep(&self) -> Sweep
    {
        Sweep {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    pub fn direction(&self, source: &Pos, target: &Pos) -> Dir
    {
        let dx = shortest_diff(source.x, target.x, self.width);
        let dy = shortest_diff(source.y, target.y, self.height);
        if dx.abs() > dy.abs() {
            if dx < 0 {
                Dir::West
            } else {
                Dir::East
            }
        } else {
            if dy < 0 {
                Dir::North
            } else {
                Dir::South
            }
        }
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

pub struct Sweep
{
    width: i16,
    height: i16,
    x: i16,
    y: i16,
}

impl Iterator for Sweep
{
    type Item = Pos;
    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.y < self.height {
            let p = Pos {
                x: self.x,
                y: self.y,
            };
            self.x += 1;
            if self.x == self.width {
                self.y += 1;
                self.x = 0;
            }
            assert!(self.x < self.width);
            Some(p)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_space_sweep()
    {
        use super::Space;
        let space = Space::with_dims(3, 5);
        let mut n = 0;
        for _ in space.sweep() {
            n += 1;
        }
        assert_eq!(n, 15);
    }
}
