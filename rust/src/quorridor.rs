// ############################################################
// #
// #  Quoridor game proper
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
    pub fn new(size: i32) -> Game {
        info!("Creating new {}x{} game", size, size);
        Game {
            size: size,
            players: BTreeMap::new(),
            walls:  BTreeSet::new(),
            turn: 0,
        }
    }

    pub fn add_player(&mut self, name: String, key: String) -> bool {
        if self.players.contains_key(&name) {
            error!("Player {} registered twice.", name);
            return false
        }
        if self.players.len() >= 4 {
            error!("Attempt to register 5th player.");
            return false
        }
        let i = self.players.len();
        self.players.insert(name, Player {
            p: [(self.size/2, 0), (self.size/2, self.size-1),
                (0, self.size/2), (self.size-1, self.size/2)][i],
            key: key,
            id: i as u8,
        });
        true
    }

    pub fn move_player(&mut self, name: String, key: String,
                       dir: String) -> bool {
        if !self.players.contains_key(&name) { return false }
        {
            let p = &self.players[&name];
            if p.key != key { return false }
            match &*dir.to_uppercase() {
                "UP" => {
                    if (p.id != 1 && p.p.1 == 0) ||
                        !self.adj(p.p, (p.p.0, p.p.1-1)) { return false }
                }, "DOWN" => {
                    if (p.id != 0 && p.p.1 == self.size) ||
                        !self.adj(p.p, (p.p.0, p.p.1+1)) { return false }
                }, "LEFT" => {
                    if (p.id != 2 && p.p.0 == 0) ||
                        !self.adj(p.p, (p.p.0-1, p.p.1)) { return false }
                }, "RIGHT" => {
                    if (p.id != 3 && p.p.1 == self.size) ||
                        !self.adj(p.p, (p.p.0+1, p.p.1)) { return false }
                }, _ => return false
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

        true
    }

    pub fn print(&self) {
        println!("");
        for j in -1..self.size + 1 {
            for i in -1..self.size + 1 {
                if self.adj((i, j), (i, j-1)){ print!("+   ") }
                else { print!("+ - ") }
            }
            println!("+");
            for i in -1..self.size + 1 {
                let mut n = ' ';
                for (_, p) in self.players.iter() {
                    if p.p.0 == i as i32 && p.p.1 == j as i32 {
                        n = ['A', 'B', 'C', 'D'][p.id as usize] }
                }
                if self.adj((i, j), (i-1, j)){ print!("  {} ", n) }
                else { print!("|   ") }
            }
            println!("|");
        }
        for _ in -1..self.size + 1 { print!("+ - ") }
        println!("+");
    }

    pub fn print_walls(&self) {
        for w in self.walls.iter() {
            println!("Wall from {:?} to {:?}", w.0, w.1);
        }
    }

    pub fn add_wall(&mut self, a: (i32, i32), b: (i32, i32)) -> bool {
        // Boundary conditions
        if a.0 < 0 || b.0 < 0 || a.1 < 0 || a.1 < 0
            || a.1 > self.size || b.1 > self.size
            || a.0 > self.size || b.0 > self.size {
            return false
        }

        // Validate adjacency
        if !(((a.0 - b.0 == 0) && (a.1 - b.1).abs() == 2) ||
             ((a.1 - b.1 == 0) && (a.0 - b.0).abs() == 2)) {
            return false
        }

        // Vertical collisions
        if a.0 == b.0 {
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0, a.1-1), (a.0, a.1+1))) ||
                self.walls.contains(&((a.0, a.1+1), (a.0, a.1+3))) ||
                self.walls.contains(&((a.0-1, a.1+1), (a.0+1, a.1+1))) {
                    return false
                }
        }

        // Horizontal collisions
        if a.1 == b.1 {
            if self.walls.contains(&(a, b)) ||
                self.walls.contains(&((a.0-1, a.1), (a.0+1, a.1))) ||
                self.walls.contains(&((a.0+1, a.1), (a.0+3, a.1))) ||
                self.walls.contains(&((a.0+1, a.1-1), (a.0+1, a.1+1))) {
                    return false
                }
        }

        info!("Adding wall {:?} -> {:?}", a, b);
        self.walls.insert((a, b));
        self.walls.insert((b, a));
        true
    }

    pub fn inc_turn(&mut self) {
        self.turn = ((self.turn + 1) as i32 % self.players.len() as i32) as u8
    }

    pub fn connected(&self, warshall: &Vec<Vec<bool>>,
                     a: (i32, i32), b: (i32, i32)) -> bool {
        let m = self.size+2;
        warshall[(a.0+1+(a.1+1)*m) as usize][(b.0+1+(b.1+1)*m) as usize]
    }

    pub fn warshall(&self) -> Vec<Vec<bool>> {
        let m = self.size + 2;
        let n = (m*m) as usize;
        let mut w = vec![vec![false; n]; n];
        for a in 0..n {
            for b in 0..n {
                w[a][b] = self.adj((a as i32 % m - 1, (a as i32) / m - 1),
                                   (b as i32 % m - 1, (b as i32) / m - 1))
            }
        }
        for k in 0..n {
            for a in 0..n {
                for b in 0..n {
                    w[a][b] = w[a][b] || (w[a][k] && w[k][b])
                }
            }
        }
        w
    }

    pub fn adj(&self, a: (i32, i32), b: (i32, i32)) -> bool {
        // Boundary conditions
        if     a.0 < -1 || b.0 < -1 || a.1 < -1 || b.1 < -1
            || a.1 > self.size || b.1 > self.size
            || a.0 > self.size || b.0 > self.size {
            return false
        }

        // If points are not neighbors
        if (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1 {
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
    assert!(!Game::new(5).add_wall((1, 1), (2, 2))) }
#[test] fn test_invalid_wall_2() {
    assert!(!Game::new(5).add_wall((1, 1), (1, 4))) }
#[test] fn test_invalid_wall_3() {
    assert!(!Game::new(5).add_wall((-1, 0), (1, 0))) }
