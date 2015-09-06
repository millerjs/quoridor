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


fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Missing required argument 'server': <host:port>");
        return
    }

    let mut g = Game::new(5);
    // g.add_wall((0, 0), (0, 2));
    g.add_wall((1, 2), (3, 2));
    // g.add_wall((0, 0), (2, 0));
    g.print();
    g.print_walls();

}
