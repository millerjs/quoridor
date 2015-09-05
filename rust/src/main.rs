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
#[macro_use]
extern crate log;
extern crate env_logger;


fn hello(_: Request, res: Response<Fresh>) {
    res.send(b"Hello World!").unwrap();
}


fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        return error!("Missing required argument 'server': <host:port>")
    }

    Server::http(&*args[1].to_string()).unwrap().handle(hello);

}
