use quoridor::constants::N;
use quoridor::quoridor::Game;
use quoridor::adjacency_matrix::AdjacencyMatrix;
use quoridor::board::Point;

/// Convert (x, y) tuple to a point
macro_rules! point { ( $x:expr, $y:expr ) => { { Point{ x: $x, y: $y} } }; }

#[test]
fn test_invalid_wall_1() {
    assert!(Game::new().add_wall_tuples((1, 1), (2, 2)).is_err())
}

#[test]
fn test_invalid_wall_2() {
    assert!(Game::new().add_wall_tuples((1, 1), (1, 4)).is_err())
}

#[test]
fn test_invalid_wall_3() {
    assert!(Game::new().add_wall_tuples((-1, 0), (1, 0)).is_err())
}

#[test]
fn test_invalid_wall_4() {
    assert!(Game::new().add_wall_tuples((4, 0), (6, 0)).is_err())
}

#[test]
fn test_wall_vertical_collisions() {
    let mut g = Game::new();
    assert!(g.add_wall_tuples((1, 1), (1, 3)).is_ok());
    assert!(g.add_wall_tuples((1, 2), (1, 4)).is_err());
    assert!(g.add_wall_tuples((1, 0), (1, 2)).is_err());
    assert!(g.add_wall_tuples((0, 2), (2, 2)).is_err());
}

#[test]
fn test_wall_horizontal_collisions() {
    let mut g = Game::new();
    assert!(g.add_wall_tuples((1, 1), (3, 1)).is_ok());
    assert!(g.add_wall_tuples((2, 1), (4, 1)).is_err());
    assert!(g.add_wall_tuples((0, 1), (2, 1)).is_err());
    assert!(g.add_wall_tuples((2, 0), (2, 2)).is_err());
}

#[test]
fn test_valid_wall_1() {
    assert!(Game::new().add_wall_tuples((1, 1), (1, 3)).is_ok())
}

#[test]
fn test_valid_wall_2() {
    assert!(Game::new().add_wall_tuples((1, 3), (1, 1)).is_ok())
}

#[test]
fn test_valid_wall_3() {
    assert!(Game::new().add_wall_tuples((1, 3), (3, 3)).is_ok())
}

#[test]
fn test_valid_wall_4() {
    assert!(Game::new().add_wall_tuples((3, 3), (1, 3)).is_ok())
}

#[test]
fn test_valid_wall_5() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "").is_ok());
    assert!(g.add_wall_tuples((0, 2), (2, 2)).is_ok());
    assert!(g.add_wall_tuples((2, 2), (4, 2)).is_ok());
}


#[test]
fn test_win_condition() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "").is_ok());
    assert!(g.add_wall_tuples((4, 0), (4, 2)).is_ok());
    assert!(g.add_wall_tuples((4, 2), (6, 2)).is_ok());
    assert!(g.add_wall_tuples((6, 0), (6, 2)).is_err());
}


#[test]
fn test_adj_1() {
    let g = Game::new();
    for i in 0..N {
        for j in 0..N {
            assert!(!g.adj(point!(i, j), point!(i+2, j)));
            assert!(!g.adj(point!(i, j), point!(i-2, j)));
            assert!(!g.adj(point!(i, j), point!(i, j+2)));
            assert!(!g.adj(point!(i, j), point!(i, j-2)));
            assert!(!g.adj(point!(i, j), point!(i-1, j-1)));
            assert!(!g.adj(point!(i, j), point!(i+1, j+1)));
            assert!(!g.adj(point!(i, j), point!(i-1, j+1)));
            assert!(!g.adj(point!(i, j), point!(i+1, j-1)));
        }
    }
}

#[test]
fn test_adj_vertical_1() {
    let mut g = Game::new();
    assert!(g.adj(point!(1, 1), point!(2, 1)));
    assert!(g.add_wall_tuples((2, 1), (2, 3)).is_ok());
    assert!(!g.adj(point!(1, 1), point!(2, 1)));
    assert!(!g.adj(point!(1, 2), point!(2, 2)));
    assert!(!g.adj(point!(2, 1), point!(1, 1)));
    assert!(!g.adj(point!(2, 2), point!(1, 2)));
}

#[test]
fn test_adj_horizontal_1() {
    let mut g = Game::new();
    assert!(g.adj(point!(1, 1), point!(1, 2)));
    assert!(g.add_wall_tuples((1, 2), (3, 2)).is_ok());
    assert!(!g.adj(point!(1, 1), point!(1, 2)));
    assert!(!g.adj(point!(2, 1), point!(2, 2)));
    assert!(!g.adj(point!(1, 2), point!(1, 1)));
    assert!(!g.adj(point!(2, 2), point!(2, 1)));
}

#[test]
fn test_add_players() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "").is_ok());
    assert!(g.add_player("Player 2", "").is_ok());
    assert!(g.add_player("Player 5", "").is_err());
}

#[test]
fn test_move_player() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "a").is_ok());
    assert!(g.move_player("Player 1", "NORTH").is_err());
    assert!(g.move_player("Player 1", "south").is_ok());
    assert_eq!(g.players[&"Player 1".to_string()].p,  point!(4, 1));
    assert!(g.move_player("Player 1", "WEST").is_ok());
    assert_eq!(g.players[&"Player 1".to_string()].p,  point!(3, 1));
    assert!(g.add_wall_tuples((2, 1), (4, 1)).is_ok());
    assert!(g.move_player("Player 1", "NORTH").is_err());
}

#[test]
fn test_move_player_boundary() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "a").is_ok());
    assert!(g.add_player("Player 2", "a").is_ok());
    assert!(g.move_player("Player 1", "NORTH").is_err());
    assert!(g.move_player("Player 2", "SOUTH").is_err());
}

#[test]
fn test_end_game() {
    let mut g = Game::new();
    assert!(g.add_player("Player 1", "a").is_ok());
    for _ in 0..N-1 {
        assert!(g.move_player("Player 1", "SOUTH").is_ok());
    }
    assert!(g.move_player("Player 1", "SOUTH").is_err());
}
