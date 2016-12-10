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

//! Uniform cost scan (or Dijkstra scan) over the space.

use std::collections::BinaryHeap;

use dir::Dir;
use space::frame::Frame;
use space::point::Point;

/// Uniform-cost scan over the space.
pub struct DijsktraScan<'a, C>
{
    cost_fn: C,
    visited: Vec<bool>,
    queue: BinaryHeap<(i16, Point<'a>)>,
}

impl<'a, C> DijsktraScan<'a, C>
{
    pub fn new(pnt: Point<'a>, cost_fn: C) -> Self
        where C: Fn(&Point) -> Option<i16>
    {
        let s = pnt.space().len();
        let mut queue = BinaryHeap::with_capacity(2 * s);
        if let Some(init_cost) = cost_fn(&pnt) {
            queue.push((-init_cost, pnt));
        }
        DijsktraScan {
            cost_fn: cost_fn,
            visited: vec![false; s],
            // The upper bound of the priority queue is 2 * width * height;
            // think of the general case to add three new cell to the search
            // queue you have to consume one cell from the existing queue.
            // This means that there are at most 2 untried branches per
            // visited cell in the search queue.
            queue: queue,
        }
    }
}

impl<'a> Point<'a>
{
    pub fn dijkstra_scan<C>(&self, cost_fn: C) -> DijsktraScan<'a, C>
        where C: Fn(&Point) -> Option<i16>
    {
        DijsktraScan::new(self.clone(), cost_fn)
    }
}

impl<'a, C> Iterator for DijsktraScan<'a, C>
    where C: Fn(&Point) -> Option<i16>
{
    type Item = (i16, Point<'a>);

    fn next(&mut self) -> Option<Self::Item>
    {
        // The queue is implemented as a maximum heap. We push the negative of
        // the cost to get the minimum cost ordering.
        while let Some((neg_cost, frame)) = self.queue.pop() {
            let unvisited = {
                let mut visited = frame.mut_on(&mut self.visited);
                if *visited {
                    false
                } else {
                    *visited = true;
                    true
                }
            };
            if unvisited {
                for dir in Dir::dirs() {
                    let adj_frame = frame.adjacent_in(dir);
                    if !*adj_frame.ref_on(&self.visited) {
                        if let Some(step_cost) = (self.cost_fn)(&adj_frame) {
                            self.queue.push((neg_cost - step_cost, adj_frame))
                        }
                    }
                }
                return Some((-neg_cost, frame));
            }
        }
        None
    }
}
