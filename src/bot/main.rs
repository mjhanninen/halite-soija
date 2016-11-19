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

use std::io::Write;
use std::env;
use std::str::FromStr;

mod brain;

enum Brain
{
    LoneExpander,
    Probe,
    Simple,
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
            _ => Err(format!("no brain with name '{}'", s)),
        }
    }
}

struct Config
{
    brain: Brain,
    log_path: Option<String>,
}

enum OptionParsing
{
    ShowUsage(String),
    Failed,
    Config(Config),
}

fn parse_options() -> OptionParsing
{
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Display this usage information")
        .optopt("b", "brain", "Select the bot brain to use", "NAME")
        .optopt("l", "log", "Produce log of internal events to file", "FILE");
    let args = env::args().collect::<Vec<String>>();
    if let Ok(matches) = opts.parse(&args[1..]) {
        if matches.opt_present("h") {
            let brief = format!("usage: {} [ options ]", args[0]);
            OptionParsing::ShowUsage(opts.usage(&brief))
        } else if !matches.free.is_empty() {
            OptionParsing::Failed
        } else {
            let mut config = Config {
                brain: Brain::default(),
                log_path: None,
            };
            if let Some(brain_name) = matches.opt_str("b") {
                match Brain::from_str(&brain_name) {
                    Ok(brain) => {
                        config.brain = brain;
                    }
                    Err(_) => {
                        return OptionParsing::Failed;
                    }
                }
            }
            if let Some(log_path) = matches.opt_str("l") {
                config.log_path = Some(log_path);
            }
            OptionParsing::Config(config)
        }
    } else {
        OptionParsing::Failed
    }
}

fn main()
{
    match parse_options() {
        OptionParsing::Config(config) => {
            match config.brain {
                Brain::LoneExpander => brain::lone_expander::run(),
                Brain::Probe => brain::probe::run(),
                Brain::Simple => brain::simple::run(),
            }
        }
        OptionParsing::ShowUsage(usage) => {
            println!("{}", usage);
            std::process::exit(0);
        }
        OptionParsing::Failed => {
            writeln!(std::io::stderr(),
                     "Error: Bad command line (try -h for help)")
                .unwrap();
            std::process::exit(1);
        }
    }
}
