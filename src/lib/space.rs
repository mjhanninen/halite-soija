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
use map::Map;

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
        assert!(width > 0 && height > 0);
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

    pub fn width(&self) -> i16
    {
        self.w as i16
    }

    pub fn height(&self) -> i16
    {
        self.h as i16
    }
}

#[derive(Clone, Debug)]
pub struct Frame<'a>
{
    space: &'a Space,
    origin: u16,
}

impl<'a> Frame<'a>
{
    #[inline]
    pub fn ix(&self) -> usize
    {
        self.origin as usize
    }

    #[inline]
    pub fn on<'b, T>(&self, map: &'b Map<T>) -> &'b T
    {
        &map[self.ix()]
    }

    #[inline]
    pub fn on_mut<'b, T>(&self, map: &'b mut Map<T>) -> &'b mut T
    {
        &mut map[self.ix()]
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
        assert!(origin < self.space.sz);
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
        assert!(self.space == other.space);
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

pub struct Mask<'a>
{
    space: &'a Space,
    mask: Vec<bool>,
}

impl<'a> Mask<'a>
{
    pub fn new(space: &'a Space) -> Self
    {
        Mask {
            space: space,
            mask: vec![false; space.len()],
        }
    }

    pub fn create<F>(space: &'a Space, f: F) -> Self
        where F: Fn(&Frame) -> bool
    {
        let mut mask = Vec::with_capacity(space.len());
        let mut p = Frame {
            space: space,
            origin: 0,
        };
        for i in 0..space.len() {
            p.origin = i as u16;
            mask.push(f(&p));
        }
        Mask {
            space: space,
            mask: mask,
        }
    }
}

pub struct Wave<'a>
{
    space: &'a Space,
    // Image of wave front indices
    wave: Vec<u8>,
    // Table of call indices belonging to fronts
    fronts: Vec<u16>,
    // Table of indices in `fronts` where the next front starts
    starts: Vec<u16>,
}

impl<'a> Wave<'a>
{
    pub fn new(space: &'a Space) -> Self
    {
        // A wave cannot reach further than this. Actually we should cut this
        // distance by half.
        let d = space.width() + space.height();
        let n = space.len();
        Wave {
            space: space,
            wave: vec![0; n],
            fronts: vec![0; n],
            starts: Vec::with_capacity(d as usize),
        }
    }

    pub fn ripple(&mut self, seed: &Mask)
    {
        assert_eq!(self.space, seed.space);
        let n = self.space.len() as u16;
        let w = self.space.width() as u16;
        let h = self.space.height() as u16;
        // Initialize with seed
        let mut s = 0;
        self.starts.clear();
        for z in 0..self.wave.len() {
            self.wave[z] = if seed.mask[z] {
                self.fronts[s] = z as u16;
                s += 1;
                1
            } else {
                0
            }
        }
        self.starts.push(s as u16);
        // Expand the wave front
        let mut a = 0;
        for t in 2.. {
            let s0 = s;
            for i in a..s0 {
                let z = self.fronts[i];
                let y = z / w;
                let x = z - w * y;
                // Expand westwards
                let z_w = if x > 0 {
                    z - 1
                } else {
                    z + w - 1
                } as usize;
                if self.wave[z_w] == 0 {
                    self.wave[z_w] = t;
                    self.fronts[s] = z_w as u16;
                    s += 1;
                }
                // Expand eastwards
                let z_e = if x + 1 < w {
                    z + 1
                } else {
                    z - x
                } as usize;
                if self.wave[z_e] == 0 {
                    self.wave[z_e] = t;
                    self.fronts[s] = z_e as u16;
                    s += 1;
                }
                // Expand northwards
                let z_n = if y > 0 {
                    z - w
                } else {
                    z + n - w
                } as usize;
                if self.wave[z_n] == 0 {
                    self.wave[z_n] = t;
                    self.fronts[s] = z_n as u16;
                    s += 1;
                }
                // Expand southwards
                let z_s = if y + 1 < h {
                    z + w
                } else {
                    x
                } as usize;
                if self.wave[z_s] == 0 {
                    self.wave[z_s] = t;
                    self.fronts[s] = z_s as u16;
                    s += 1;
                }
            }
            // Did we expand the wave?
            if s > s0 {
                self.starts.push(s as u16);
                a = s0;
            } else {
                break;
            }
        }
    }

    pub fn contraction(&'a self) -> Contraction<'a>
    {
        Contraction { wave: &self }
    }

    pub fn expansion(&'a self) -> Expansion<'a>
    {
        Expansion { wave: &self }
    }
}

pub struct Contraction<'a>
{
    wave: &'a Wave<'a>,
}

impl<'a> Iterator for Contraction<'a>
{
    type Item = Frame<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        None
    }
}

pub struct Expansion<'a>
{
    wave: &'a Wave<'a>,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_wave_1() {
        let space = Space::with_dims(6, 5);
        let map = vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 0, 1, 0, 0,
            0, 1, 1, 1, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let seed = Mask::create(&space, |f: &Frame| {
            *f.on(&map) == 1
        });
        let mut wave = Wave::new(&space);
        wave.ripple(&seed);
        let expected = vec![
            4, 3, 2, 3, 4, 5,
            3, 2, 1, 2, 3, 4,
            3, 2, 2, 1, 2, 3,
            2, 1, 1, 1, 2, 3,
            3, 2, 2, 2, 3, 4,
        ];
        assert_eq!(wave.wave, expected);
    }

    // The same as test_wave_1 except panned.
    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_wave_2() {
        let space = Space::with_dims(6, 5);
        let map = vec![
            1, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0,
        ];
        let seed = Mask::create(&space, |f: &Frame| {
            *f.on(&map) == 1
        });
        let mut wave = Wave::new(&space);
        wave.ripple(&seed);
        let expected = vec![
            1, 2, 3, 2, 1, 1,
            2, 3, 4, 3, 2, 2,
            3, 4, 5, 4, 3, 2,
            2, 3, 4, 3, 2, 1,
            1, 2, 3, 3, 2, 2,
        ];
        assert_eq!(wave.wave, expected);
    }
}

mod modular {

    #[inline]
    pub fn add(a: u16, b: u16, m: u16) -> u16
    {
        assert!(a < m && b < m);
        let c = a + b;
        if c < m {
            c
        } else {
            c - m
        }
    }

    #[inline]
    pub fn sub(a: u16, b: u16, m: u16) -> u16
    {
        assert!(a < m && b < m);
        if a >= b {
            a - b
        } else {
            a + m - b
        }
    }

    #[inline]
    pub fn dist(a: u16, b: u16, m: u16) -> u16
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

    #[cfg(test)]
    mod test {

        #[test]
        fn test_sub()
        {
            for m in 1..100 {
                for a in 0..m {
                    for b in 0..m {
                        let c = super::sub(a, b, m);
                        if c >= m {
                            panic!("failed for a={}, b={}, m={}", a, b, m);
                        }
                        assert_eq!((c + b) % m, a);
                    }
                }
            }
        }
    }
}