#[test] fn test_invalid_wall_4() {
    assert!(!Game::new(5).add_wall((4, 0), (6, 0))) }
#[test] fn test_wall_vertical_collisions() {
    let mut g = Game::new(5);
    assert!(g.add_wall((1, 1), (1, 3)));
    assert!(!g.add_wall((1, 2), (1, 4)));
    assert!(!g.add_wall((1, 0), (1, 2)));
    assert!(!g.add_wall((0, 2), (2, 2)));
}
#[test] fn test_wall_horizontal_collisions() {
    let mut g = Game::new(5);
    assert!(g.add_wall((1, 1), (3, 1)));
    g.print();
    assert!(!g.add_wall((2, 1), (4, 1)));
    assert!(!g.add_wall((0, 1), (2, 1)));
    assert!(!g.add_wall((2, 0), (2, 2)));
}
#[test] fn test_valid_wall_1() {
    assert!(Game::new(5).add_wall((1, 1), (1, 3))) }
#[test] fn test_valid_wall_2() {
    assert!(Game::new(5).add_wall((1, 3), (1, 1))) }
#[test] fn test_valid_wall_3() {
    assert!(Game::new(5).add_wall((1, 3), (3, 3))) }
#[test] fn test_valid_wall_4() {
    assert!(Game::new(5).add_wall((3, 3), (1, 3))) }
#[test] fn test_adj_vertical_1() {
    let mut g = Game::new(5);
    assert!(g.adj((1, 1), (2, 1)));
    assert!(g.add_wall((2, 1), (2, 3)));
    assert!(!g.adj((1, 1), (2, 1)));
    assert!(!g.adj((1, 2), (2, 2)));
}
#[test] fn test_adj_horizontal_1() {
    let mut g = Game::new(5);
    assert!(g.adj((1, 1), (1, 2)));
    assert!(g.add_wall((1, 2), (3, 2)));
    assert!(!g.adj((1, 1), (1, 2)));
    assert!(!g.adj((2, 1), (2, 2)));
}
#[test] fn test_add_players() {
    let mut g = Game::new(5);
    assert!(g.add_player("Player 1".to_string(), "".to_string()));
    assert!(g.add_player("Player 2".to_string(), "".to_string()));
    assert!(g.add_player("Player 3".to_string(), "".to_string()));
    assert!(g.add_player("Player 4".to_string(), "".to_string()));
    assert!(!g.add_player("Player 5".to_string(), "".to_string()));
}
#[test] fn test_move_player() {
    let mut g = Game::new(5);
    assert!(g.add_player(s("Player 1"), s("a")));
    assert!(!g.move_player(s("Player 1"), s("b"), s("UP")));
    assert!(!g.move_player(s("Player 1"), s("a"), s("UP")));
    assert!(g.move_player(s("Player 1"), s("a"), s("DOWN")));
    assert_eq!(g.players[&s("Player 1")].p,  (2, 1));
    assert!(g.move_player(s("Player 1"), s("a"), s("LEFT")));
    assert_eq!(g.players[&s("Player 1")].p,  (1, 1));
    assert!(g.add_wall((0, 1), (2, 1)));
    assert!(!g.move_player(s("Player 1"), s("a"), s("UP")));
}
