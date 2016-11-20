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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::Write;
use std::result::Result;

pub trait LoggedUnwrap<T>
{
    fn unwrap_or_log(self, log: &mut Write) -> T;
}

impl<T, E> LoggedUnwrap<T> for Result<T, E>
    where E: Debug
{
    fn unwrap_or_log(self, log: &mut Write) -> T
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                if write!(log, "unwrapping failed due to: {:?}\n", e).is_ok() {
                    log.flush().unwrap();
                }
                panic!();
            }
        }
    }
}

#[inline]
pub fn f32_cmp(a: &f32, b: &f32) -> Ordering
{
    if a < b {
        Ordering::Less
    } else if b < a {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
