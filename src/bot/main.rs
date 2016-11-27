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

#![allow(non_snake_case)]

extern crate getopts;
extern crate rand;
extern crate ua;

use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::str::FromStr;

mod brain;
mod params;

enum Brain
{
    LoneExpander,
    Probe,
    Simple,
    Teddy,
}

impl Brain
{
    fn default() -> Self
    {
        if cfg!(feature = "probe") {
            Brain::Probe
        } else {
            Brain::LoneExpander
        }
    }
}

impl FromStr for Brain
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s {
            "lone_expander" => Ok(Brain::LoneExpander),
            "probe" => Ok(Brain::Probe),
            "simple" => Ok(Brain::Simple),
            "teddy" => Ok(Brain::Teddy),
            _ => Err(format!("no brain with name '{}'", s)),
        }
    }
}

#[allow(dead_code)]
struct Config
{
    brain: Brain,
    log_path: Option<String>,
    params: params::Params,
}

enum OptionParsing
{
    ShowUsage(String),
    Config(Config),
}

fn parse_free(free: &str) -> Result<(String, f32), &'static str>
{
    static ERR_MSG: &'static str = "invalid extra argument (should be of form \
                                    KEY=VALUE)";
    let mut s = free.split('=');
    let k = try!(s.next().ok_or(ERR_MSG));
    let v = try!(s.next().ok_or(ERR_MSG));
    let n = try!(v.parse::<f32>().map_err(|_| ERR_MSG));
    if s.next().is_none() {
        Ok((k.to_owned(), n))
    } else {
        Err(ERR_MSG)
    }
}

fn parse_options() -> Result<OptionParsing, String>
{
    use getopts::{Options, ParsingStyle};
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::StopAtFirstFree)
        .optflag("h", "help", "Display this usage information")
        .optopt("b", "brain", "Select the bot brain to use", "NAME")
        .optopt("l", "log", "Produce log of internal events to file", "FILE");
    let args = env::args().collect::<Vec<String>>();
    if let Ok(matches) = opts.parse(&args[1..]) {
        if matches.opt_present("h") {
            let brief = format!("usage: {} [ options ]", args[0]);
            Ok(OptionParsing::ShowUsage(opts.usage(&brief)))
        } else {
            let config = Config {
                brain: match matches.opt_str("b") {
                    Some(brain_name) => try!(Brain::from_str(&brain_name)),
                    None => Brain::default(),
                },
                log_path: matches.opt_str("l"),
                params: try!(matches.free.iter()
                             .map(|f| parse_free(&f))
                             .collect::<Result<BTreeMap<String, f32>, _>>()
                             .map_err(|s| s.to_owned())),
            };
            Ok(OptionParsing::Config(config))
        }
    } else {
        Err("bad command line (try -h for help)".to_owned())
    }
}

fn main()
{
    match parse_options() {
        Ok(OptionParsing::Config(config)) => {
            let mold: Box<brain::Mold> = match config.brain {
                Brain::Teddy => Box::new(brain::teddy::TeddyMold),
                Brain::LoneExpander => Box::new(brain::lone_expander::LoneMold),
                Brain::Probe => Box::new(brain::probe::ProbeMold),
                Brain::Simple => Box::new(brain::simple::SimpleMold),
            };
            if let Err(why) = brain::brain::run_forever(mold.as_ref(),
                                                        &config.params) {
                writeln!(std::io::stderr(),
                         "Error while driving brain: {:?}",
                         why)
                    .unwrap();
                std::process::exit(1);
            }
        }
        Ok(OptionParsing::ShowUsage(usage)) => {
            println!("{}", usage);
            std::process::exit(0);
        }
        Err(why) => {
            writeln!(std::io::stderr(), "Error: {}", why).unwrap();
            std::process::exit(1);
        }
    }
}
