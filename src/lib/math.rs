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

pub trait Economic
{
    fn perpetuity(self) -> Self;
    fn annuity(self, n: i32) -> Self;
    fn discount(self, n: i32) -> Self;
    fn discount_of(self) -> Self;
}

impl Economic for f32
{
    #[inline]
    fn perpetuity(self) -> f32
    {
        assert!(0.0 <= self && self < 1.0);
        self / (1.0 - self)
    }

    #[inline]
    fn annuity(self, n: i32) -> f32
    {
        self.perpetuity() * (1.0 - self.discount(n))
    }

    #[inline]
    fn discount(self, n: i32) -> f32
    {
        self.powi(n)
    }

    #[inline]
    fn discount_of(self) -> f32
    {
        assert!(self != -1.0);
        1.0 / (1.0 + self)
    }
}

impl Economic for f64
{
    #[inline]
    fn perpetuity(self) -> f64
    {
        assert!(0.0 <= self && self < 1.0);
        self / (1.0 - self)
    }

    #[inline]
    fn annuity(self, n: i32) -> f64
    {
        self.perpetuity() * (1.0 - self.discount(n))
    }

    #[inline]
    fn discount(self, n: i32) -> f64
    {
        self.powi(n)
    }

    #[inline]
    fn discount_of(self) -> f64
    {
        assert!(self != -1.0);
        1.0 / (1.0 + self)
    }
}

#[cfg(test)]
mod test {

    use super::Economic;

    #[test]
    fn test_discounting()
    {
        // Case from: https://en.wikipedia.org/wiki/Discounting
        let r = 0.12_f32;
        let d = r.discount_of();
        assert_eq!((10_000.0 * d.discount(5)).round(), 5674.0);
    }
}
