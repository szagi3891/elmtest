use hyper::server::{Response};
use lib::response_type::ResponseType;

use hyper::status::StatusCode;
use hyper::header::{/*Headers,*/ ContentType};
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
                self.set_content_type(TopLevel::Text, SubLevel::Html, true);
            },

            ResponseType::Css => {
                self.set_content_type(TopLevel::Text, SubLevel::Css, true);
            },

            ResponseType::Js => {
                self.set_content_type(TopLevel::Application, SubLevel::Javascript, true);
            },

            ResponseType::Ico => {
                let sub_level = SubLevel::Ext("x-icon".to_string());                    //TODO - remove unnessesery alocation
                self.set_content_type(TopLevel::Image, sub_level, false);
            },

            ResponseType::NotFound => {
                self.set_response_code(StatusCode::NotFound);
                self.set_content_type(TopLevel::Text, SubLevel::Html, true);
            },
            
            ResponseType::ServerError => {
                self.set_response_code(StatusCode::InternalServerError);
                self.set_content_type(TopLevel::Text, SubLevel::Html, true);
            },
        }
        
        //TODO - dodać niemutowalny router i wykorzystać go przy wyświetlaniu informacji o zepsutej rurce

        match self.res.send(body) {
            Ok(_) => {
                //sending ok
            },
            Err(err) => {
                //std::io::Error - standardowy błąd zapisu na strumień
                println!("response: sending error - {:#?}", err);
            }
        }
    }
    
    fn set_response_code(&mut self, status_code: StatusCode) {
        *self.res.status_mut() = status_code;
    }

    fn set_content_type(&mut self, top: TopLevel, sub:SubLevel, add_utf8: bool) {

        let encoding = match add_utf8 {
            true => vec![(Attr::Charset, Value::Utf8)],
            false => vec![],
        };
        
        self.res.headers_mut().set(
            ContentType(
                Mime(
                    top,
                    sub,
                    encoding
                )
            )
        )
    }
}

//res.headers_mut().set(ContentLength(body.len() as u64));
