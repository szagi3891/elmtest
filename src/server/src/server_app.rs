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

                let mut router = Router::new(url.as_str());

                if router.eq("api") {
                    let url = router.url();

                    res.send(format!("Api {:?}", url).as_bytes()).unwrap();
                    return;
                }
                
                for (prefix, prefix_path) in self.static_path.iter() {
                    
                    if router.eq(prefix.as_str()) {
                        println!("dopasowano {:?} {:?}", prefix, prefix_path);
                        res.send(format!("dopasowano {:?} {:?}", prefix, prefix_path).as_bytes()).unwrap();
                        return;
                    }
                }

                println!("dd{:?} {:?} {:?}", url, self.data, self.static_path);
                res.send(format!("Hello World! {:?}", url).as_bytes()).unwrap();
            },
            _ => {
                res.send(b"Hello World! - error").unwrap();
            }
        };
    }
}
