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
pub struct DijsktraScan<'a, F>
{
    cost_fn: F,
    visited: Vec<bool>,
    queue: BinaryHeap<(i16, Point<'a>)>,
}

impl<'a> Point<'a>
{
    pub fn dijkstra_scan<F>(&self, cost_fn: F) -> DijsktraScan<F>
        where F: Fn(&Point) -> Option<i16>
    {
        let mut queue = BinaryHeap::with_capacity(self.space().len() * 2);
        if let Some(init_cost) = cost_fn(&self) {
            queue.push((-init_cost, self.clone()));
        }
        DijsktraScan {
            cost_fn: cost_fn,
            visited: vec![false; self.space().len()],
            // The upper bound of the priority queue is 2 * width * height;
            // think of the general case to add three new cell to the search
            // queue you have to consume one cell from the existing queue.
            // This means that there are at most 2 untried branches per
            // visited cell in the search queue.
            queue: queue,
        }
    }
}

impl<'a, F> Iterator for DijsktraScan<'a, F>
    where F: Fn(&Point) -> Option<i16>
{
    type Item = (i16, Point<'a>);

    fn next(&mut self) -> Option<Self::Item>
    {
        // The queue is implemented as a maximum heap. We push the negative of
        // the cost to get the minimum cost ordering.
        while let Some((neg_cost, frame)) = self.queue.pop() {
            let unvisited = {
                let mut visited = frame.on_mut(&mut self.visited);
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
                    if !*adj_frame.on(&self.visited) {
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
