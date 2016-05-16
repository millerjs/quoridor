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

//! Quoridor board data structures and logic

use constants::N;
use errors::{QuoridorError, QuoridorResult};

/// Convert (x, y) tuple to a point
macro_rules! point { ( $x:expr, $y:expr ) => { { Point{ x: $x, y: $y} } }; }

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
    Invalid,
}


#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
}

impl<S> From<S> for Direction
    where S: Into<String>
{
    fn from(s: S) -> Direction {
        match &*s.into().to_uppercase() {
            "NORTH" => Direction::North,
            "EAST" => Direction::East,
            "SOUTH" => Direction::South,
            "WEST" => Direction::West,
            "NORTHEAST" => Direction::NorthEast,
            "SOUTHEAST" => Direction::SouthEast,
            "SOUTHWEST" => Direction::SouthWest,
            "NORTHWEST" => Direction::NorthWest,
            _ => Direction::Invalid,
        }
    }
}

impl Direction {
    pub fn reversed(&self) -> Direction
    {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
            Direction::Invalid => Direction::Invalid,
        }
    }
}

impl Point {

    /// Returns a shifted copy of the point
    pub fn shift<D>(&self, direction: D) -> Point
        where D: Into<Direction>
    {
        match direction.into() {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
            Direction::NorthEast => self.north().east(),
            Direction::SouthEast => self.south().east(),
            Direction::SouthWest => self.south().west(),
            Direction::NorthWest => self.north().west(),
            Direction::Invalid => self.clone(),
        }
    }

    /// Returns new point one space to the right
    pub fn east(&self) -> Point
    {
        Point { x: self.x + 1, y: self.y }
    }

    /// Returns new point one space to the right
    pub fn west(&self) -> Point
    {
        Point { x: self.x - 1, y: self.y }
    }

    /// Returns new point one space up
    pub fn north(&self) -> Point
    {
        Point { x: self.x, y: self.y - 1 }
    }

    /// Returns new point one space down
    pub fn south(&self) -> Point
    {
        Point { x: self.x, y: self.y + 1 }
    }

    /// Is the point on the board
    pub fn inbounds(&self) -> bool
    {
        self.x >= 0 && self.x < N && self.y >= 0 && self.y < N
    }

    /// Is this point neighbors with the other
    pub fn neighbors(&self, other: Point) -> bool
    {
        ((self.x - other.x).abs() == 0 && (self.y - other.y).abs() == 1) ||
        ((self.y - other.y).abs() == 0 && (self.x - other.x).abs() == 1)
    }
}


impl Wall {
    /// Creates new horizontal wall
    pub fn horizontal(x: i32, y: i32) -> Wall
    {
        Wall {
            orientation: Orientation::Horizontal,
            x: x,
            y: y,
        }
    }

    /// Creates new vertical wall
    pub fn vertical(x: i32, y: i32) -> Wall
    {
        Wall {
            orientation: Orientation::Vertical,
            x: x,
            y: y,
        }
    }

    /// Is wall on the board
    pub fn inbounds(&self) -> bool
    {
        self.x > 0 && self.x < N && self.y > 0 && self.y < N
    }

    /// Returns a new wall shifted +x, +y
    pub fn shifted(&self, x: i32, y: i32) -> Wall
    {
        Wall {
            x: self.x + x,
            y: self.y + y,
            ..*self
        }
    }

    /// Rotates wall around center point
    pub fn rotated(&self) -> Wall
    {
        Wall {
            orientation: match self.orientation {
                Orientation::Vertical => Orientation::Horizontal,
                Orientation::Horizontal => Orientation::Vertical,
            },
            ..*self
        }
    }

    pub fn to_tuples(&self) -> ((i32, i32), (i32, i32))
    {
        match self.orientation {
            Orientation::Vertical => ((self.x, self.y - 1), (self.x, self.y + 1)),
            Orientation::Horizontal => ((self.x - 1, self.y), (self.x + 1, self.y)),
        }
    }

    pub fn from_tuples(a: (i32, i32), b: (i32, i32)) -> QuoridorResult<Wall>
    {
        if a.1 == b.1 && (a.0 - b.0).abs() == 2 {
            Ok(Wall::horizontal((a.0 + b.0) / 2, a.1))
        } else if a.0 == b.0 && (a.1 - b.1).abs() == 2 {
            Ok(Wall::vertical(a.0, (a.1 + b.1) / 2))
        } else {
            Err(QuoridorError::InvalidWall("Wall points must distance 2 away".into()))
        }
    }

    pub fn from_points(a: Point, b: Point) -> QuoridorResult<Wall>
    {
        Wall::from_tuples((a.x, a.y), (b.x, b.y))
    }
}
