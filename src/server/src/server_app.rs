use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri;
use std::collections::HashMap;
use router::Router;
use routing::process_router;
use outresponse::OutResponse;

pub fn start_server(data_path: String, static_path: HashMap<String, String>) {

    let app = ServerApp {
        data: data_path,
        static_path: static_path
    };

    Server::http("0.0.0.0:8888").unwrap().handle(app).unwrap();
}

pub struct ServerApp {
    data: String,
    static_path: HashMap<String, String>,
}

impl Handler for ServerApp {
    fn handle(&self, req: Request, res: Response) {
        
        match req.uri {
            RequestUri::AbsolutePath(url) => {

                let router = Router::new(url.as_str());
                let out_response = OutResponse::new(res);

                process_router(router, &(self.data), &(self.static_path), out_response);
            },
            _ => {
                res.send(b"Hello World! - error").unwrap();
            }
        };
    }
}
