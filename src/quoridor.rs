//! Quoridor game logic
//!
//! author: Joshua Miller
//! email: jsmiller@uchicago.edu

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::cmp;

const MAX_DIST: i32 = 100000;
pub const GAME_OVER: i32 = -2;
pub const GAME_NOT_STARTED: i32 = -2;
pub const N: i32 = 9;

/// Shortcut to convert str to String
pub fn s(string: &str) -> String {
    string.to_owned()
}

/// Shorter Point constructor
pub fn _p(x: i32, y: i32) -> Point {
    Point { x: x, y: y }
}

macro_rules! p { ( $x:expr, $y:expr ) => { { Point{ x: $x, y: $y} } }; }


#[derive(Debug)]
pub struct Player {
    pub p: Point,
    pub p_last: Option<Point>,
    pub key: String,
    pub id: u8,
    pub walls: u8,
    pub name: String,
}

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}


#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq)]
pub struct Path {
    pub nodes: Vec<Point>,
}

#[derive(Hash,Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub d: Direction,
}

#[derive(Debug)]
pub struct Game {
    pub walls: HashSet<Wall>,
    pub players: HashMap<String, Player>,
    pub turn: i32,
}

#[derive(Debug)]
pub enum Turn {
    PlaceWall(Wall),
    Move(Point),
}


impl Path {
    pub fn new() -> Path {
        Path { nodes: vec![] }
    }
}


impl Point {
    /// Returns new point one space to the right
    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Returns new point one space to the right
    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Returns new point one space up
    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Returns new point one space down
    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Is the point on the board
    pub fn inbounds(&self) -> bool {
        self.x >= 0 && self.x < N && self.y >= 0 && self.y < N
    }

    /// Is this point neighbors with the other
    pub fn neighbors(&self, other: Point) -> bool {
        ((self.x - other.x).abs() == 0 && (self.y - other.y).abs() == 1) ||
        ((self.y - other.y).abs() == 0 && (self.x - other.x).abs() == 1)
    }
}


impl Wall {
    /// Creates new horizontal wall
    pub fn horizontal(x: i32, y: i32) -> Wall {
        Wall {
            d: Direction::Horizontal,
            x: x,
            y: y,
        }
    }

    /// Creates new vertical wall
    pub fn vertical(x: i32, y: i32) -> Wall {
        Wall {
            d: Direction::Vertical,
            x: x,
            y: y,
        }
    }

    /// Is wall on the board
    pub fn inbounds(&self) -> bool {
        self.x > 0 && self.x < N && self.y > 0 && self.y < N
    }

    /// Returns a new wall shifted +x, +y
    pub fn shift(&self, x: i32, y: i32) -> Wall {
        Wall {
            x: self.x + x,
            y: self.y + y,
            ..*self
        }
    }

    /// Rotates wall around center point
    pub fn rotate(&self) -> Wall {
        Wall {
            d: match self.d {
                Direction::Vertical => Direction::Horizontal,
                Direction::Horizontal => Direction::Vertical,
            },
            ..*self
        }
    }

    pub fn to_tuples(&self) -> ((i32, i32), (i32, i32)) {
        match self.d {
            Direction::Vertical => ((self.x, self.y - 1), (self.x, self.y + 1)),
            Direction::Horizontal => ((self.x - 1, self.y), (self.x + 1, self.y)),
        }
    }

    pub fn from_tuples(a: (i32, i32), b: (i32, i32)) -> Result<Wall, String> {
        if a.1 == b.1 && (a.0 - b.0).abs() == 2 {
            Ok(Wall::horizontal((a.0 + b.0) / 2, a.1))
        } else if a.0 == b.0 && (a.1 - b.1).abs() == 2 {
            Ok(Wall::vertical(a.0, (a.1 + b.1) / 2))
        } else {
            Err(s("Wall points must distance 2 away"))
        }
    }

    pub fn from_points(a: Point, b: Point) -> Result<Wall, String> {
        Wall::from_tuples((a.x, a.y), (b.x, b.y))
    }
}


impl Player {
    pub fn has_won(&self) -> bool {
        return (self.id != 1 && self.p.y < 0) || (self.id != 2 && self.p.x < 0) ||
               (self.id != 0 && self.p.y >= N) || (self.id != 3 && self.p.y >= N);
    }

    pub fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(s("position"), vec![self.p.x, self.p.y].to_json());
        d.insert(s("id"), self.id.to_json());
        d.insert(s("walls"), self.walls.to_json());
        d.insert(s("name"), self.name.to_json());
        Json::Object(d)
    }
}


