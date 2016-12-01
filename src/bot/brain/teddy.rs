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

use ua::{Action, Economic, Environment, Frame, Mask, Occupation, State, Wave};

use brain::{Brain, Mold};
use params::Params;

pub struct TeddyMold;

impl Mold for TeddyMold
{
    fn reanimate(&self,
                 _params: &Params,
                 environment: Environment,
                 _init_state: &State)
        -> Box<Brain>
    {
        Box::new(TeddyBrain { environment: environment })
    }

    fn name(&self) -> Cow<str>
    {
        Cow::Borrowed(&"Teddy")
    }
}

pub struct TeddyBrain
{
    environment: Environment,
}

fn compute_outward_utility(z: &Frame,
                           avoid: &Mask,
                           occupations: &Vec<Occupation>)
    -> i16
{
    z.dijkstra_scan(|q: &Frame| {
         if *q.on(avoid) {
             None
         } else {
             Some(q.on(occupations).strength)
         }
     })
     .map(|e: (i16, Frame)| e.0)
     .sum()
}

impl Brain for TeddyBrain
{
    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        // 1. Update the mask of the current occupied area and the rim.
        //
        // 2. Generate a wave from the current rim outwards and then compute
        //    utilities for the rim cells by rolling the wave back.
        let me = self.environment.my_tag;
        let my_body = Mask::create(&self.environment.space, |z: &Frame| {
            z.on(&state.occupation_map).tag == me
        });
        // Compute the utility frontier along the rim
        let wave = Wave::from(&my_body);
        if let Some(rim) = wave.front(0) {
            for z in rim {
                let _u = compute_outward_utility(&z,
                                                 &my_body,
                                                 &state.occupation_map);
            }
        } else {
            // Yeah,
        }
        //
        let gamma = 0.5_f32;
        let space = &self.environment.space;
        let mut my_extensions = Wave::new(space);
        my_extensions.ripple(&my_body);
        let mut prods = vec![0.0_f32; space.len()];
        for z in my_extensions.contraction() {
            /* let max_succ_reward = z.successors().map(|s: &Frame| {
             * s.on(&prods)
             * }).max();
             * */
            let max_succ_reward = 0.0;
            let curr_reward = *z.on(&self.environment.production_map) as f32 *
                              gamma.perpetuity();
            let resistance = z.on(&state.occupation_map).strength;
            *z.on_mut(&mut prods) = gamma.discount(resistance as i32) *
                                    (curr_reward + max_succ_reward);
        }
        // Do nothing
        vec![]
    }
}
