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

#[derive(Clone, Debug)]
struct Par
{
    // The discount factor.
    gamma: f32,
    // We measure the distance of each outward path in terms of strength
    // points we need to conquer along that path.  To convert that into a
    // number of turns we assume that we can produce fixed amount of
    // conquering strength per turn.  In reality that number varies over time
    // but this is start.
    prod_per_turn: f32,
}

const PAR: Par = Par {
    gamma: 0.5,
    prod_per_turn: 16.0,
};

/// Computes the expected "outward" or "explorative" utility for the `source`
/// cell that belongs to the rim of foreign cells surrounding the body of the
/// bot.
fn compute_outward_utility(source: &Frame,
                           me: &Tag,
                           productions: &Vec<Production>,
                           occupations: &Vec<Occupation>,
                           par: &Par,
                           turns_left: i32)
    -> f32
{
    debug_assert!(source.on(occupations).tag != *me);
    let gamma_dist = (par.gamma.ln() / par.prod_per_turn).exp();
    // XXX: These scans waste resources like buffers that could be re-used.
    // If this approach works (i.e. the bot is better than its predecessor)
    // change the architecture so that all the Dijkstra scans in one turn use
    // the same buffer.
    source.dijkstra_scan(|z| {
              let occupation = z.on(occupations);
              if occupation.tag == *me {
                  None
              } else {
                  Some(occupation.strength)
              }
          })
          .map(|(dist, z)| {
              let turns_till_capture = (dist as f32 / par.prod_per_turn)
                  .ceil() as i32;
              let capacity = *z.on(productions) as f32;
              let output = par.gamma
                              .annuity(turns_left as i32 - turns_till_capture) *
                           capacity;
              gamma_dist.discount(dist as i32) * output
          })
          .sum()
}


/// Computes the expected utilities for conquering any of the cells belonging
/// to the rim of foreign or unoccupied cells immediately surroinding the body
/// of the bot.
fn compute_rim_utility<'a>(who: &Tag,
                           body: &'a Mask,
                           productions: &Vec<Production>,
                           occupations: &Vec<Occupation>,
                           par: &Par,
                           turns_left: i32)
    -> Vec<(Frame<'a>, f32)>
{
    debug_assert!(turns_left >= 0);
    // XXX: Here we need only the first wavefront. The wave function should be
    // altered so that the wave can be produced only partially thus avoiding
    // wasting work.
    let wave = Wave::from(body);
    if let Some(rim) = wave.front(1) {
        rim.map(|z| {
               let u = compute_outward_utility(&z,
                                               who,
                                               productions,
                                               occupations,
                                               par,
                                               turns_left);

               (z.hack(body.space), u)
           })
           .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

use ua::Coord;
use std::collections::BTreeMap;

type ActioMap = BTreeMap<Coord, (Action, f32)>;

impl Brain for TeddyBrain
{
    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        debug_assert!(state.turn <= self.environment.total_turns);
        let turns_left = (self.environment.total_turns - state.turn) as i32;
        let me = self.environment.my_tag;
        let my_body = Mask::create(&self.environment.space, |z: &Frame| {
            z.on(&state.occupation_map).tag == me
        });
        let _rim = compute_rim_utility(&me,
                                       &my_body,
                                       &self.environment.production_map,
                                       &state.occupation_map,
                                       &PAR,
                                       turns_left);
        // For each of those cells along the rim compute a wave that sweeps
        // across the bot internals and contracts to the cell in question.
        //
        // Each wave will assign an action-reward pair for all internal cells.
        //
        // Once the whole rim has been porcessed in this way each internal
        // cell will pick its utility maximizing action.
        vec![]
    }
}
