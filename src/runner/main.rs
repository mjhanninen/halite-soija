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

extern crate ua;

mod runner;

use ua::space::Space;

fn main()
{
    let mut bot1 = runner::Bot::new("./target/release/MyBot".to_owned());
    bot1.set_param("aggression", 100.0)
        .set_param("production", 50.0);
    println!("Bot command: {}", bot1);
    runner::mk_bot_script(&bot1, "./tmp/foo").unwrap();
    let space = Space::with_dims(20, 20);
    let outcome = runner::Match::new("../Environment/halite", &space)
        .seed(123)
        .bot("./target/release/MyBot")
        .bot("./target/release/MyBot")
        .bot("./target/release/MyBot")
        .run()
        .unwrap();
    println!("Halite Tournament Runner");
    println!("Outcome = {:?}", outcome);
}
