use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri;
use std::collections::HashMap;
use router::Router;

pub fn start_server(data_path: String, static_path: HashMap<String, String>) {

    let app = ServerApp {
        data: data_path,
        static_path: static_path
    };

    Server::http("0.0.0.0:8888").unwrap().handle(app).unwrap();
}

pub struct ServerApp {
    //sender: Mutex<Sender<&'static str>>
    data: String,
    static_path: HashMap<String, String>,
}

impl Handler for ServerApp {
    fn handle(&self, req: Request, res: Response) {
        
        match req.uri {
            RequestUri::AbsolutePath(url) => {
                //Some(url.clone())
                let router = Router::new(url.as_str());

                println!("dd{:?} {:?} {:?}", url, self.data, self.static_path);
                res.send(format!("Hello World! {:?}", url).as_bytes()).unwrap();
            },
            _ => {
                res.send(b"Hello World! - error").unwrap();
            }
        };
    }
}
