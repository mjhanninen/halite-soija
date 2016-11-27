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

use std::borrow::Cow;
use std::num::Wrapping;

use rand::{self, Rng};
use ua::{Action, Coord, Environment, State};

use brain::{Brain, Mold};
use params::Params;

const ALPHA: f64 = 1000.0;
const BETA: f64 = 0.125;

pub struct ProbeMold;

impl Mold for ProbeMold
{
    fn name(&self) -> Cow<str>
    {
        Cow::Borrowed(&"Probe")
    }


    fn reanimate(&self,
                 _params: &Params,
                 _environment: Environment,
                 _init_state: &State)
        -> Box<Brain>
    {
        Box::new(ProbeBrain { iteration: 0 })
    }
}

pub struct ProbeBrain
{
    iteration: i32,
}

impl Brain for ProbeBrain
{
    fn tick(&mut self, _state: &State) -> Vec<Action>
    {
        self.iteration += 1;
        let work_load = (ALPHA * (self.iteration as f64 * BETA).exp()) as usize;
        let mut path = Wrapping(0);
        let mut rng = rand::thread_rng();
        for _ in 0..work_load {
            path += Wrapping(rng.gen::<u64>())
        }
        if path.0 & 1 == 0 {
            vec![(Coord { x: 0, y: 0 }, None)]
        } else {
            vec![(Coord { x: 1, y: 0 }, None)]
        }
    }
}
