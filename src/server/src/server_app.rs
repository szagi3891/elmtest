use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri;
use std::collections::HashMap;
use routing::process_router;
use std::path::PathBuf;

use lib::router::Router;
use lib::outresponse::OutResponse;
use lib::blob_stor::BlobStor;


pub struct ServerApp {
    stor: BlobStor,
    static_path: HashMap<String, String>,
}

impl Handler for ServerApp {
    fn handle(&self, req: Request, res: Response) {
        
        match req.uri {
            RequestUri::AbsolutePath(url) => {

                let router = Router::new(url.as_str());
                let out_response = OutResponse::new(res);

                process_router(
                    &self.stor,
                    router,
                    &(self.static_path),
                    out_response
                );
            },
            _ => {
                res.send(b"Hello World! - error").unwrap();
            }
        };
    }
}

pub fn start_server(data_path: PathBuf, static_path: HashMap<String, String>) {

    let app = ServerApp {
        stor: BlobStor::new(data_path, 1000),
        static_path: static_path
    };

    Server::http("0.0.0.0:8888").unwrap().handle(app).unwrap();
}
