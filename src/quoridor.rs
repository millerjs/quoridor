/***********************************************************************
 * Quoridor game logic
 *
 * author: Joshua Miller
 * email: jshuasmiller@gmail.com
 *
 ***********************************************************************/

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::borrow::ToOwned;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp;

const MAX_DIST: i32 = 100000;
pub const GAME_OVER: i32 = -2;
pub const GAME_NOT_STARTED: i32 = -2;
pub const N: i32 = 9;

pub fn s(string: &str) -> String {
    string.to_owned()
}


/***********************************************************************
 * Game Structs
 ***********************************************************************/

#[derive(Debug)]
pub struct Player {
    pub p: (i32, i32),
    pub p_last: Option<(i32, i32)>,
    pub key: String,
    pub id: u8,
    pub walls: u8,
    pub name: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
pub struct Wall {
    x: i32,
    y: i32,
    d: Direction,
}

pub struct Point {
    x: i32,
    y: i32,
}

trait OnBoard {
    fn inbounds(&self) -> bool;
}


#[derive(Debug)]
pub struct Game {
    pub walls: BTreeSet<Wall>,
    pub players: BTreeMap<String, Player>,
    pub turn: i32,
}

#[derive(Debug)]
pub enum Turn {
    PlaceWall(Wall),
    Move(i32, i32),
}


impl OnBoard for Point {
    fn inbounds(&self) -> bool {
        self.x >= 0 && self.x < N && self.y >= 0 && self.y < N
    }
}

impl OnBoard for Wall {
    fn inbounds(&self) -> bool {
        self.x > 0 && self.x < N && self.y > 0 && self.y < N
    }
}

impl Wall {
    pub fn to_tuples(&self) -> ((i32, i32), (i32, i32)) {
        match self.d {
            Direction::Vertical => {
                ((self.x, self.y-1), (self.x, self.y+1))
            }
            Direction::Horizontal => {
                ((self.x-1, self.y), (self.x+1, self.y))
            }
        }
    }
    pub fn from_tuples(a: (i32, i32), b: (i32, i32)) -> Result<Wall, String> {
        if a.1 == b.1 && (a.0 - b.0).abs() == 2 {
            Ok(Wall { x: (a.0+b.0)/2, y: a.1, d: Direction::Horizontal})
        } else if a.0 == b.0 && (a.1 - b.1).abs() == 2 {
            Ok(Wall { x: a.0, y: (a.1+b.1)/2, d: Direction::Vertical})
        } else {
            Err(s("Wall points must distance 2 away"))
        }
    }
    pub fn from_points(a: Point, b: Point) -> Result<Wall, String> {
        Wall::from_tuples((a.x, a.y), (b.x, b.y))
    }

}

/***********************************************************************
 * Player implementations
 ***********************************************************************/

impl Player {
    pub fn has_won(&self) -> bool {
        return (self.id != 1 && self.p.1 < 0)
            || (self.id != 2 && self.p.0 < 0)
            || (self.id != 0 && self.p.1 >= N)
            || (self.id != 3 && self.p.1 >= N)
    }

    pub fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(s("position"), vec![self.p.0, self.p.1].to_json());
        d.insert(s("id"), self.id.to_json());
        d.insert(s("walls"), self.walls.to_json());
        d.insert(s("name"), self.name.to_json());
        Json::Object(d)
    }
}

/***********************************************************************
 * Game implementations
 ***********************************************************************/

#[allow(dead_code)]
impl Game {

    /// Creates a new game of size `size x size`
    pub fn new() -> Game
    {
        Game {
            players: BTreeMap::new(),
            walls:  BTreeSet::new(),
            turn: GAME_NOT_STARTED,
        }
    }

    /// Starts the game, assigns wall chips, sets the turn
    pub fn start_game(&mut self)
    {
        self.turn = 0;
        assert!(self.players.len() == 4 || self.players.len() == 2);
        if self.players.len() == 4 {
            for (_, p) in self.players.iter_mut() { p.walls = 5 }
        }
        if self.players.len() == 2 {
            for (_, p) in self.players.iter_mut() { p.walls = 10 }
        }
    }

