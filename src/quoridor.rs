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


const MAX_DIST: i32 = 100000;
pub const GAME_OVER: i32 = -2;
pub const GAME_NOT_STARTED: i32 = -2;

pub fn s(string: &str) -> String {
    string.to_owned()
}


/***********************************************************************
 * Game Structs
 ***********************************************************************/

#[derive(Debug)]
pub struct Player {
    pub p: (i32, i32),
    pub key: String,
    pub id: u8,
    pub walls: u8,
}

pub struct Game {
    pub size: i32,
    pub walls: BTreeSet<((i32, i32), (i32, i32))>,
    pub players: BTreeMap<String, Player>,
    pub turn: i32,
}

/***********************************************************************
 * Player implementations
 ***********************************************************************/

impl Player {
    pub fn to_json(&self, name: &String) -> Json {
        let mut d = BTreeMap::new();
        d.insert(s("position"), vec![self.p.0, self.p.1].to_json());
        d.insert(s("id"), self.id.to_json());
        d.insert(s("walls"), self.walls.to_json());
        d.insert(s("name"), name.to_json());
        Json::Object(d)
    }
}

/***********************************************************************
 * Game implementations
 ***********************************************************************/

#[allow(dead_code)]
impl Game {

    /// Creates a new game of size `size x size`
    pub fn new(size: i32) -> Game
    {
        Game {
            size: size,
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
            if     (p.p.1 < 0 && p.id != 1)
                || (p.p.1 >= self.size && p.id != 0)
                || (p.p.0 < 0 && p.id != 2)
                || (p.p.1 >= self.size && p.id != 3)
            {
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
        self.players.insert(name.clone(), Player {
            p: [(self.size/2, 0), (self.size/2, self.size-1),
                (0, self.size/2), (self.size-1, self.size/2)][i],
            key: key,
            id: i as u8,
            walls: 0,
        });

        // If we have enough players, start the game
        if self.players.len() == 2 { self.start_game() }

        return Ok(format!("Added player {}", name))
    }

    /// Moves a player to an (x, y) coordinate.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player.
    /// * `pos`  - The (x, y) coordinates to try and move to
    pub fn move_player_to(&mut self, name: String, pos: (i32, i32))
                       -> Result<String, String>
    {
        if !self.players.contains_key(&name) {
            return Err(s("Player not found."))
        } else {
            let p = &self.players[&name];

            // Boundary checks
            if (pos.1 < 0 && p.id != 1) || (pos.1 >= self.size && p.id != 0) ||
                (pos.0 < 0 && p.id != 2) || (pos.0 >= self.size && p.id != 3){
                    return Err(s("Atempted to move out of bounds"))
                }

            // Check position
            if self.get_player_at_position(pos).is_ok() {
                return Err(s("Position is not empty"))
            }

            // Check for jumps
            if !self.adj(p.p, pos) {
                let (dx, dy) = (pos.0 - p.p.0, pos.1 - p.p.1);

                if (dy.abs() == 2 && dx == 0) || (dy == 0 && dx.abs() == 2) {
                    // Linear jumps
                    if self.get_player_at_position((pos.0-dx/2, pos.1-dy/2)).is_err()
                        || !self.adj((pos.0-dx/2, pos.1-dy/2), (pos.0, pos.1))
                        || !self.adj((pos.0-dx/2, pos.1-dy/2), (pos.0-dx, pos.1-dy)) {
                        return Err(format!(
                            "Invalid linear jump for {} from {:?} to {:?}",
                            name, p.p, pos))
                    }

                } else if dx.abs() == 1 && dy.abs() == 1 {
                    // Corner Jumps
                    if !((!self.adj((pos.0, p.p.1), (pos.0+dx, p.p.1))
                          && self.adj((p.p.0, p.p.1), (pos.0, p.p.1))
                          && self.adj((pos.0, pos.1), (pos.0, p.p.1)))
                         || (!self.adj((p.p.0, pos.1), (p.p.0, pos.1+dy))
                             && self.adj((p.p.0, p.p.1), (p.p.0, pos.1))
                             && self.adj((pos.0, pos.1), (p.p.0, pos.1)))) {
                        return Err(format!(
                            "Invalid corner jump for {} from {:?} to {:?}",
                            name, p.p, pos))
                    }

                } else {
                    return Err(format!(
                        "Cannot move player {} to {:?}", name, pos))
                }
            }
        }

        // If we made it here, then the move is valid.
        if let Some(p) = self.players.get_mut(&name) { p.p = pos; }

        Ok(format!("Moved player to {:?}", &self.players[&name].p))
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
        for i in -1..self.size + 1 {
            board.push_str(&*format!("{:4}", i)) };
        board.push_str("\n");

        // Vertical iteration
        for j in -1..self.size + 1 {
            board.push_str("   ");
            for i in -1..self.size + 1 {
                if self.adj((i, j), (i, j-1)){
                    board.push_str(&*format!("{}   ", x)) }
                else { board.push_str(&*format!("{} - ", x)) }
            }
            board.push_str(&*format!("+\n{:2} ", j));

            // Horizontal iteration
            for i in -1..self.size + 1 {
                let n = match self.get_player_at_position((i, j)) {
                    Ok(name) => format!("{}", self.players[&name].id),
                    Err(_) => s(" ")
                };
                if self.adj((i, j), (i-1, j)){
                    board.push_str(&*format!("  {} ", n)) }
                else { board.push_str(&*format!("| {} ", n)) }
            }

            board.push_str("|\n");
        }
        board.push_str("   ");
        for _ in -1..self.size + 1 { board.push_str("+ - ") }
        board.push_str("+\n");
        board
    }

    /// Place a wall from intersection a->b
    ///
    /// # Note: wall b->a will also be stored
    pub fn add_wall(&mut self, mut a: (i32, i32), mut b: (i32, i32))
                    -> Result<String, String>
    {
        let mut c;
        // Boundary conditions
        if a.0 < 0 || b.0 < 0 || a.1 < 0 || a.1 < 0
            || a.1 > self.size || b.1 > self.size
            || a.0 > self.size || b.0 > self.size {
            return Err(s("Wall out of bounds."))
        }

        // Validate adjacency
        if !(((a.0 - b.0 == 0) && (a.1 - b.1).abs() == 2) ||
             ((a.1 - b.1 == 0) && (a.0 - b.0).abs() == 2)) {
            return Err(s("Two points must be adjacent"))
        }

        // Vertical collisions
        if a.0 == b.0 {
            if a.1 > b.1 { c = a; a = b; b = c }
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0, a.1-1), (a.0, a.1+1))) ||
                self.walls.contains(&((a.0, a.1+1), (a.0, a.1+3))) ||
                self.walls.contains(&((a.0-1, a.1+1), (a.0+1, a.1+1))) ||
                self.walls.contains(&((a.0-1, a.1-1), (a.0+1, a.1-1)))
            {
                return Err(s("Vertical wall collides with existing wall"))
            }
        }

