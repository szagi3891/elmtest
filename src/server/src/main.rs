extern crate hyper;
extern crate getopts;

use hyper::server::{Server, Request, Response};
use getopts::Options;
use std::env;

fn hello(req: Request, res: Response) {
    // handle things here
    res.send(b"Hello World!").unwrap();
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("d", "data", "set path of data", "DATA_PATH");
    opts.optmulti("s", "static", "set path of static", "STATIC_PATH");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    let data_path = match matches.opt_str("data") {
        Some(path) => path,
        None => panic!("Required option 'data'")
    };
    
    let static_list = matches.opt_strs("static");

    println!("data {:?}", data_path);
    println!("ścieżki {:?}", static_list);

    println!("server start");
    Server::http("0.0.0.0:8888").unwrap().handle(hello).unwrap();
}