    /// Increment the turn counter
    pub fn next_turn(&mut self)
    {
        // Check if someone won
        for (_, p) in self.players.iter() {
            if p.has_won() { self.turn == GAME_OVER; }
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
    pub fn add_player(&mut self, name: String, key: String)
                      -> Result<String, String>
    {
        if self.turn >= 0 {
            return Err(s("Game already started"));
        } else if self.players.contains_key(&name) {
            return Err(format!("Player {} already registered.", name.clone()))
        } else if self.players.len() >= 2 {
            return Err(format!("Attempt to register 3rd player."))
        }

        // Create and add the player
        let i = self.players.len();
        let starting_positions = [(N/2, 0), (N/2, N-1), (0, N/2), (N-1, N/2)];
        self.players.insert(name.clone(), Player {
            p: starting_positions[i],
            p_last: None,
            key: key,
            id: i as u8,
            walls: 0,
            name: name.clone(),
        });

        // If we have enough players, start the game
        if self.players.len() == 2 { self.start_game() }

        return Ok(format!("Added player {}", name))
    }

    pub fn do_turn(&mut self, name: String, turn: Turn) {
        match turn {
            Turn::PlaceWall(wall) => self.add_wall(&wall),
            Turn::Move(x, y) => self.move_player_to(name, (x, y))
        };
    }

    pub fn undo_turn(&mut self, name: String, turn: Turn) {
        match turn {
            Turn::PlaceWall(wall) => {
                self.walls.remove(&wall);
                self.walls.remove(&wall);
            }
            Turn::Move(x, y) => {
                if let Some(p) = self.players.get_mut(&name) {
                    match p.p_last {
                        Some(pos) => p.p = pos,
                        None => debug!("no last move"),
                    }
                }
            }
        };
    }

    pub fn is_valid_move(&mut self, name: String, pos: (i32, i32)) -> bool
    {
        if !self.players.contains_key(&name) {
            debug!("Player not found.");
            return false
        } else {
            let p = &self.players[&name];

            // Boundary checks
            if (pos.1 < 0 && p.id != 1) || (pos.1 >= N && p.id != 0) ||
                (pos.0 < 0 && p.id != 2) || (pos.0 >= N && p.id != 3){
                    debug!("Attempted to move out of bounds");
                    return false
                }

            // Check position
            if self.get_player_at_position(pos).is_ok() {
                debug!("Position is not empty");
                return false
            }

            // Check for jumps
            if !self.adj(p.p, pos) {
                let (dx, dy) = (pos.0 - p.p.0, pos.1 - p.p.1);

                if (dy.abs() == 2 && dx == 0) || (dy == 0 && dx.abs() == 2) {
                    // Linear jumps
                    if self.get_player_at_position((pos.0-dx/2, pos.1-dy/2)).is_err()
                        || !self.adj((pos.0-dx/2, pos.1-dy/2), (pos.0, pos.1))
                        || !self.adj((pos.0-dx/2, pos.1-dy/2), (pos.0-dx, pos.1-dy)) {
                            debug!("Invalid linear jump for {} from {:?} to {:?}",
                                   name, p.p, pos);
                            return false
                    }

                } else if dx.abs() == 1 && dy.abs() == 1 {
                    // Corner Jumps
                    if !((!self.adj((pos.0, p.p.1), (pos.0+dx, p.p.1))
                          && self.adj((p.p.0, p.p.1), (pos.0, p.p.1))
                          && self.adj((pos.0, pos.1), (pos.0, p.p.1)))
                         || (!self.adj((p.p.0, pos.1), (p.p.0, pos.1+dy))
                             && self.adj((p.p.0, p.p.1), (p.p.0, pos.1))
                             && self.adj((pos.0, pos.1), (p.p.0, pos.1)))) {
                        debug!("Invalid corner jump for {} from {:?} to {:?}",
                               name, p.p, pos);
                        return false
                    }

                } else {
                    debug!("Cannot move player {} to {:?}", name, pos);
                    return false
                }
            }
        }
        // If we made it here, then the move is valid.
        return true;
    }

    pub fn move_player_to(&mut self, name: String, pos: (i32, i32))
                          -> Result<String, String>
    {
        if self.is_valid_move(name.clone(), pos) {
            if let Some(p) = self.players.get_mut(&name) {
                p.p_last = Some(p.p);
                p.p = pos;
            }
            return Ok(format!("Moved player to {:?}", &self.players[&name].p));
        }
        return Err(s("Invalid move"))
    }

    /// Moves a player a direction (one of UP, DOWN, LEFT, RIGHT)
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.
    /// * `dir`  - A string specifying which direction to move
    pub fn move_player(&mut self, name: String, dir: String)
                       -> Result<String, String>
    {
        let p = self.players[&name].p;
        self.move_player_to(name, match &*dir {
            "UP" => (p.0, p.1-1),
            "DOWN" => (p.0, p.1+1),
            "LEFT" => (p.0-1, p.1),
            "RIGHT" => (p.0+1, p.1),
            _ => return Err(s("Unknown direction"))
        })
    }

    /// Check if there is a player at position (x, y)
    pub fn get_player_at_position(&self, p: (i32, i32))
                                  -> Result<String, String>
    {
        for (name, player) in self.players.iter() {
            if player.p.0 == p.0 as i32 && player.p.1 == p.1 as i32 {
                return Ok(name.clone());
            }
        }
        Err(s("No player found"))
    }

    /// Print ASCII representation to stdout
    pub fn print(&self)
    {
        println!("{}", self.to_string());
    }

    /// Construct ASCII representation
    pub fn to_string(&self) -> String
    {
        let mut board = "".to_string();
        let x = "+";
        board.push_str("  ");
        for i in 0..N {
            board.push_str(&*format!("{:4}", i)) };
        board.push_str("\n");

        // Vertical iteration
        for j in 0..N  {
            board.push_str("   ");
            for i in 0..N  {
                if self.adj((i, j), (i, j-1)){
                    board.push_str(&*format!("{}   ", x)) }
                else { board.push_str(&*format!("{} - ", x)) }
            }
            board.push_str(&*format!("+\n{:2} ", j));

            // Horizontal iteration
            for i in 0..N {
                let n = match self.get_player_at_position((i, j)) {
                    Ok(name) => format!("{}", self.players[&name].id),
                    Err(_) => s(" ")
                };
                if self.adj((i, j), (i-1, j)){
                    board.push_str(&*format!("  {} ", n)) }
                else { board.push_str(&*format!("| {} ", n)) }
            }

            board.push_str(" \n");
        }
        board.push_str("   ");
        for _ in 0..N { board.push_str("+   ") }
        board.push_str("+\n");
        board
    }

    pub fn is_valid_wall(&mut self, wall: &Wall) -> Result<String, String>
    {
        if !wall.inbounds() {
            return Err(s("Out of bounds"))
        }

        if self.walls.contains(wall) {
            return Err(s("Wall already exists"))
        }

        if wall.d == Direction::Vertical {
            let rot = Wall { d: Direction::Horizontal, x: wall.x, y: wall.y };
            let shift_down = Wall { d: wall.d, x: wall.x, y: wall.y-1};
            let shift_up  =  Wall { d: wall.d, x: wall.x, y: wall.y+1};
            if self.walls.contains(&rot)
                || self.walls.contains(&shift_up)
                || self.walls.contains(&shift_down) {
                    return Err(s("Wall collision"))
            }
        } else {
            let rot = Wall { d: Direction::Vertical, x: wall.x, y: wall.y };
            let shift_left  = Wall { d: wall.d, x: wall.x-1, y: wall.y};
            let shift_right = Wall { d: wall.d, x: wall.x+1, y: wall.y};
            if self.walls.contains(&rot)
                || self.walls.contains(&shift_left)
                || self.walls.contains(&shift_right) {
                    return Err(s("Wall collision"))
            }
        }

        self.walls.insert(*wall);
        let win_conditions = self.players.keys().fold(
            true, |v, i| v && self.check_win_condition(i.clone()));
        self.walls.remove(wall);

        if !win_conditions {
            return Err(s("Wall eliminates path"));
        }

        return Ok(s("Valid wall"))

    }


    /// Place a wall from intersection a->b
    ///
    /// # Note: wall b->a will also be stored
    pub fn add_wall(&mut self, wall: &Wall) -> Result<String, String>
    {
        match self.is_valid_wall(wall) {
            Ok(r) => {self.walls.insert(*wall); Ok(r)},
            Err(r) => Err(r)
        }
    }

    pub fn add_wall_tuples(&mut self, a: (i32, i32), b: (i32, i32))
                           -> Result<String, String>
    {
        match Wall::from_tuples(a, b) {
            Ok(w) => self.add_wall(&w),
            Err(w) => Err(format!("Invalid wall {:?}", w))
        }
    }


    /// Check to see if all players have at least 1 possible path to
    /// their endzone
    pub fn check_win_condition(&self, name: String) -> bool
    {
        let p = &self.players[&name];
        let d = self.dijkstra(p.p);
        let m = N;
        match p.id {
            0 => (1..N).fold(
                false, |v, i| v || (d[(i+1+(N+1)*m) as usize]) < MAX_DIST),
            1 => (1..N).fold(
                false, |v, i| v || (d[((i+1)*m) as usize]) < MAX_DIST),
            2 => (1..N).fold(
                false, |v, i| v || (d[(N+1+(i+1)*m) as usize]) < MAX_DIST),
            3 => (1..N).fold(
                false, |v, i| v || (d[((i+1)*m) as usize]) < MAX_DIST),
            _ => false,
        }
    }

    /// Calculate the length of the shorted path from given source point to
    /// all other points points on the board. This is O(n^2).
    pub fn dijkstra(&self, src: (i32, i32)) -> Vec<i32>
    {
        let m = N + 2;
        let n = (m*m) as usize;
        let mut dist = vec![MAX_DIST; n];
        let mut spt_set = vec![false; n];
        dist[(src.0+1+m*(src.1+1)) as usize] = 0;
        for _ in 0..n {
            let mut min = MAX_DIST;
            let mut u = 0;
            for v in 0..n {
                if !spt_set[v] && dist[v] < min {
                    min =  dist[v];
                    u = v;
                }
            }
            spt_set[u] = true;
            for v in 0..n {
                let uu = (u as i32 % m, (u as i32) / m);
                let vv = (v as i32 % m, (v as i32) / m);
                let guv = match self.adj(uu, vv) { true => 1, false => 0 };
                if !spt_set[v] && guv == 1 && dist[u] != MAX_DIST
                    && dist[u] + guv < dist[v] {
                        dist[v] = dist[u] + guv;
                    }
            }
        }
        dist
    }

    /// Calculate if traversal is possible to for all cominations of
    /// points on the board. This is O(n^3).
    pub fn warshall(&self) -> Vec<Vec<bool>>
    {
        let m = N + 2;
        let n = (m*m) as usize;
        let mut w = vec![vec![false; n]; n];
        for a in 0..n {
            for b in 0..n {
                w[a][b] = self.adj(
                    (a as i32 % m - 1, (a as i32) / m - 1),
                    (b as i32 % m - 1, (b as i32) / m - 1))
            }
        }
        for k in 0..n {
            for b in 0..n {
                for a in 0..n {
                    w[a][b] = w[a][b] || (w[a][k] && w[k][b])
                }
            }
        }
        w
    }

    /// Test to see if points a and b are adjacent (you can move a
    /// piece from a to b).
    pub fn adj(&self, a: (i32, i32), b: (i32, i32)) -> bool
    {
        // Boundary conditions
        if     a.0 < -1 || b.0 < -1 || a.1 < -1 || b.1 < -1
            || a.1 > N || b.1 > N
            || a.0 > N || b.0 > N {
            return false
        }

        // if points are not neighbors
        if (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1 ||
           (a.0 - b.0).abs() + (a.1 - b.1).abs() == 2 {
            return false
        }

        // Endzones
        if     ((a.1 == -1 || b.1 == -1) && (a.0 != b.0))
            || ((a.0 == -1 || b.0 == -1) && (a.1 != b.1))
            || ((a.1 == N || b.1 == N) && (a.0 != b.0))
            || ((a.0 == N || b.0 == N) && (a.1 != b.1)) {
                return false
            }

        // Look for vertical wall
        if a.1 == b.1 {
            let x = cmp::max(a.0, b.0);
            let y = a.1;
            let wall1 = Wall {d: Direction::Vertical, x: x, y: y};
            let wall2 = Wall {d: Direction::Vertical, x: x, y: y+1};
            if self.walls.contains(&wall1) || self.walls.contains(&wall2) {
                return false
            }
        }

        // Look for horizontal wall
        if a.0 == b.0 {
            let x = a.0;
            let y = cmp::max(a.1, b.1);
            let wall1 = Wall {d: Direction::Horizontal, x: x, y: y};
            let wall2 = Wall {d: Direction::Horizontal, x: x+1, y: y};
            if self.walls.contains(&wall1) || self.walls.contains(&wall2) {
                return false
            }
        }

        return true;
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
        for (name, p) in self.players.iter() {
            players.push(p.to_json())
        }
        d.insert(s("players"), players.to_json());
        Json::Object(d)
    }

    /// Return game from JSON
    pub fn from_json(doc: String) -> Game {
        let data = Json::from_str(&*doc).unwrap();
        let size = data["size"].as_u64().unwrap() as i32;
        let mut game = Game::new();
        for player in data["players"].as_array().unwrap() {
            println!("player: {:?}", player);
            let name = player["name"].as_string().unwrap().to_string();
            game.players.insert(name.clone(), Player {
                p: (player["position"][0].as_u64().unwrap() as i32,
                    player["position"][1].as_u64().unwrap() as i32),
                id: player["id"].as_u64().unwrap() as u8,
                p_last: None,
                key: "".to_string(),
                walls: player["walls"].as_u64().unwrap() as u8,
                name: name,
            });
        }
        return game
    }
}
