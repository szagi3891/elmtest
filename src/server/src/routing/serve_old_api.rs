use lib::outresponse::OutResponse;
use lib::response_type::ResponseType;
use lib::router::Router;
use rustc_serialize::json;

/*
"status" : "ok",
"content" : content.toString(),
"child" : []
*/

#[derive(RustcDecodable, RustcEncodable)]
pub struct Respose  {
    status: String,
    content: String,
    child: Vec<String>,
}

pub fn serve<'a>(out_response: OutResponse, mut router: Router<'a>) {

    if (router.eq("get")) {

        let resp = Respose {
            status: "ok".to_string(),
            content: "bla bla bla".to_string(),
            child: vec![],
        };

        let encoded = json::encode(&resp).unwrap();
        
        out_response.send(ResponseType::Json, encoded.as_bytes());
        return;
    }
    

        //TODO - użyć niemutowalnej wersji routera do wyświetlania pierwotnego adresu
    let error_message = format!("Missing in router ..TODO..");
    out_response.send(ResponseType::ServerError, error_message.as_bytes());
}
