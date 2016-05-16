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

//! Quoridor game logic

use adjacency_matrix::AdjacencyMatrix;
use board::{Point, Wall, Orientation, Direction};
use constants::{N, MAX_DIST};
use errors::{QuoridorError, QuoridorResult};
use player::Player;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::cmp;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt;

#[derive(Debug)]
pub enum GameState {
    Setup,
    GameOver,
    Started(u8),
}

#[derive(Debug)]
pub struct Game {
    pub walls: HashSet<Wall>,
    pub players: HashMap<String, Player>,
    pub state: GameState,
}

impl AdjacencyMatrix for Game {
    fn adj(&self, a: Point, b: Point) -> bool {
        a.neighbors(b)
            && a.inbounds()
            && b.inbounds()
            && self.get_player_at_position(b).is_err()
            && !self.has_wall_between(a, b)
    }
}

impl Game {
    /// Create a default game
    pub fn new() -> Game
    {
        Game {
            players: HashMap::new(),
            walls: HashSet::new(),
            state: GameState::Setup,
        }
    }

    /// Check to see if all players have at least 1 possible path to
    /// their endzone
    pub fn check_win_condition(&self, player: &Player) -> bool
    {
        let (d, _) = self.dijkstra(player.p);
        match player.id {
            0 => (0..N).fold(false, |v, i| v || d[&point!(i, N - 1)] < MAX_DIST),
            1 => (0..N).fold(false, |v, i| v || d[&point!(i, 0)] < MAX_DIST),
            _ => false,
        }
    }

    pub fn is_valid_jump(&self, a: Point, b: Point) -> QuoridorResult<()>
    {
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let horizontal_jump = dy.abs() == 2 && dx == 0;
        let vertical_jump = dy == 0 && dx.abs() == 2;
        let linear_jump = horizontal_jump || vertical_jump;
        let corner_jump = dx.abs() == 1 && dy.abs() == 1;

        if linear_jump {
            let them_pos = point!(b.x - dx / 2, b.y - dy / 2);
            let occupied = self.get_player_at_position(them_pos).is_ok();
            let step1 = !self.has_wall_between(a, them_pos);
            let step2 = !self.has_wall_between(them_pos, b);

            return match step1 && occupied && step2 {
                true => Ok(()),
                false => Err(QuoridorError::InvalidJump("Invalid linear jump.".into())),
            }

        } else if corner_jump {
            let path1 = self.get_player_at_position(point!(b.x, a.y)).is_ok();
            let back_wall = self.has_wall_between(point!(b.x, a.y), point!(b.x + dx, a.y));
            let step1 = !self.has_wall_between(a, point!(a.x + dx, a.y));
            let step2 = !self.has_wall_between(point!(a.x + dx, a.y), b);

            if path1 && back_wall && step1 && step2 {
                return Ok(());
            }

            let path2 = self.get_player_at_position(point!(a.x, b.y)).is_ok();
            let back_wall = self.has_wall_between(point!(a.x, b.y), point!(a.x, b.y + dy));
            let step1 = !self.has_wall_between(a, point!(a.x, a.y + dy));
            let step2 = !self.has_wall_between(point!(a.x, a.y + dy), b);

            return match path2 && back_wall && step1 && step2 {
                true => Ok(()),
                false => Err(QuoridorError::InvalidJump("Invalid corner jump.".into())),
            }
        }

        return Err(QuoridorError::InvalidJump("This doesn't even look like a jump.".into()));
    }

    pub fn has_wall_between(&self, a: Point, b: Point) -> bool
    {
        if !a.neighbors(b) {
            return false;
        }

        if a.y == b.y {
            // vertical
            let test_wall = Wall::vertical(cmp::max(a.x, b.x), a.y);
            self.walls.contains(&test_wall) || self.walls.contains(&test_wall.shifted(0, 1))

        } else if a.x == b.x {
            // horizontal
            let test_wall = Wall::horizontal(a.x, cmp::max(a.y, b.y));
            self.walls.contains(&test_wall) || self.walls.contains(&test_wall.shifted(1, 0))

        } else {
            false
        }
    }

