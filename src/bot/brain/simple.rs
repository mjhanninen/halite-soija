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

use ua::{Action, Dir, Environment, Frame, Occupation, Point, State};

use params::Params;
use brain::{Brain, Mold};

pub struct SimpleMold;

impl Mold for SimpleMold
{
    fn name(&self) -> Cow<str>
    {
        Cow::Borrowed(&"Simple")
    }

    fn reanimate(&self,
                 _params: &Params,
                 environment: Environment,
                 _init_state: &State)
        -> Box<Brain>
    {
        Box::new(SimpleBrain { environment: environment })
    }
}

pub struct SimpleBrain
{
    environment: Environment,
}

fn tick_site(origin: &Point,
             src: &Occupation,
             occupations: &Vec<Occupation>,
             me: u8)
    -> Option<Dir>
{
    for d in Dir::dirs() {
        let f = origin.adjacent_in(d);
        let tgt = f.ref_on(occupations);
        if tgt.tag != me && tgt.strength < src.strength {
            return Some(*d);
        }
    }
    for d in Dir::dirs() {
        let f = origin.adjacent_in(d);
        let tgt = &occupations[f.ix()];
        if tgt.tag == me && 2 * (tgt.strength as i16) < src.strength as i16 {
            return Some(*d);
        }
    }
    None
}

impl Brain for SimpleBrain
{
    fn tick(&mut self, state: &State) -> Vec<Action>
    {
        let me = self.environment.my_tag;
        let mut actions = vec![];
        for f in self.environment.space.points() {
            let source = &state.occupation_map[f.ix()];
            if source.tag == me {
                if let Some(dir) = tick_site(&f,
                                             source,
                                             &state.occupation_map,
                                             me) {
                    actions.push((f.coord(), Some(dir)));
                }
            }
        }
        actions
    }
}
