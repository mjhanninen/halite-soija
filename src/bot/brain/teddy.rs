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

use ua::{Action, Economic, Environment, Frame, Mask, Occupation, Production,
         State, Tag, Wave};

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

const PROD_PER_TURN: f32 = 16.0;

fn compute_outward_utility(source: &Frame,
                           me: &Tag,
                           gamma: f32,
                           turns_left: u32,
                           productions: &Vec<Production>,
                           occupations: &Vec<Occupation>)
    -> f32
{
    // We measure the distance of each outward path in terms of strength
    // points we need to conquer along that path.  To convert that into a
    // number of turns we assume that we can produce fixed amount of
    // conquering strength per turn.  In reality that number varies over time
    // but this is start.
    let gamma_dist = (gamma.ln() / PROD_PER_TURN).exp();
    source.dijkstra_scan(|z| {
              let occupation = z.on(occupations);
              if occupation.tag == *me {
                  None
              } else {
                  Some(occupation.strength)
              }
          })
          .map(|(dist, z)| {
              let turns_till_capture =
                  (dist as f32 / PROD_PER_TURN).ceil() as i32;
              let capacity = *z.on(productions) as f32;
              let output = gamma.annuity(turns_left as i32 -
                                         turns_till_capture) *
                           capacity;
              gamma_dist.discount(dist as i32) * output
          })
          .sum()
}

impl Brain for TeddyBrain
{
    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        let turns_left = self.environment.total_turns - state.turn;
        let gamma = 0.5_f32;
        let me = self.environment.my_tag;
        let my_body = Mask::create(&self.environment.space, |z: &Frame| {
            z.on(&state.occupation_map).tag == me
        });
        // Compute the expected utilities along the rim consisting of cells
        // immediately adjacent to my bot.
        let wave = Wave::from(&my_body);
        if let Some(rim) = wave.front(1) {
            rim.map(|z| {
                   let utility = compute_outward_utility(&z,
                                                         &me,
                                                         gamma,
                                                         turns_left,
                                                         &self.environment
                                                              .production_map,
                                                         &state.occupation_map);

                   (z, utility)
               })
               .collect::<Vec<_>>();
        }
        // Do nothing
        vec![]
    }
}
