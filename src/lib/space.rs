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

    pub fn width(&self) -> i16
    {
        self.w as i16
    }

    pub fn height(&self) -> i16
    {
        self.h as i16
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

// =============================================================================
// Uniform-cost scan
// -----------------------------------------------------------------------------

use std::collections::BinaryHeap;

#[allow(dead_code)]
pub struct DijsktraScan<'a, F>
{
    space: &'a Space,
    cost_fn: F,
    visited: Vec<bool>,
    queue: BinaryHeap<(i16, Frame<'a>)>,
}

impl<'a, F> DijsktraScan<'a, F>
{
    pub fn new(cost_fn: F, frame: &Frame<'a>) -> DijsktraScan<'a, F>
        where F: Fn(&Frame) -> Option<i16>
    {
        let mut queue = BinaryHeap::with_capacity(frame.space.len() * 2);
        if let Some(init_cost) = cost_fn(frame) {
            queue.push((-init_cost, frame.clone()));
        }
        DijsktraScan {
            space: frame.space,
            cost_fn: cost_fn,
            visited: vec![false; frame.space.len()],
            // The upper bound of the priority queue is 2 * width * height;
            // think of the general case to add three new cell to the search
            // queue you have to consume one cell from the existing queue.
            // This means that there are at most 2 untried branches per
            // visited cell in the search queue.
            queue: queue,
        }
    }
}

impl<'a> Frame<'a>
{
    pub fn dijkstra_scan<F>(&self, cost_fn: F) -> DijsktraScan<F>
        where F: Fn(&Frame) -> Option<i16>
    {
        DijsktraScan::new(cost_fn, &self)
    }
}

impl<'a, F> Iterator for DijsktraScan<'a, F>
    where F: Fn(&Frame) -> Option<i16>
{
    type Item = (i16, Frame<'a>);

    fn next(&mut self) -> Option<Self::Item>
    {
        // The queue is implemented as a maximum heap. We push the negative of
        // the cost to get the minimum cost ordering.
        while let Some((neg_cost, frame)) = self.queue.pop() {
            let unvisited = {
                let mut visited = frame.on_mut(&mut self.visited);
                if *visited {
                    false
                } else {
                    *visited = true;
                    true
                }
            };
            if unvisited {
                for dir in Dir::dirs() {
                    let adj_frame = frame.adjacent_in(dir);
                    if !*adj_frame.on(&self.visited) {
                        if let Some(step_cost) = (self.cost_fn)(&adj_frame) {
                            self.queue.push((neg_cost - step_cost, adj_frame))
                        }
                    }
                }
                return Some((-neg_cost, frame));
            }
        }
        None
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

impl<'a> Map<bool> for Mask<'a>
{
    #[inline]
    fn at(&self, z: usize) -> &bool
    {
        &self.mask[z]
    }

    #[inline]
    fn at_mut(&mut self, z: usize) -> &mut bool
    {
        &mut self.mask[z]
    }
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
    zs: Vec<u16>,
    // Table of indices in `fronts` where the next front starts
    stops: Vec<u16>,
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
            zs: vec![0; n],
            stops: Vec::with_capacity(d as usize),
        }
    }

    pub fn from(source: &'a Mask) -> Self
    {
        let mut wave = Wave::new(source.space);
        wave.ripple(&source);
        wave
    }

    pub fn ripple(&mut self, seed: &Mask)
    {
        debug_assert_eq!(self.space, seed.space);
        let n = self.space.len() as u16;
        let w = self.space.width() as u16;
        let h = self.space.height() as u16;
        // Initialize with seed
        let mut s = 0;
        self.stops.clear();
        for z in 0..self.wave.len() {
            self.wave[z] = if seed.mask[z] {
                self.zs[s] = z as u16;
                s += 1;
                1
            } else {
                0
            }
        }
        self.stops.push(s as u16);
        // Expand the wave front
        let mut a = 0;
        for t in 2.. {
            let s0 = s;
            for i in a..s0 {
                let z = self.zs[i];
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
                    self.zs[s] = z_w as u16;
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
                    self.zs[s] = z_e as u16;
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
                    self.zs[s] = z_n as u16;
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
                    self.zs[s] = z_s as u16;
                    s += 1;
                }
            }
            // Did we expand the wave?
            if s > s0 {
                self.stops.push(s as u16);
                a = s0;
            } else {
                break;
            }
        }
    }

    pub fn front(&'a self, ix: usize) -> Option<Front<'a>>
    {
        if ix < self.stops.len() {
            let start = if ix == 0 {
                0
            } else {
                self.stops[ix - 1]
            };
            let stop = self.stops[ix];
            Some(Front {
                wave: self,
                zs: &self.zs,
                start: start as usize,
                stop: stop as usize,
            })
        } else {
            None
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Expansion<'a>
{
    wave: &'a Wave<'a>,
}

pub struct Front<'a>
{
    wave: &'a Wave<'a>,
    zs: &'a [u16],
    start: usize,
    stop: usize,
}

impl<'a> Iterator for Front<'a>
{
    type Item = Frame<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.start < self.stop {
            let z = Frame {
                space: self.wave.space,
                origin: self.zs[self.start],
            };
            self.start += 1;
            Some(z)
        } else {
            None
        }
    }
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
        debug_assert!(a < m && b < m);
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
        debug_assert!(a < m && b < m);
        if a >= b {
            a - b
        } else {
            a + m - b
        }
    }

    #[inline]
    pub fn dist(a: u16, b: u16, m: u16) -> u16
    {
        debug_assert!(a < m && b < m);
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
