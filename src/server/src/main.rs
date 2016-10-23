extern crate hyper;
extern crate getopts;

use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri;
use getopts::Options;
use std::env;
use std::collections::HashMap;

struct ServerApp {
    //sender: Mutex<Sender<&'static str>>
    data: String,
    static_path: HashMap<String, String>,
}

impl Handler for ServerApp {
    fn handle(&self, req: Request, res: Response) {
        
        let uri = match req.uri {
            RequestUri::AbsolutePath(url) => Some(url.clone()),
            _ => None
        };

        println!("dd{:?} {:?}", uri, self.data);

        match uri {
            Some(uri) => {
                res.send(format!("Hello World! {:?}", uri).as_bytes()).unwrap();
            },
            None => {
                res.send(b"Hello World! - error").unwrap();
            },
        }
    }
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

    /*
    Split by char:
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

    Split by string:
    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    Split by closure:
    let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);
    */
    
    let mut static_path = HashMap::new();
    
    for static_item in &static_list {
        println!("static item {:?}", static_item);
        let parts: Vec<&str> = static_item.split('=').collect();
        
        if parts.len() == 2 {
            println!(" parts {:?}", parts);
            let key = parts[0].into();
            let value = parts[1].into();
            
            if key == "api" {
                panic!("incorrect param: {:?}", static_item);
            }
            
            static_path.insert(key, value);

        } else {
            panic!("incorect param: {:?}", static_item);
        }
        
    }

    let app = ServerApp {
        data: data_path,
        static_path: static_path
    };
    
    println!("server start -> 0.0.0.0:8888");
    Server::http("0.0.0.0:8888").unwrap().handle(app).unwrap();
}
