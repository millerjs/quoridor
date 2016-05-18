extern crate quoridor;
extern crate env_logger;

use std::env;

mod tests;
use quoridor::quoridor::Game;
use quoridor::server::listen;

fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing required argument 'server': <host:port>");
        return
    }

    listen(args[1].clone(), Game::new());
}