    pub fn move_player_to<S>(&mut self, name: S, pos: Point) -> QuoridorResult<String>
        where S: Into<String>
    {
        let name = name.into();

        if !self.players.contains_key(&name) {
            return Err(QuoridorError::PlayerNotFound)
        }

        let start = self.players[&name].p;
        let _ = try!(self.describe_adj(start, pos));

        if let Some(p) = self.players.get_mut(&name) {
            p.p = pos;
        }
        return Ok(format!("Moved player to {:?}", &self.players[&name].p));
    }

    pub fn is_valid_wall(&mut self, wall: &Wall) -> QuoridorResult<()>
    {
        if !wall.inbounds() {
            return Err(QuoridorError::InvalidWall("Out of bounds".into()))
        }

        if self.walls.contains(wall) {
            return Err(QuoridorError::InvalidWall("Wall exists".into()))
        }

        // Check collisions
        match wall.orientation {
            Orientation::Vertical => {
                if self.walls.contains(&wall.rotated()) || self.walls.contains(&wall.shifted(0, -1)) ||
                    self.walls.contains(&wall.shifted(0, 1)) {
                        return Err(QuoridorError::InvalidWall("Vertical wall collision".into()))
                    }
            }
            Orientation::Horizontal => {
                if self.walls.contains(&wall.rotated()) || self.walls.contains(&wall.shifted(-1, 0)) ||
                    self.walls.contains(&wall.shifted(1, 0)) {
                        return Err(QuoridorError::InvalidWall("Horizontal wall collision".into()))
                    }
            }
        }

        // Check each player has a path
        self.walls.insert(*wall);
        let win_conditions = self.players.values().fold(true, |v, player| v && self.check_win_condition(player));
        self.walls.remove(wall);

        if !win_conditions {
            return Err(QuoridorError::InvalidWall("Wall eliminates all paths".into()))
        }

        return Ok(());
    }

    /// Moves a player a direction
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.
    /// * `direction`  - A String or Direction specifying which orientation to move
    pub fn move_player<S, D>(&mut self, name: S, direction: D) -> QuoridorResult<String>
        where S: Into<String> + Clone, D: Into<Direction>
    {
        let pos = self.get_player(&name).p;
        self.move_player_to(name, pos.shift(direction))
    }

    pub fn get_player<S>(&self, name: &S) -> &Player
        where S: Into<String> + Clone
    {
        &self.players[&name.clone().into()]
    }

    /// Check if there is a player at position (x, y)
    pub fn get_player_at_position(&self, p: Point) -> QuoridorResult<String>
    {
        for (name, player) in self.players.iter() {
            if player.p == p {
                return Ok(name.clone());
            }
        }
        Err(QuoridorError::PlayerNotFound)
    }

    /// Place a wall from intersection a->b
    pub fn add_wall(&mut self, wall: &Wall) -> QuoridorResult<String>
    {
        let _ = try!(self.is_valid_wall(wall));
        self.walls.insert(*wall);
        Ok("Added wall.".into())
    }

    pub fn add_wall_tuples(&mut self, a: (i32, i32), b: (i32, i32)) -> QuoridorResult<String>
    {
        let wall = try!(Wall::from_tuples(a, b));
        self.add_wall(&wall)
    }

    /// Starts the game, assigns wall chips, sets the turn
    pub fn start_game(&mut self) {
        let turn = 0;
        self.state = GameState::Started(turn);
        if self.players.len() == 4 {
            for (_, p) in self.players.iter_mut() {
                p.walls = 5
            }
        }
    }

    /// Increment the turn counter
    pub fn increment_turn(&mut self) -> QuoridorResult<()> {
        self.state = match self.state {
            GameState::Started(turn) => {
                if self.winner().is_some() {
                    GameState::GameOver
                } else {
                    GameState::Started((turn + 1) % (self.players.len() as u8))
                }
            },
            GameState::Setup => return Err(QuoridorError::TurnError("Game not started".into())),
            GameState::GameOver => return Err(QuoridorError::TurnError("Game is over".into())),
        };
        Ok(())
    }

    pub fn winner(&self) -> Option<u8>
    {
        for (_, p) in self.players.iter() {
            if p.has_won() {
                return Some(p.id)
            }
        }
        None
    }