        // Horizontal collisions
        if a.1 == b.1 {
            if a.0 > b.0 { c = a; a = b; b = c }
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0-1, a.1), (a.0+1, a.1))) ||
                self.walls.contains(&((a.0+1, a.1), (a.0+3, a.1))) ||
                self.walls.contains(&((a.0+1, a.1-1), (a.0+1, a.1+1))) ||
                self.walls.contains(&((a.0+1, a.1+1), (a.0+1, a.1-1)))
            {
                return Err(s("Horizontal Wall collides with existing wall"))
            }
        }

        self.walls.insert((a, b));
        self.walls.insert((b, a));

        for name in self.players.keys() {
            if !self.check_win_condition((*name).clone()) {
                self.walls.remove(&(a, b));
                self.walls.remove(&(b, a));
                return Err(format!("Wall eliminates path for {}", name))
            }
        }

        Ok(format!("Added wall {:?} -> {:?}", a, b))
    }

    /// Check to see if all players have at least 1 possible path to
    /// their endzone
    pub fn check_win_condition(&self, name: String) -> bool
    {
        let p = &self.players[&name];
        let d = self.dijkstra(p.p);
        let m = self.size+2;
        match p.id {
            0 => (1..self.size).fold(
                false, |v, i| v || (d[(i+1+(self.size+1)*m) as usize]) < MAX_DIST),
            1 => (1..self.size).fold(
                false, |v, i| v || (d[((i+1)*m) as usize]) < MAX_DIST),
            2 => (1..self.size).fold(
                false, |v, i| v || (d[(self.size+1+(i+1)*m) as usize]) < MAX_DIST),
            3 => (1..self.size).fold(
                false, |v, i| v || (d[((i+1)*m) as usize]) < MAX_DIST),
            _ => false,
        }
    }

    /// Calculate the length of the shorted path from given source point to
    /// all other points points on the board. This is O(n^2).
    pub fn dijkstra(&self, src: (i32, i32)) -> Vec<i32>
    {
        let m = self.size + 2;
        let n = (m*m) as usize;
        let mut dist = vec![MAX_DIST; n];
        let mut spt_set = vec![false; n];
        dist[(src.0+1+m*(src.1+1)) as usize] = 0;
        for _ in 0..n-1 {
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
                let uu = (u as i32 % m - 1, (u as i32) / m - 1);
                let vv = (v as i32 % m - 1, (v as i32) / m - 1);
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
        let m = self.size + 2;
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
            || a.1 > self.size || b.1 > self.size
            || a.0 > self.size || b.0 > self.size {
            return false
        }

        // If points are not neighbors
        if (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1 ||
           (a.0 - b.0).abs() + (a.1 - b.1).abs() == 2 {
            return false
        }

        // Endzones
        if     ((a.1 == -1 || b.1 == -1) && (a.0 != b.0))
            || ((a.0 == -1 || b.0 == -1) && (a.1 != b.1))
            || ((a.1 == self.size || b.1 == self.size) && (a.0 != b.0))
            || ((a.0 == self.size || b.0 == self.size) && (a.1 != b.1)) {
            return false
        }

        // Look for vertical wall
        if a.1 == b.1 {
            let p = match a.0 < b.0 { true  => a, false => b };
            if self.walls.contains(&((p.0+1, p.1-1), (p.0+1, p.1+1))) ||
                self.walls.contains(&((p.0+1, p.1),   (p.0+1, p.1+2))) {
                return false
            }
        }

        // Look for horizontal wall
        else if a.0 == b.0 {
            let p = match a.1 < b.1 { true  => b, false => a };
            if self.walls.contains(&((p.0-1, p.1), (p.0+1, p.1))) ||
                self.walls.contains(&((p.0+2, p.1), (p.0,   p.1))) {
                return false
            }
        }

        return true;
    }

    /// Return the game state as JSON
    pub fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let mut walls: Vec<Vec<Vec<i32>>> = vec![];
        for w in self.walls.iter() {
            let (a, b) = (w.0, w.1);
            walls.push(vec![vec![a.0, a.1], vec![b.0, b.1]])
        }
        d.insert(s("turn"), self.turn.to_json());
        d.insert(s("size"), self.size.to_json());
        d.insert(s("walls"), walls.to_json());
        let mut players: Vec<Json> = vec![];
        for (name, p) in self.players.iter() {
            players.push(p.to_json(name))
        }
        d.insert(s("players"), players.to_json());
        Json::Object(d)
    }

}
