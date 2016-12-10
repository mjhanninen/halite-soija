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

use ua::{Action, Choice, Economic, Environment, Frame, Mask, Occupation,
         Point, Production, Space, State, Tag, Wave};

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
fn compute_outward_utility<F>(source: &F,
                              me: &Tag,
                              productions: &Vec<Production>,
                              occupations: &Vec<Occupation>,
                              par: &Par,
                              turns_left: i32)
    -> f32
    where F: Frame
{
    debug_assert!(source.ref_on(occupations).tag != *me);
    let gamma_dist = (par.gamma.ln() / par.prod_per_turn).exp();
    // XXX: These scans waste resources like buffers that could be re-used.
    // If this approach works (i.e. the bot is better than its predecessor)
    // change the architecture so that all the Dijkstra scans in one turn use
    // the same buffer.
    source.to_point()
          .dijkstra_scan(|z| {
              let occupation = z.ref_on(occupations);
              if occupation.tag == *me {
                  None
              } else {
                  Some(occupation.strength)
              }
          })
          .map(|(dist, z)| {
              let turns_till_capture = (dist as f32 / par.prod_per_turn)
                  .ceil() as i32;
              let capacity = *z.ref_on(productions) as f32;
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
    -> Vec<(Point<'a>, f32)>
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

               (z.to_point().hack(body.space), u)
           })
           .collect()
    } else {
        vec![]
    }
}

fn compute_action_utilities<'a>(choices: &mut Vec<Choice<Option<f32>>>,
                                who: &Tag,
                                body: &'a Mask,
                                productions: &Vec<Production>,
                                occupations: &Vec<Occupation>,
                                rim_utilities: &Vec<(Point<'a>, f32)>,
                                par: &Par)
{
    let sink = body;
    let mut zs: Vec<Point<'a>> = Vec::new();
    for &(ref target, ref fwd_utility) in rim_utilities.iter() {
        debug_assert!(target.ref_on(occupations).tag != *who);
        // Here `r` is the "resistance" faced when attempting to occupy the
        // target cell.  Currently it is just the strength but later on we
        // might want to change it to some functional form the strength.
        // Maybe of time too.  Note that this should be consistent with the
        // resistance used in calculating the forward utilities for the rim
        // cells.  Currently this probably isn't the case.
        let r = target.ref_on(occupations).strength;
        // Here `s` is the total strength that will be channeled to the target
        // frame if the wave is set into (backwards) motion from the current
        // front right now.
        let mut s = 0;
        // Here `p` is the productive capacity of the partial wave up to this
        // front.
        let mut p = 0;
        // Propagate the wave from the target cell until the frame contains
        // enough strength to overcome the resistance.
        zs.clear();
        let source = Mask::singleton(&target);
        let mut wave = Wave::between(&source, sink);
        for t in 2.. {
            let front = wave.front(t).unwrap();
            s += p;
            for cell in front {
                debug_assert!(cell.ref_on(occupations).tag == *who);
                s += cell.ref_on(occupations).strength;
                p += *cell.ref_on(productions);
            }
            if s > r {
                let u = fwd_utility * par.gamma.discount(t as i32);
                let front = wave.front(t).unwrap();
                for t2 in 2..t {
                    let front = wave.front(t2).unwrap();
                    for cell in front {
                        let mut u_still = cell.mut_on(choices).get_mut(&None);
                        if let Some(ref mut u_2) = *u_still {
                            *u_2 = u;
                        } else {
                            *u_still = Some(u);
                        }
                    }
                }
                {
                    let front = wave.front(t).unwrap();
                    for cell in front {
                        let flux = cell.on(&wave);
                        /*
                        // TODO: Set utility according to where
                        for x in flux.sources() {
                        }
                        */
                    }
                }
                break;
            }
        }
    }
}

fn select_actions(space: &Space,
                  occupations: &Vec<Occupation>,
                  who: &Tag,
                  utilities: &Vec<Choice<Option<f32>>>)
    -> Vec<Action>
{
    // As a first stab at the action selection we just select the action that
    // seems to yield the maximal utility for each cell without considering
    // the "externailities" of that action.
    //
    // Later we might improve the algorithm to, yes, greedily pick the maximal
    // utility yielding actions but then updating the utilities of the
    // "neighboring" actions so that the cost from conflicting actions is
    // minimized.
    //
    space.points()
         .filter(|pt| pt.ref_on(occupations).tag == *who)
         .map(|pt| (pt.coord(), pt.ref_on(utilities).find_max_action()))
         .collect()
}

impl Brain for TeddyBrain
{
    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        debug_assert!(state.turn <= self.environment.total_turns);
        let turns_left = (self.environment.total_turns - state.turn) as i32;
        let me = self.environment.my_tag;
        let my_body = Mask::create(&self.environment.space, |z: &Point| {
            z.ref_on(&state.occupation_map).tag == me
        });
        let rim_utilities = compute_rim_utility(&me,
                                                &my_body,
                                                &self.environment
                                                     .production_map,
                                                &state.occupation_map,
                                                &PAR,
                                                turns_left);
        let mut utilities = Vec::new();
        compute_action_utilities(&mut utilities,
                                 &me,
                                 &my_body,
                                 &self.environment.production_map,
                                 &state.occupation_map,
                                 &rim_utilities,
                                 &PAR);
        select_actions(&self.environment.space,
                       &state.occupation_map,
                       &self.environment.my_tag,
                       &utilities)
    }
}