    /// Adds a player given a name, and a password `key`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.  Use this later when moving
    ///            the player
    /// * `key` - The password for the player.  Use this later when moving
    ///           the player.   It is used to prevent unauthorized moves.
    pub fn add_player<S>(&mut self, name: S, key: S) -> QuoridorResult<String>
        where S: Into<String>
    {
        let name = name.into();
        let key = key.into();

        match self.state {
            GameState::Started(_) => return Err(QuoridorError::TurnError("Game already started".into())),
            _ => (),
        }

        if self.players.contains_key(&name.clone()) {
            return Err(QuoridorError::RegistrationError(format!("Player {} already registered.", name.clone())))
        } else if self.players.len() >= 2 {
            return Err(QuoridorError::RegistrationError(format!("Attempt to register 3rd player.")))
        }

        let starting_positions = [
            point!(N / 2, 0),
            point!(N / 2, N - 1),
            point!(0, N / 2),
            point!(N - 1, N / 2)
        ];

        // Create and add the player
        let i = self.players.len();
        let player = Player {
            p: starting_positions[i],
            key: key,
            id: i as u8,
            walls: 10,
            name: name.clone(),
        };

        self.players.insert(name.clone(), player);

        // If we have enough players, start the game
        if self.players.len() == 2 {
            self.start_game()
        }

        return Ok(format!("Added player {}", name));
    }

    /// Return a Result specifying whether two points are or why they
    /// aren't adjacent
    fn describe_adj(&self, a: Point, b: Point) -> QuoridorResult<()> {
        if !a.inbounds() {
            return Err(QuoridorError::InvalidMove(format!("Point {:?} is not in bounds", a)))
        }

        if !b.inbounds() {
            return Err(QuoridorError::InvalidMove(format!("Point {:?} is not in bounds", b)))
        }

        if a == b {
            return Err(QuoridorError::InvalidMove(format!("{:?} and {:?} are the same point", a, b)))
        }

        if !a.neighbors(b) {
            try!(self.is_valid_jump(a, b))
        }

        if self.get_player_at_position(b).is_ok() {
            return Err(QuoridorError::InvalidMove(format!("Position {:?} is not empty.", b)));
        }

        if self.has_wall_between(a, b) {
            return Err(QuoridorError::InvalidMove(format!("There is a wall between {:?} and {:?}.", a, b)));
        }

        Ok(())
    }

    /// Return the game state as JSON
    pub fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let mut walls: Vec<Vec<Vec<i32>>> = vec![];
        for wall in self.walls.iter() {
            let w = wall.to_tuples();
            let (a, b) = (w.0, w.1);
            walls.push(vec![vec![a.0, a.1], vec![b.0, b.1]])
        }
        // d.insert("turn".into(), self.turn.to_json());
        d.insert("size".into(), N.to_json());
        d.insert("walls".into(), walls.to_json());
        let mut players: Vec<Json> = vec![];
        for (_, p) in self.players.iter() {
            players.push(p.to_json())
        }
        d.insert("players".into(), players.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Game {
    /// Construct ASCII representation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = "+";
        try!(write!(f, "  "));
        for i in 0..N {
            try!(write!(f, "{:4}", i));
        }
        try!(write!(f, "\n"));

        // Vertical iteration
        for j in 0..N {
            try!(write!(f, "   "));
            for i in 0..N {
                if !self.has_wall_between(point!(i, j), point!(i, j - 1)) {
                    try!(write!(f, "{}   ", x));
                } else {
                    try!(write!(f, "{} - ", x));
                }
            }
            try!(write!(f, "+\n{:2} ", j));

            // Horizontal iteration
            for i in 0..N {
                let n = match self.get_player_at_position(point!(i, j)) {
                    Ok(name) => format!("{}", self.players[&name].id),
                    Err(_) => " ".into(),
                };
                if !self.has_wall_between(point!(i, j), point!(i - 1, j)) {
                    try!(write!(f, "  {} ", n));
                } else {
                    try!(write!(f, "| {} ", n));
                }
            }
            try!(write!(f, "\n"));
        }
        try!(write!(f, "   "));
        for _ in 0..N {
            try!(write!(f, "+   "));
        }
        write!(f, "+\n")
    }
}
