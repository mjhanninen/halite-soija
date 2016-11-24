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

use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::string::String;

use ua::space::Space;

#[derive(Debug)]
pub enum Error
{
    Io(io::Error),
    Runtime(String),
}

impl error::Error for Error
{
    fn description(&self) -> &str
    {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Runtime(_) => "execution failed during run-time",
        }
    }
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self
    {
        Error::Io(e)
    }
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Runtime(ref stderr) => {
                write!(f, "run-time failure: \"{}\"", stderr)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bot
{
    exe_path: String,
    brain: Option<String>,
    params: BTreeMap<String, f32>,
}

impl Bot
{
    pub fn new(exe_path: String) -> Self
    {
        Bot {
            exe_path: exe_path,
            brain: None,
            params: BTreeMap::new(),
        }
    }

    pub fn set_param(&mut self, key: &str, value: f32) -> &mut Self
    {
        self.params.insert(key.to_owned(), value);
        self
    }
}

impl fmt::Display for Bot
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        try!(write!(f, "{}", self.exe_path));
        if let Some(ref brain) = self.brain {
            try!(write!(f, " -b {}", brain));
        }
        for (k, v) in self.params.iter() {
            try!(write!(f, " {}={}", k, v));
        }
        Ok(())
    }
}

pub fn mk_bot_script<P>(bot: &Bot, script_path: P) -> Result<(), Error>
    where P: AsRef<Path>
{
    {
        let mut f = try!(File::create(&script_path));
        try!(write!(f, "#!/bin/bash\n{}\n", bot));
    }
    let mut p = try!(fs::metadata(&script_path)).permissions();
    let m = p.mode();
    p.set_mode(m | 0o110);
    try!(fs::set_permissions(&script_path, p));
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Match
{
    server: String,
    seed: Option<usize>,
    width: i16,
    height: i16,
    bots: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Outcome
{
    pub seed: u64,
    pub rankings: Vec<u32>,
}

impl Match
{
    pub fn new(server: &str, space: &Space) -> Self
    {
        Match {
            server: server.to_owned(),
            seed: None,
            width: space.width(),
            height: space.height(),
            bots: Vec::new(),
        }
    }

    pub fn seed(mut self, seed: usize) -> Self
    {
        self.seed = Some(seed);
        self
    }

    pub fn bot(mut self, path: &str) -> Self
    {
        self.bots.push(path.to_owned());
        self
    }

    fn parse_output(&self, output: &[u8]) -> Result<Outcome, Error>
    {
        use std::borrow::Cow;
        let output = String::from_utf8_lossy(output);
        let n_bots = self.bots.len();
        let mut lines = output.lines();
        fn err<'a>(output: &Cow<'a, str>) -> Error
        {
            Error::Runtime(output.clone().into_owned())
        }
        // Ignore bot commands
        for _ in 0..n_bots {
            try!(lines.next().ok_or(err(&output)));
        }
        let seed;
        // The replay file name and seed
        {
            let l = try!(lines.next().ok_or(err(&output)));
            let mut s = l.split(' ');
            let _hlt_path = try!(s.next().ok_or(err(&output)));
            seed = try!(s.next()
                         .ok_or(err(&output))
                         .and_then(|s| {
                             s.parse::<u64>().map_err(|_| err(&output))
                         }));
            if s.next().is_some() {
                return Err(err(&output));
            }
        }
        let mut rankings = Vec::new();
        // The results
        for _ in 0..n_bots {
            let l = try!(lines.next().ok_or(err(&output)));
            let mut s = l.split(' ');
            let _rank = try!(s.next()
                              .ok_or(err(&output))
                              .and_then(|s| {
                                  s.parse::<u32>().map_err(|_| err(&output))
                              }));
            let bot_id = try!(s.next()
                               .ok_or(err(&output))
                               .and_then(|s| {
                                   s.parse::<u32>().map_err(|_| err(&output))
                               }));
            rankings.push(bot_id);
            if s.next().is_some() {
                return Err(err(&output));
            }
        }
        // The two odd extra lines
        for _ in 0..2 {
            let () = try!(lines.next().ok_or(err(&output)).and_then(|l| {
                if l == " " {
                    Ok(())
                } else {
                    Err(err(&output))
                }
            }));
        }
        if lines.next().is_none() {
            Ok(Outcome {
                seed: seed,
                rankings: rankings,
            })
        } else {
            Err(err(&output))
        }
    }

    pub fn run(&mut self) -> Result<Outcome, Error>
    {
        let mut command = Command::new(self.server.clone());
        command.arg("-q")
               .arg("-d")
               .arg(format!("{} {}", self.width, self.height));
        if let Some(seed) = self.seed {
            command.arg("-s").arg(format!("{}", seed));
        }
        for bot in self.bots.iter() {
            command.arg(bot);
        }
        let output = try!(command.output());
        if output.status.success() {
            self.parse_output(&output.stdout)
        } else {
            Err(Error::Runtime(String::from_utf8_lossy(&output.stderr)
                .into_owned()))
        }
    }
}
