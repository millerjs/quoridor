// ############################################################
// #
// #  Quoridorr implementation in rust
// #
// ############################################################

// Standard library
use std::env;

// Util libraries
#[macro_use]
extern crate log;
extern crate num;
extern crate env_logger;

// Quorridor game logic
#[allow(dead_code)]
mod quoridor;
use quoridor::Game;
use quoridor::s;

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Missing required argument 'server': <host:port>");
        return
    }

    let mut g = Game::new(5);
    let a_key = "qwehgaa";
    let b_key = "wetgasd";

    g.add_player(s("Player 1"), s(a_key)).is_ok();
    g.add_player(s("Player 2"), s(b_key)).is_ok();
    g.move_player(s("Player 1"), s(a_key), s("DOWN")).is_ok();
    // g.move_player(s("Player 1"), s(a_key), s("DOWN")).is_ok();
    // println!("{}", g.check_win_condition(s("Player 1")));

    // g.add_wall((2, 2), (2, 0)).is_ok();
    g.add_wall((3, 2), (3, 0)).is_ok();
    g.add_wall((0, 4), (2, 4)).is_ok();
    g.add_wall((0, 4), (2, 4)).is_ok();
    g.add_wall((0, 2), (2, 2)).is_ok();
    g.add_wall((2, 2), (4, 2)).is_ok();

    g.print();

    let d = g.dijkstra((2, 1));
    let m = g.size + 2;
    for b in -1..g.size+1 {
        println!("");
        for a in -1..g.size+1 {
            print!("{} ", d[((a+1)+(b+1)*m) as usize]);
        }
    }

}
