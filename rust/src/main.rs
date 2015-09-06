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
mod quorridor;
use quorridor::Game;
use quorridor::s;

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

    g.add_player(s("Player 1"), s(a_key));
    g.add_player(s("Player 2"), s(b_key));
    g.add_wall((0, 2), (2, 2));
    g.add_wall((2, 2), (4, 2));
    g.add_wall((4, 0), (4, 2));
    g.print();

    let w = g.warshall();

    for j in -1..g.size+1 {
        for i in -1..g.size+1 {
            print!("{} ", match g.connected(&w, (0, 0), (i, j)) {
                true => '.', false => 'x'});
        }
        println!("");
    }


}
