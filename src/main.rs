// ############################################################
// #
// #  Quoridorr implementation in rust
// #
// ############################################################

// Standard libraries
use std::env;

// Util libraries
#[macro_use]
extern crate log;
extern crate num;
extern crate env_logger;
extern crate rustc_serialize;
extern crate router;
extern crate iron;
extern crate mount;
extern crate staticfile;

// Declare tests module
#[allow(dead_code, unused_imports)]
mod tests;

// Quorridor game logic
#[allow(dead_code)]
mod quoridor;
use quoridor::Game;

// Module for serving API
#[allow(dead_code)]
mod server;
use server::listen;


#[allow(dead_code)]
fn main() {

    // Get the server host argument
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Missing required argument 'server': <host:port>");
        return
    }

    // Start game server
    listen(args[1].clone(), Game::new(9));

}
