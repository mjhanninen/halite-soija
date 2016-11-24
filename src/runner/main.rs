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

extern crate data_encoding;
extern crate md5;
extern crate ua;

mod runner;

use std::process;

use ua::space::Space;

fn main()
{
    match runner::Env::new("./tmp/runner", "../Environment/halite") {
        Ok(env) => {
            let proto = runner::Bot::new("./target/release/MyBot").unwrap();
            let bot1 = proto.clone();
            let mut bot2 = proto.clone();
            bot2.set_param("aggression_weigth", 100.0);
            let mut bot3 = proto.clone();
            bot3.set_param("minimum_movable_strength", 20.0);
            let space = Space::with_dims(20, 20);
            let outcome = runner::Match::new(&space)
                .seed(123)
                .bot(&bot1)
                .bot(&bot2)
                .bot(&bot3)
                .run(&env)
                .unwrap();
            println!("Halite Tournament Runner");
            println!("Outcome = {:?}", outcome);
        }
        Err(e) => {
            println!("Failed to initialize environment: {:?}", e);
            process::exit(1);
        }
    };
}
