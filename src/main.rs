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
extern crate rustc_serialize;
extern crate router;
extern crate iron;

// Quorridor game logic
#[allow(dead_code)]
mod quoridor;
use quoridor::Game;


#[allow(dead_code)]
mod server;
use server::listen;


fn main() {

    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Missing required argument 'server': <host:port>");
        return
    }

    let mut game = Game::new(9);
    game.turn = 0;
    listen(args[1].clone(), game);

}
