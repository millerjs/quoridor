// Copyright (c) 2015-2016 Joshua S. Miller
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use board::Point;
use std::collections::{HashMap, HashSet};
use constants::{N, MAX_DIST};

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq)]
pub struct Path {
    pub nodes: Vec<Point>,
}

impl Path {
    pub fn new() -> Path {
        Path { nodes: vec![] }
    }
}

pub trait AdjacencyMatrix {
    /// Return a boolean specifying whether two points are adjacent
    fn adj(&self, a: Point, b: Point) -> bool;

    /// Reconstructs a the shortest path given an HashMap (from points
    /// to points) result of self.dijkstra
    fn reconstruct_path(&self, prev: &HashMap<Point, Point>, dst: Point) -> Path
    {
        let mut path = Path::new();
        let mut u = dst;
        while prev.contains_key(&u) {
            path.nodes.insert(0, u);
            u = prev[&u];
        }
        return path;
    }

    /// Use Dijkstra's algorithm to calculate single-source shortest paths
    fn dijkstra(&self, src: Point) -> (HashMap<Point, i32>, HashMap<Point, Point>)
    {
        let n = (N * N) as usize;
        let mut dist = HashMap::with_capacity(n);
        let mut prev = HashMap::with_capacity(n);
        let mut nodes: HashSet<Point> = HashSet::with_capacity(n);

        for x in 0..N {
            for y in 0..N {
                dist.insert(point!(x, y), MAX_DIST);
                nodes.insert(point!(x, y));
            }
        }
        dist.insert(src, 0);

        while !nodes.is_empty() {
            let mut u = *nodes.iter().next().unwrap();
            let mut min = MAX_DIST;

            for node in &nodes {
                if dist[node] < min {
                    min = dist[node];
                    u = *node
                }
            }

            nodes.remove(&u);

            let mut neighbors = Vec::with_capacity(9);
            for dx in -2..3 {
                for dy in -2..3 {
                    neighbors.push(point!(u.x + dx, u.y + dy))
                }
            }

            for v in &neighbors {
                if dist.contains_key(v) && self.adj(u, *v) {
                    let alt = dist[&u] + 1;
                    if alt < dist[v] {
                        dist.insert(*v, alt);
                        prev.insert(*v, u);
                    }
                }
            }
        }

        return (dist, prev);
    }

    /// Use Warshall's algorithm to determine point-to-point connectedness
    fn warshall(&self) -> Vec<Vec<bool>>
    {
        let m = N + 2;
        let n = (m * m) as usize;
        let mut w = vec![vec![false; n]; n];
        for a in 0..n {
            for b in 0..n {
                w[a][b] = self.adj(
                    point!(a as i32 % m - 1, (a as i32) / m - 1),
                    point!(b as i32 % m - 1, (b as i32) / m - 1)
                )
            }
        }
        for k in 0..n {
            for b in 0..n {
                for a in 0..n {
                    w[a][b] = w[a][b] || (w[a][k] && w[k][b])
                }
            }
        }
        return w;
    }
}
