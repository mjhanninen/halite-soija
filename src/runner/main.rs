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

extern crate bbt;
extern crate data_encoding;
extern crate md5;
extern crate rand;
extern crate ua;

mod runner;

use std::process;

use ua::space::Space;
use ua::util::f32_cmp;
use bbt::Rating;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;


use runner::{Bot, Env};

fn do_match(env: &Env, space: &Space, bots: &mut [(Bot, Rating)])
{
    let outcome = {
        let mut game = runner::Match::new(space);
        for bot in bots.iter() {
            game = game.bot(&bot.0);
        }
        game.run(&env).unwrap()
    };
    let rater = bbt::Rater::default();
    let old_ratings =
        bots.iter().map(|bot| vec![bot.1.clone()]).collect::<Vec<_>>();
    let new_ratings = rater.update_ratings(old_ratings,
                                           outcome.rankings.clone())
                           .unwrap();
    for ix in 0..bots.len() {
        bots[ix].1 = new_ratings[ix][0].clone();
    }
}

fn randomize_bot<R: Rng>(bot: &mut Bot, rng: &mut R)
{
    let wt_range = Range::new(0_f32, 100.0);
    let mm_range = Range::new(0_f32, 150.0);
    let df_range = Range::new(0_f32, 1.0);
    bot.set_param("aggression_weight", wt_range.ind_sample(rng))
       .set_param("density_weight", wt_range.ind_sample(rng))
       .set_param("expansion_weight", wt_range.ind_sample(rng))
       .set_param("discount_factor", df_range.ind_sample(rng))
       .set_param("minimum_movable_strength", mm_range.ind_sample(rng));
}

const POPULATION_SIZE: usize = 100;
const TOURNAMENT_SIZE: usize = 1000;

fn do_tournament(env: &Env, proto: &Bot)
{
    let mut bots = vec![(proto.clone(), Rating::default()); POPULATION_SIZE];
    let mut rng = rand::thread_rng();
    for bot in bots.iter_mut() {
        randomize_bot(&mut bot.0, &mut rng);
    }
    // Run matches
    let n_players_range = Range::new(2, 7);
    let map_size_range = Range::new(5, 9);
    for ix in 0..TOURNAMENT_SIZE {
        bots.sort_by(|a, b| {
            f32_cmp(&(b.1.sigma() as f32), &(a.1.sigma() as f32))
        });
        let max_sigma = bots[0].1.sigma();
        let min_sigma = bots[bots.len() - 1].1.sigma();
        let n_players = n_players_range.ind_sample(&mut rng);
        let map_size = map_size_range.ind_sample(&mut rng) * 5;
        println!("Game {}, Map = {}x{}, Players = {}, Sigma = ({:.*}, {:.*})",
                 ix + 1,
                 map_size,
                 map_size,
                 n_players,
                 2,
                 min_sigma,
                 2,
                 max_sigma);
        let space = Space::with_dims(map_size, map_size);
        do_match(env, &space, &mut bots[0..n_players]);
    }
    // Print results
    bots.sort_by(|a, b| f32_cmp(&(b.1.mu() as f32), &(a.1.mu() as f32)));
    for bot in bots {
        println!("{:?}", bot);
    }
}

fn main()
{
    // Next up generate 100 different bots and
    match runner::Env::new("./tmp/runner", "../Environment/halite") {
        Ok(env) => {
            let proto = runner::Bot::new("./target/release/MyBot").unwrap();
            do_tournament(&env, &proto);
        }
        Err(e) => {
            println!("Failed to initialize environment: {:?}", e);
            process::exit(1);
        }
    };
}
