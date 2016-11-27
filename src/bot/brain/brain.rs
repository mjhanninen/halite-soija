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

use ua::*;

use params::Params;

pub trait Brain
{
    fn tick(&mut self, state: &State) -> Vec<Action>;
}

pub trait Mold
{
    fn name(&self) -> Cow<str>;

    fn reanimate(&self,
                 params: &Params,
                 environment: Environment,
                 init_state: &State)
        -> Box<Brain>;
}

#[derive(Debug)]
pub enum Error
{
    Io(io::Error),
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self
    {
        Error::Io(e)
    }
}

pub fn run_forever(mold: &Mold, params: &Params) -> Result<(), Error>
{
    let mut connection = io::Connection::new();
    let environment = try!(connection.recv_environment());
    let mut state_frame = State::for_environment(&environment);
    try!(connection.recv_state(&mut state_frame));
    let my_tag = environment.my_tag;
    let mut brain = mold.reanimate(params, environment, &state_frame);
    let name = &format!("UA_{}", mold.name());
    try!(connection.send_ready(&my_tag, name));
    loop {
        try!(connection.recv_state(&mut state_frame));
        let actions = brain.tick(&state_frame);
        try!(connection.send_actions(actions.iter()));
    }
}
