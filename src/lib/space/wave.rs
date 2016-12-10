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

use map::{Map, RefMap};
use space::Space;
use space::frame::Frame;
use space::mask::Mask;

// Implementation notes:
//
// * This wave needs to have couple different properties; firstly it needs to
//   have reusable state so that we don't have to allocate for each wave
//   separately.  Secondly it has to partial in sense that I don't have to
//   compute the full wave if I need only the couple first wave fronts.
//
// So we could have:
//
// * Wave that contains the whole wave state.
//
// * Wavefronts iterator over the fronts of the wave. The fronts would
//   generated on demand.

pub struct Wave<'a>
{
    space: &'a Space,
    // Image of wave front indices
    // XXX: This could be made a simple bit vector
    wave: Vec<u8>,
    // Table of call indices belonging to fronts
    ixs: Vec<u16>,
    // Table of indices in `fronts` where the next front starts
    // XXX: this could be called the `breaks`
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
            ixs: vec![0; n],
            stops: Vec::with_capacity(d as usize),
        }
    }

    pub fn from(source: &'a Mask) -> Wave<'a>
    {
        let mut wave = Wave::new(source.space);
        wave.ripple(source, None);
        wave
    }

    pub fn between(source: &'a Mask, sink: &'a Mask) -> Wave<'a>
    {
        let mut wave = Wave::new(source.space);
        wave.ripple(&source, Some(sink));
        wave
    }

    fn ripple(&mut self, source: &Mask, sink: Option<&Mask>)
    {
        debug_assert_eq!(self.space, source.space);
        let n = self.space.len() as u16;
        let w = self.space.width() as u16;
        let h = self.space.height() as u16;
        // Initialize with the wave source and sink
        let mut s = 0;
        self.stops.clear();
        if let Some(sink) = sink {
            debug_assert_eq!(self.space, sink.space);
            for ix in 0..self.wave.len() {
                self.wave[ix] = if *source.ref_at(ix) {
                    self.ixs[s] = ix as u16;
                    s += 1;
                    1
                } else if *sink.ref_at(ix) {
                    255
                } else {
                    0
                }
            }
        } else {
            for ix in 0..self.wave.len() {
                self.wave[ix] = if *source.ref_at(ix) {
                    self.ixs[s] = ix as u16;
                    s += 1;
                    1
                } else {
                    0
                }
            }
        }
        self.stops.push(s as u16);
        // Expand the wave front
        let mut a = 0;
        for t in 2.. {
            let s0 = s;
            for i in a..s0 {
                let ix = self.ixs[i];
                let y = ix / w;
                let x = ix - w * y;
                // Expand westwards
                let ix_w = if x > 0 {
                    ix - 1
                } else {
                    ix + w - 1
                } as usize;
                if self.wave[ix_w] == 0 {
                    self.wave[ix_w] = t;
                    self.ixs[s] = ix_w as u16;
                    s += 1;
                }
                // Expand eastwards
                let ix_e = if x + 1 < w {
                    ix + 1
                } else {
                    ix - x
                } as usize;
                if self.wave[ix_e] == 0 {
                    self.wave[ix_e] = t;
                    self.ixs[s] = ix_e as u16;
                    s += 1;
                }
                // Expand northwards
                let ix_n = if y > 0 {
                    ix - w
                } else {
                    ix + n - w
                } as usize;
                if self.wave[ix_n] == 0 {
                    self.wave[ix_n] = t;
                    self.ixs[s] = ix_n as u16;
                    s += 1;
                }
                // Expand southwards
                let ix_s = if y + 1 < h {
                    ix + w
                } else {
                    x
                } as usize;
                if self.wave[ix_s] == 0 {
                    self.wave[ix_s] = t;
                    self.ixs[s] = ix_s as u16;
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
                start: start as usize,
                stop: stop as usize,
            })
        } else {
            None
        }
    }
}

pub struct Flux<'a>
{
    wave: &'a Wave<'a>,
    ix: u16,
}

impl<'a, 'b> Map<'a, Flux<'b>> for Wave<'b>
    where 'a: 'b
{
    #[inline]
    fn at(&'a self, ix: usize) -> Flux<'b>
    {
        Flux {
            wave: &self,
            ix: ix as u16,
        }
    }
}

impl<'a> Frame for Flux<'a>
{
    #[inline]
    fn space(&self) -> &Space
    {
        self.wave.space
    }

    #[inline]
    fn ix(&self) -> usize
    {
        self.ix as usize
    }
}

impl<'a> Flux<'a>
{
    // Returns the fluxes that flow into this one
    fn sources(&self)
    {
        let w = self.space().width();
        let h = self.space().height();
    }

    // Returns the fluxes that this one could flow into
    fn sinks(&self)
    {
        unimplemented!();
    }
}

struct Sources<'a>
{

}

struct Sinks<'a>
{
}

pub struct Front<'a>
{
    wave: &'a Wave<'a>,
    start: usize,
    stop: usize,
}

impl<'a> Iterator for Front<'a>
{
    type Item = Flux<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.start < self.stop {
            let flux = Flux {
                wave: self.wave,
                ix: self.wave.ixs[self.start],
            };
            self.start += 1;
            Some(flux)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use space::Space;
    use space::frame::Frame;
    use space::mask::Mask;
    use space::point::Point;

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
        let seed = Mask::create(&space, |f: &Point| {
            *f.ref_on(&map) == 1
        });
        let mut wave = Wave::new(&space);
        wave.ripple(&seed, None);
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
        let seed = Mask::create(&space, |f: &Point| {
            *f.ref_on(&map) == 1
        });
        let mut wave = Wave::new(&space);
        wave.ripple(&seed, None);
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
