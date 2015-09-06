// ############################################################
// #
// #  Quoridor game proper
// #
// ############################################################

use std::collections::BTreeSet;

pub struct Player {
    pub p: (i32, i32),
    pub name: String,
}


pub struct Game {
    pub size: i32,
    pub walls: BTreeSet<((i32, i32), (i32, i32))>,
    pub players: Vec<Player>,
}


#[allow(dead_code)]
impl Game {
    pub fn new(size: i32) -> Game {
        info!("Creating new {}x{} game", size, size);
        Game {
            size: size,
            players: vec![],
            walls:  BTreeSet::new(),
        }
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
                if self.adj((i, j), (i-1, j)){ print!("    ") }
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
            || a.0 > self.size || a.0 > self.size {
            return false
        }

        // Validate adjacency
        if !(((a.0 - b.0 == 0) && (a.1 - b.1).abs() == 2) ||
             ((a.1 - b.1 == 0) && (a.0 - b.0).abs() == 2)) {
            warn!("Invalid wall {:?} - {:?}", a, b);
            return false;
        }

        info!("Adding wall {:?} -> {:?}", a, b);
        self.walls.insert((a, b));
        self.walls.insert((b, a));
        true
    }


    pub fn adj(&self, a: (i32, i32), b: (i32, i32)) -> bool {
        // Boundary conditions
        if     a.0 < -1 || b.0 < -1 || a.1 < -1 || b.1 < -1
            || a.1 > self.size || b.1 > self.size
            || a.0 > self.size || a.0 > self.size {
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
