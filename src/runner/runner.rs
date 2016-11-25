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

mod util {

    use std::io;
    use std::path::{Path, PathBuf};

    // Caveats: Doesn't revert ".." in a sensible way
    fn revert_path<P>(a: P) -> PathBuf
        where P: AsRef<Path>
    {
        let mut p = PathBuf::new();
        for _ in a.as_ref().components() {
            p.push("..");
        }
        p
    }

    // Caveats: Won't handle the cases where one is relative and the other is
    // absolute.  Both `a` and `b` are expectd to share a common base whether
    // that is the file system root or the "current" directory.
    fn route_between_<P>(a: P, b: P) -> PathBuf
        where P: AsRef<Path>
    {
        let mut a = a.as_ref().components();
        let mut b = b.as_ref().components();
        loop {
            match (a.next(), b.next()) {
                (Some(a_comp), Some(b_comp)) => {
                    if a_comp == b_comp {
                        continue;
                    } else {
                        let mut p = PathBuf::new();
                        p.push("..");
                        p.push(revert_path(a.as_path()));
                        p.push(b_comp.as_os_str());
                        p.push(b.as_path());
                        return p;
                    }
                }
                (Some(_), None) => {
                    let mut p = PathBuf::new();
                    p.push("..");
                    p.push(revert_path(a.as_path()));
                    return p;
                }
                (None, Some(b_comp)) => {
                    let mut p = PathBuf::new();
                    p.push(b_comp.as_os_str());
                    p.push(b.as_path());
                    return p;
                }
                (None, None) => {
                    return PathBuf::new();
                }
            }
        }
    }

    pub fn route_between<P>(a: P, b: P) -> io::Result<PathBuf>
        where P: AsRef<Path>
    {
        let a = try!(a.as_ref().canonicalize());
        let b = try!(b.as_ref().canonicalize());
        Ok(route_between_(a, b))
    }

    #[cfg(test)]
    mod test {
        use std::path::PathBuf;
        use super::{revert_path, route_between_};

        #[test]
        fn test_revert_path()
        {
            let cases = [("", ""),
                         ("foo", ".."),
                         ("for/bar", "../.."),
                         ("foo/bar/baz", "../../..")];
            for &(a, b) in cases.iter() {
                println!("Recersion from {} should be {}", a, b);
                let a = PathBuf::from(a);
                let b = PathBuf::from(b);
                assert_eq!(revert_path(&a), b);
            }
        }

        #[test]
        fn test_route_between_()
        {
            let cases = [("foo", "foo/baz", "baz"),
                         ("foo/bar", "foo/baz", "../baz"),
                         ("foo/bar/xyzzy", "foo/baz", "../../baz"),
                         ("foo/bar/xyzzy", "foo/baz/yzxxz", "../../baz/yzxxz"),
                         ("foo/bar", "foo/baz/yzxxz", "../baz/yzxxz"),
                         ("foo", "foo/baz/yzxxz", "baz/yzxxz"),
                         ("", "foo/baz/yzxxz", "foo/baz/yzxxz")];
            for &(a, b, c) in cases.iter() {
                println!("From {} to {} should be {}", a, b, c);
                let a = PathBuf::from(a);
                let b = PathBuf::from(b);
                let c = PathBuf::from(c);
                assert_eq!(route_between_(&a, &b), c);
            }
        }
    }
}
