// ############################################################
// #
// #  Quoridorr implementation in rust
// #
// ############################################################

// Standard library
extern crate hyper;
use std::io::Write;
use std::env;

// HTTP library
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

// Util libraries
extern crate getopts;
use getopts::Options;


fn hello(_: Request, res: Response<Fresh>) {
    res.send(b"Hello World!").unwrap();
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }

    // Server::http("127.0.0.1:3000").unwrap().handle(hello);

}
