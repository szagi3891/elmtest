extern crate hyper;

use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
    // handle things here
    res.send(b"Hello World!").unwrap();
}

fn main() {

    println!("server start");
    Server::http("0.0.0.0:8888").unwrap().handle(hello).unwrap();
}

/*
fn main() {
    println!("Hello, world!");
}
*/