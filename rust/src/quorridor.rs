// ############################################################
// #
// #  Quoridor game logic
// #
// ############################################################

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::borrow::ToOwned;

pub fn s(string: &str) -> String {
    string.to_owned()
}

#[derive(Debug)]
pub struct Player {
    pub p: (i32, i32),
    pub key: String,
    pub id: u8,
}


pub struct Game {
    pub size: i32,
    pub walls: BTreeSet<((i32, i32), (i32, i32))>,
    pub players: BTreeMap<String, Player>,
    pub turn: u8,
}


#[allow(dead_code)]
impl Game {
    pub fn new(size: i32) -> Game
    {
        info!("Creating new {}x{} game", size, size);
        Game {
            size: size,
            players: BTreeMap::new(),
            walls:  BTreeSet::new(),
            turn: 0,
        }
    }

    pub fn add_player(&mut self, name: String, key: String)
                      -> Result<String, String>
    {
        if self.players.contains_key(&name) {
            return Err(format!("Player {} registered twice.", name.clone()))
        }
        if self.players.len() >= 4 {
            return Err(format!("Attempt to register 5th player."))
        }
        let i = self.players.len();
        self.players.insert(name.clone(), Player {
            p: [(self.size/2, 0), (self.size/2, self.size-1),
                (0, self.size/2), (self.size-1, self.size/2)][i],
            key: key,
            id: i as u8,
        });
        return Ok(format!("Added player {}", name))
    }

    pub fn move_player(&mut self, name: String, key: String, dir: String)
                       -> Result<String, String>
    {
        if !self.players.contains_key(&name) {
            return Err(s("Player not found."))
        }

        {
            let p = &self.players[&name];
            if p.key != key { return Err(s("Unauthorized move")) }
            let e = format!("Cannot move player {} {}", name, dir);
            match &*dir.to_uppercase() {
                "UP" => {
                    if (p.id != 1 && p.p.1 == 0) ||
                        !self.adj(p.p, (p.p.0, p.p.1-1)) { return Err(e) }
                }, "DOWN" => {
                    if (p.id != 0 && p.p.1 == self.size) ||
                        !self.adj(p.p, (p.p.0, p.p.1+1)) { return Err(e) }
                }, "LEFT" => {
                    if (p.id != 2 && p.p.0 == 0) ||
                        !self.adj(p.p, (p.p.0-1, p.p.1)) { return Err(e) }
                }, "RIGHT" => {
                    if (p.id != 3 && p.p.1 == self.size) ||
                        !self.adj(p.p, (p.p.0+1, p.p.1)) { return Err(e) }
                }, _ => return Err(format!("Unknown direction {}", dir))
            }
        }

        if let Some(p) = self.players.get_mut(&name) {
            p.p = match &*dir.to_uppercase() {
                "UP"    => (p.p.0,   p.p.1-1),
                "DOWN"  => (p.p.0,   p.p.1+1),
                "LEFT"  => (p.p.0-1, p.p.1),
                "RIGHT" => (p.p.0+1, p.p.1),
                _ => p.p
            }
        }

        Ok(format!("Moved player to {:?}", &self.players[&name].p))
    }

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

    pub fn print(&self)
    {
        println!("");
        for j in -1..self.size + 1 {
            for i in -1..self.size + 1 {
                if self.adj((i, j), (i, j-1)){ print!("+   ") }
                else { print!("+ - ") }
            }
            println!("+");
            for i in -1..self.size + 1 {
                let n = match self.get_player_at_position((i, j)) {
                    Ok(name) => format!("{}", self.players[&name].id),
                    Err(_) => s(" ")
                };
                if self.adj((i, j), (i-1, j)){ print!("  {} ", n) }
                else { print!("| {} ", n) }
            }
            println!("|");
        }
        for _ in -1..self.size + 1 { print!("+ - ") }
        println!("+");
    }

