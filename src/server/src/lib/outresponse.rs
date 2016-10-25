use hyper::server::{Response};

pub struct OutResponse<'a> {
    res: Response<'a>
}

impl<'a> OutResponse<'a> {

    pub fn new(res: Response) -> OutResponse {
        OutResponse {
            res: res
        }
    }

    //json(response)    -- status 200

    pub fn send(self, body: &[u8]) {
        self.res.send(body).unwrap();
    }
}