impl Game {
    pub fn do_turn(&mut self, name: String, turn: Turn) {
        let _ = match turn {
            Turn::PlaceWall(wall) => self.add_wall(&wall),
            Turn::Move(p) => self.move_player_to(name, p),
        };
    }

    pub fn undo_turn(&mut self, name: String, turn: Turn) {
        match turn {
            Turn::PlaceWall(wall) => {
                self.walls.remove(&wall);
                self.walls.remove(&wall);
            }
            Turn::Move(_) => {
                if let Some(p) = self.players.get_mut(&name) {
                    match p.p_last {
                        Some(pos) => p.p = pos,
                        None => debug!("no last move"),
                    }
                }
            }
        };
    }

    /// Check to see if all players have at least 1 possible path to
    /// their endzone
    pub fn check_win_condition(&self, name: String) -> bool {
        let p = &self.players[&name];
        let (d, _) = self.dijkstra(p.p);
        match p.id {
            0 => (0..N).fold(false, |v, i| v || d[&p!(i, N - 1)] < MAX_DIST),
            1 => (0..N).fold(false, |v, i| v || d[&p!(i, 0)] < MAX_DIST),
            _ => false,
        }
    }

    pub fn dijkstra(&self, src: Point) -> (HashMap<Point, i32>, HashMap<Point, Point>) {
        let n = (N * N) as usize;
        let mut dist = HashMap::with_capacity(n);
        let mut prev = HashMap::with_capacity(n);
        let mut nodes: HashSet<Point> = HashSet::with_capacity(n);

        for x in 0..N {
            for y in 0..N {
                dist.insert(p!(x, y), MAX_DIST);
                nodes.insert(p!(x, y));
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
                    neighbors.push(p!(u.x + dx, u.y + dy))
                }
            }

            for v in &neighbors {
                if dist.contains_key(v) && self.adj(u, *v).is_ok() {
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

    pub fn reconstruct_path(&self, prev: &HashMap<Point, Point>, dst: Point) -> Path {
        let mut path = Path::new();
        let mut u = dst;
        while prev.contains_key(&u) {
            path.nodes.insert(0, u);
            u = prev[&u];
        }
        return path;
    }

    pub fn warshall(&self) -> Vec<Vec<bool>> {
        let m = N + 2;
        let n = (m * m) as usize;
        let mut w = vec![vec![false; n]; n];
        for a in 0..n {
            for b in 0..n {
                w[a][b] = self.adj(p!(a as i32 % m - 1, (a as i32) / m - 1),
                                   p!(b as i32 % m - 1, (b as i32) / m - 1))
                              .is_ok()
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

    pub fn adj(&self, a: Point, b: Point) -> Result<(), String> {
        if !a.inbounds() {
            return Err(format!("Point {:?} is not in bounds", a));
        }

        if !b.inbounds() {
            return Err(format!("Point {:?} is not in bounds", b));
        }

        if a == b {
            return Err(format!("{:?} and {:?} are the same point", a, b));
        }

        if !a.neighbors(b) {
            return self.is_valid_jump(a, b);
        }

        if self.get_player_at_position(b).is_ok() {
            return Err(format!("Position {:?} is not empty.", b));
        }

        if self.has_wall_between(a, b) {
            return Err(format!("There is a wall between {:?} and {:?}.", a, b));
        }

        return Ok(());
    }

    pub fn is_valid_jump(&self, a: Point, b: Point) -> Result<(), String> {
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let horizontal_jump = dy.abs() == 2 && dx == 0;
        let vertical_jump = dy == 0 && dx.abs() == 2;
        let linear_jump = horizontal_jump || vertical_jump;
        let corner_jump = dx.abs() == 1 && dy.abs() == 1;

        if linear_jump {
            let them_pos = p!(b.x - dx / 2, b.y - dy / 2);
            let occupied = self.get_player_at_position(them_pos).is_ok();
            let step1 = !self.has_wall_between(a, them_pos);
            let step2 = !self.has_wall_between(them_pos, b);

            if step1 && occupied && step2 {
                return Ok(());
            } else {
                return Err(format!("Invalid linear jump."));
            }

        } else if corner_jump {

            let mut path1 = self.get_player_at_position(p!(b.x, a.y)).is_ok();
            let back_wall = self.has_wall_between(p!(b.x, a.y), p!(b.x + dx, a.y));
            let step1 = !self.has_wall_between(a, p!(a.x + dx, a.y));
            let step2 = !self.has_wall_between(p!(a.x + dx, a.y), b);

            if path1 && back_wall && step1 && step2 {
                return Ok(());
            }

            let mut path2 = self.get_player_at_position(p!(a.x, b.y)).is_ok();
            let back_wall = self.has_wall_between(p!(a.x, b.y), p!(a.x, b.y + dy));
            let step1 = !self.has_wall_between(a, p!(a.x, a.y + dy));
            let step2 = !self.has_wall_between(p!(a.x, a.y + dy), b);

            if path2 && back_wall && step1 && step2 {
                return Ok(());
            } else {
                return Err(format!("Invalid corner jump."));
            }

        }

        return Err(format!("This doesn't even look like a jump."));
    }

    pub fn has_wall_between(&self, a: Point, b: Point) -> bool {
        if !a.neighbors(b) {
            return false;
        }

        if a.y == b.y {
            // Look for vertical wall
            let test_wall = Wall::vertical(cmp::max(a.x, b.x), a.y);
            return self.walls.contains(&test_wall) || self.walls.contains(&test_wall.shift(0, 1));
        } else if a.x == b.x {
            // Look for horizontal wall
            let test_wall = Wall::horizontal(a.x, cmp::max(a.y, b.y));
            return self.walls.contains(&test_wall) || self.walls.contains(&test_wall.shift(1, 0));
        } else {
            return false;
        }
    }

    /// Creates a new game of size `size x size`
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            walls: HashSet::new(),
            turn: GAME_NOT_STARTED,
        }
    }

    pub fn move_player_to(&mut self, name: String, pos: Point) -> Result<String, String> {
        if !self.players.contains_key(&name) {
            return Err(s("Player not found."));
        }

        let start = self.players[&name].p;
        match self.adj(start, pos) {
            Ok(_) => {
                if let Some(p) = self.players.get_mut(&name) {
                    p.p_last = Some(p.p);
                    p.p = pos;
                }
                return Ok(format!("Moved player to {:?}", &self.players[&name].p));
            }
            Err(msg) => return Err(msg),
        }
    }

    /// Moves a player a direction (one of UP, DOWN, LEFT, RIGHT)
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.
    /// * `dir`  - A string specifying which direction to move
    pub fn move_player(&mut self, name: String, dir: String) -> Result<String, String> {
        let p = self.players[&name].p;
        self.move_player_to(name,
                            match &*dir.to_uppercase() {
                                "UP" => p.up(),
                                "DOWN" => p.down(),
                                "LEFT" => p.left(),
                                "RIGHT" => p.right(),
                                _ => return Err(s("Unknown direction")),
                            })
    }

    /// Check if there is a player at position (x, y)
    pub fn get_player_at_position(&self, p: Point) -> Result<String, String> {
        for (name, player) in self.players.iter() {
            if player.p == p {
                return Ok(name.clone());
            }
        }
        Err(s("No player found"))
    }

    /// Print ASCII representation to stdout
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn is_valid_wall(&mut self, wall: &Wall) -> bool {
        if !wall.inbounds() {
            debug!("Out of bounds");
            return false;
        }

        if self.walls.contains(wall) {
            debug!("Wall exists");
            return false;
        }

        if wall.d == Direction::Vertical {
            // Check vertical collisions
            if self.walls.contains(&wall.rotate()) || self.walls.contains(&wall.shift(0, -1)) ||
               self.walls.contains(&wall.shift(0, 1)) {
                debug!("Vertical wall collision");
                return false;
            }
        } else {
            // Check horizontal collisions
            if self.walls.contains(&wall.rotate()) || self.walls.contains(&wall.shift(-1, 0)) ||
               self.walls.contains(&wall.shift(1, 0)) {
                debug!("Horizontal wall collision");
                return false;
            }
        }

        // Check each player has a path
        self.walls.insert(*wall);
        let win_conditions = self.players
                                 .keys()
                                 .fold(true, |v, i| v && self.check_win_condition(i.clone()));
        self.walls.remove(wall);

        if !win_conditions {
            debug!("Wall eliminates path");
            return false;
        }

        return true;

    }


    /// Place a wall from intersection a->b
    pub fn add_wall(&mut self, wall: &Wall) -> Result<String, String> {
        match self.is_valid_wall(wall) {
            true => {
                self.walls.insert(*wall);
                Ok(s("Added wall."))
            }
            false => Err(s("Invalid wall.")),
        }
    }

    pub fn add_wall_tuples(&mut self, a: (i32, i32), b: (i32, i32)) -> Result<String, String> {
        match Wall::from_tuples(a, b) {
            Ok(w) => self.add_wall(&w),
            Err(w) => Err(format!("Invalid wall {:?}", w)),
        }
    }


    /// Starts the game, assigns wall chips, sets the turn
    pub fn start_game(&mut self) {
        self.turn = 0;
        assert!(self.players.len() == 4 || self.players.len() == 2);
        if self.players.len() == 4 {
            for (_, p) in self.players.iter_mut() {
                p.walls = 5
            }
        }
        if self.players.len() == 2 {
            for (_, p) in self.players.iter_mut() {
                p.walls = 10
            }
        }
    }

    /// Increment the turn counter
    pub fn next_turn(&mut self) {
        // Check if someone won
        for (_, p) in self.players.iter() {
            if p.has_won() {
                self.turn == GAME_OVER;
            }
        }

        self.turn = (self.turn + 1) % (self.players.len() as i32)
    }

    /// Adds a player given a name, and a password `key`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.  Use this later when moving
    ///            the player
    /// * `key` - The password for the player.  Use this later when moving
    ///           the player.   It is used to prevent unauthorized moves.
    pub fn add_player(&mut self, name: String, key: String) -> Result<String, String> {
        if self.turn >= 0 {
            return Err(s("Game already started"));
        } else if self.players.contains_key(&name) {
            return Err(format!("Player {} already registered.", name.clone()));
        } else if self.players.len() >= 2 {
            return Err(format!("Attempt to register 3rd player."));
        }

        // Create and add the player
        let i = self.players.len();
        let starting_positions = [p!(N / 2, 0), p!(N / 2, N - 1), p!(0, N / 2), p!(N - 1, N / 2)];

        let player = Player {
            p: starting_positions[i],
            p_last: None,
            key: key,
            id: i as u8,
            walls: 0,
            name: name.clone(),
        };

        self.players.insert(name.clone(), player);

        // If we have enough players, start the game
        if self.players.len() == 2 {
            self.start_game()
        }

        return Ok(format!("Added player {}", name));
    }

    /// Construct ASCII representation
    pub fn to_string(&self) -> String {
        let mut board = "".to_string();
        let x = "+";
        board.push_str("  ");
        for i in 0..N {
            board.push_str(&*format!("{:4}", i))
        }
        board.push_str("\n");

        // Vertical iteration
        for j in 0..N {
            board.push_str("   ");
            for i in 0..N {
                if !self.has_wall_between(p!(i, j), p!(i, j - 1)) {
                    board.push_str(&*format!("{}   ", x))
                } else {
                    board.push_str(&*format!("{} - ", x))
                }
            }
            board.push_str(&*format!("+\n{:2} ", j));

            // Horizontal iteration
            for i in 0..N {
                let n = match self.get_player_at_position(p!(i, j)) {
                    Ok(name) => format!("{}", self.players[&name].id),
                    Err(_) => s(" "),
                };
                if !self.has_wall_between(p!(i, j), p!(i - 1, j)) {
                    board.push_str(&*format!("  {} ", n))
                } else {
                    board.push_str(&*format!("| {} ", n))
                }
            }
            board.push_str("\n");
        }
        board.push_str("   ");
        for _ in 0..N {
            board.push_str("+   ")
        }
        board.push_str("+\n");
        board
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
        d.insert(s("turn"), self.turn.to_json());
        d.insert(s("size"), N.to_json());
        d.insert(s("walls"), walls.to_json());
        let mut players: Vec<Json> = vec![];
        for (_, p) in self.players.iter() {
            players.push(p.to_json())
        }
        d.insert(s("players"), players.to_json());
        Json::Object(d)
    }

    /// Return game from JSON
    pub fn from_json(doc: String) -> Game {
        debug!("game from string: {:?}", doc);
        let data = Json::from_str(&*doc).unwrap();
        let mut game = Game::new();
        game.turn = data["turn"].as_u64().unwrap() as i32;
        for player in data["players"].as_array().unwrap() {
            println!("player: {:?}", player);
            let name = player["name"].as_string().unwrap().to_string();
            let player = Player {
                p: p!(player["position"][0].as_u64().unwrap() as i32,
                      player["position"][1].as_u64().unwrap() as i32),
                id: player["id"].as_u64().unwrap() as u8,
                p_last: None,
                key: "".to_string(),
                walls: player["walls"].as_u64().unwrap() as u8,
                name: name.clone(),
            };
            game.players.insert(name.clone(), player);
        }
        return game;
    }
}