    pub fn print_walls(&self)
    {
        for w in self.walls.iter() {
            println!("Wall from {:?} to {:?}", w.0, w.1);
        }
    }

    pub fn add_wall(&mut self, a: (i32, i32), b: (i32, i32))
                    -> Result<String, String>
    {
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
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0, a.1-1), (a.0, a.1+1))) ||
                self.walls.contains(&((a.0, a.1+1), (a.0, a.1+3))) ||
                self.walls.contains(&((a.0-1, a.1+1), (a.0+1, a.1+1))) {
                    return Err(s("Wall collides with existing vertical wall"))
                }
        }

        // Horizontal collisions
        if a.1 == b.1 {
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0-1, a.1), (a.0+1, a.1))) ||
                self.walls.contains(&((a.0+1, a.1), (a.0+3, a.1))) ||
                self.walls.contains(&((a.0+1, a.1-1), (a.0+1, a.1+1))) {
                    return Err(s("Wall collides with existing horizontal wall"))
                }
        }

        info!("Adding wall {:?} -> {:?}", a, b);
        self.walls.insert((a, b));
        self.walls.insert((b, a));

        let w = &self.warshall();
        for name in self.players.keys() {
            if !self.check_win_condition(w, (*name).clone()) {
                info!("Redacting wall {:?} -> {:?}, eliminates path for {}",
                      a, b, name);
                self.walls.remove(&(a, b));
                self.walls.remove(&(b, a));
                return Err(format!("Wall eliminates path for {}", name))
            }
        }

        Ok(format!("Added wall {:?} -> {:?}", a, b))
    }

    pub fn check_win_condition(&self, w: &Vec<Vec<bool>>, name: String) -> bool
    {
        let p = &self.players[&name];
        match p.id {
            0 => (1..self.size).fold(
                false, |v, i| v || self.connected(&w, p.p, (i, self.size))),
            1 => (1..self.size).fold(
                false, |v, i| v || self.connected(&w, p.p, (i, 0))),
            2 => (1..self.size).fold(
                false, |v, i| v || self.connected(&w, p.p, (self.size, i))),
            3 => (1..self.size).fold(
                false, |v, i| v || self.connected(&w, p.p, (0, i))),
            _ => false,
        }
    }

    pub fn inc_turn(&mut self)
    {
        self.turn = ((self.turn + 1) as i32 % self.players.len() as i32) as u8
    }

    pub fn connected(&self, warshall: &Vec<Vec<bool>>,
                     a: (i32, i32), b: (i32, i32)) -> bool
    {
        let m = self.size+2;
        warshall[(a.0+1+(a.1+1)*m) as usize][(b.0+1+(b.1+1)*m) as usize]
    }

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

}


#[test] fn test_invalid_wall_1() {
    assert!(Game::new(5).add_wall((1, 1), (2, 2)).is_err())
}

#[test] fn test_invalid_wall_2() {
    assert!(Game::new(5).add_wall((1, 1), (1, 4)).is_err())
}

#[test] fn test_invalid_wall_3() {
    assert!(Game::new(5).add_wall((-1, 0), (1, 0)).is_err())
}

#[test] fn test_invalid_wall_4() {
    assert!(Game::new(5).add_wall((4, 0), (6, 0)).is_err())
}

#[test] fn test_wall_vertical_collisions() {
    let mut g = Game::new(5);
    assert!(g.add_wall((1, 1), (1, 3)).is_ok());
    assert!(g.add_wall((1, 2), (1, 4)).is_err());
    assert!(g.add_wall((1, 0), (1, 2)).is_err());
    assert!(g.add_wall((0, 2), (2, 2)).is_err());
}

#[test] fn test_wall_horizontal_collisions() {
    let mut g = Game::new(5);
    assert!(g.add_wall((1, 1), (3, 1)).is_ok());
    g.print();
    assert!(g.add_wall((2, 1), (4, 1)).is_err());
    assert!(g.add_wall((0, 1), (2, 1)).is_err());
    assert!(g.add_wall((2, 0), (2, 2)).is_err());
}

