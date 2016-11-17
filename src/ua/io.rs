use std::io::{self, BufRead, Stdin, Stdout, Write};

use ua::world::*;

struct Connection {
    input: Stdin,
    output: Stdout,
    buffer: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    IoError(io::Error),
    Other(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            input: io::stdin(),
            output: io::stdout(),
            buffer: String::new(),
        }
    }
    fn recv_string(&mut self) -> Result<usize, Error> {
        self.buffer.clear();
        self.input.lock().read_line(&mut self.buffer).map_err(Error::from)
    }
    fn parse_err(&self, message: &str) -> Error {
        Error::ParseError(format!("{}: {:?}", message, &self.buffer))
    }
    pub fn recv_environment(&mut self) -> Result<Environment, Error> {
        // Tag
        try!(self.recv_string());
        let my_tag = try!(self.buffer
            .trim_right()
            .parse::<Tag>()
            .map_err(|_| self.parse_err("bad player tag")));
        // World dimensions
        try!(self.recv_string());
        let (width, height) = {
            let mut parts = self.buffer.trim_right().split(" ");
            let width = parts.next()
                .ok_or(self.parse_err("missing map width"))?
                .parse::<i16>()
                .map_err(|e| self.parse_err("bad map width"))?;
            let height = parts.next()
                .ok_or(self.parse_err("missing map height"))?
                .parse::<i16>()
                .map_err(|_| self.parse_err("bad map height"))?;
            if parts.next() != None {
                return Err(self.parse_err("unconsumed input after parsing map \
                                           size message"));
            }
            (width, height)
        };
        // Production map
        let mut environment = Environment::create(my_tag, width, height);
        try!(self.recv_string());
        for part in self.buffer.trim_right().split(' ') {
            let production = try!(part.parse::<Production>()
                .map_err(|_| self.parse_err("bad production level")));
            environment.production_map.push(production);
        }
        if environment.production_map.len() != environment.space.size() {
            return Err(self.parse_err("production map size mismatches expected \
                                       size"));
        }
        Ok(environment)
    }
    pub fn recv_state(&mut self,
                      environment: &Environment,
                      state: &mut State)
                      -> Result<(), Error> {
        assert_eq!(state.occupation_map.len(), environment.space.size());
        try!(self.recv_string());
        let mut parts = self.buffer.trim_right().split(" ");
        // Occupiers
        let mut run_length = 0;
        let mut tag = 0;
        for cell in state.occupation_map.iter_mut() {
            while run_length == 0 {
                run_length = parts.next()
                    .ok_or(self.parse_err("missing run-length"))?
                    .parse::<u16>()
                    .map_err(|_| self.parse_err("bad run-length"))?;
                tag = parts.next()
                    .ok_or(self.parse_err("missing player tag"))?
                    .parse::<Tag>()
                    .map_err(|_| self.parse_err("bad player tag"))?;
            }
            cell.tag = tag;
        }
        if run_length != 0 {
            return Err(self.parse_err("non-zero residual run-length"));
        }
        // Strengths
        for cell in state.occupation_map.iter_mut() {
            let strength = parts.next()
                .ok_or(self.parse_err("missing strength level"))?
                .parse::<Strength>()
                .map_err(|_| self.parse_err("bad strength level"))?;
            cell.strength = strength;
        }
        if parts.next() != None {
            return Err(self.parse_err("unconsumed input after parse state \
                                       message"));
        }
        Ok(())
    }
    pub fn send_init(&mut self,
                     environment: &Environment,
                     name: &str)
                     -> Result<(), Error> {
        write!(self.output.lock(), "{}_{}\n", name, environment.my_tag)
            .map_err(Error::from)
    }
    pub fn send_moves<'a, I>(&mut self, moves: I) -> Result<(), Error>
        where I: Iterator<Item = &'a Move>
    {
        let mut count = 0;
        let mut output_handle = self.output.lock();
        for &(ref pos, ref dir) in moves {
            let encoded_dir = match *dir {
                Dir::Still => 0,
                Dir::North => 1,
                Dir::East => 2,
                Dir::South => 3,
                Dir::West => 4,
            };
            if count > 0 {
                try!(write!(output_handle, " "));
            }
            try!(write!(output_handle, "{} {} {}", pos.0, pos.1, encoded_dir));
            count += 1;
        }
        write!(output_handle, "\n").map_err(Error::from)
    }
}
