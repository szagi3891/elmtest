use hyper::server::{Response};
use lib::response_type::ResponseType;

use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub struct OutResponse<'a> {
    res: Response<'a>
}

impl<'a> OutResponse<'a> {

    pub fn new(res: Response) -> OutResponse {
        OutResponse {
            res: res
        }
    }

    pub fn send(mut self, resp_type: ResponseType, body: &[u8]) {

        match resp_type {
            ResponseType::Html => {
                self.set_content_type(TopLevel::Text, SubLevel::Html);
            },

            ResponseType::Css => {
                self.set_content_type(TopLevel::Text, SubLevel::Css);
            },

            ResponseType::Js => {
                self.set_content_type(TopLevel::Application, SubLevel::Javascript);
            },

            ResponseType::ServerError => {
                self.set_response_code(StatusCode::InternalServerError);
                self.set_content_type(TopLevel::Text, SubLevel::Html);
            },
        }
        
        self.res.send(body).unwrap();
    }
    
    fn set_response_code(&mut self, status_code: StatusCode) {
        *self.res.status_mut() = status_code;
    }

    fn set_content_type(&mut self, top: TopLevel, sub:SubLevel) {
        self.res.headers_mut().set(
            ContentType(
                Mime(
                    top,
                    sub,
                    vec![(Attr::Charset, Value::Utf8)]
                )
            )
        )
    }
}

//res.headers_mut().set(ContentLength(body.len() as u64));