#[test] fn test_valid_wall_1() {
    assert!(Game::new(5).add_wall((1, 1), (1, 3)).is_ok())
}

#[test] fn test_valid_wall_2() {
    assert!(Game::new(5).add_wall((1, 3), (1, 1)).is_ok())
}

#[test] fn test_valid_wall_3() {
    assert!(Game::new(5).add_wall((1, 3), (3, 3)).is_ok())
}

#[test] fn test_valid_wall_4() {
    assert!(Game::new(5).add_wall((3, 3), (1, 3)).is_ok())
}

#[test] fn test_valid_wall_5() {
    let mut g = Game::new(5);
    g.add_player(s("Player 1"), s(""));
    assert!(g.add_wall((0, 2), (2, 2)).is_ok());
    assert!(g.add_wall((2, 2), (4, 2)).is_ok());
    assert!(g.add_wall((4, 0), (4, 2)).is_err());
}

#[test] fn test_adj_1() {
    let mut g = Game::new(5);
    for i in 0..g.size {
        for j in 0..g.size {
            assert!(!g.adj((i, j), (i+2, j)));
            assert!(!g.adj((i, j), (i-2, j)));
            assert!(!g.adj((i, j), (i, j+2)));
            assert!(!g.adj((i, j), (i, j-2)));
            assert!(!g.adj((i, j), (i-1, j-1)));
            assert!(!g.adj((i, j), (i+1, j+1)));
            assert!(!g.adj((i, j), (i-1, j+1)));
            assert!(!g.adj((i, j), (i+1, j-1)));
        }
    }
}

#[test] fn test_adj_vertical_1() {
    let mut g = Game::new(5);
    assert!(g.adj((1, 1), (2, 1)));
    assert!(g.add_wall((2, 1), (2, 3)).is_ok());
    assert!(!g.adj((1, 1), (2, 1)));
    assert!(!g.adj((1, 2), (2, 2)));
    assert!(!g.adj((2, 1), (1, 1)));
    assert!(!g.adj((2, 2), (1, 2)));
}

#[test] fn test_adj_horizontal_1() {
    let mut g = Game::new(5);
    assert!(g.adj((1, 1), (1, 2)));
    assert!(g.add_wall((1, 2), (3, 2)).is_ok());
    assert!(!g.adj((1, 1), (1, 2)));
    assert!(!g.adj((2, 1), (2, 2)));
    assert!(!g.adj((1, 2), (1, 1)));
    assert!(!g.adj((2, 2), (2, 1)));
}

#[test] fn test_add_players() {
    let mut g = Game::new(5);
    assert!(g.add_player("Player 1".to_string(), "".to_string()).is_ok());
    assert!(g.add_player("Player 2".to_string(), "".to_string()).is_ok());
    assert!(g.add_player("Player 3".to_string(), "".to_string()).is_ok());
    assert!(g.add_player("Player 4".to_string(), "".to_string()).is_ok());
    assert!(g.add_player("Player 5".to_string(), "".to_string()).is_err());
}

#[test] fn test_move_player() {
    let mut g = Game::new(5);
    assert!(g.add_player(s("Player 1"), s("a")).is_ok());
    assert!(g.move_player(s("Player 1"), s("b"), s("UP")).is_err());
    assert!(g.move_player(s("Player 1"), s("a"), s("UP")).is_err());
    assert!(g.move_player(s("Player 1"), s("a"), s("DOWN")).is_ok());
    assert_eq!(g.players[&s("Player 1")].p,  (2, 1));
    assert!(g.move_player(s("Player 1"), s("a"), s("LEFT")).is_ok());
    assert_eq!(g.players[&s("Player 1")].p,  (1, 1));
    assert!(g.add_wall((0, 1), (2, 1)).is_ok());
    assert!(g.move_player(s("Player 1"), s("a"), s("UP")).is_err());
}
