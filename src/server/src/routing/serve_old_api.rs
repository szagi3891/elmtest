use lib::outresponse::OutResponse;
use lib::response_type::ResponseType;
use lib::router::Router;

pub fn serve<'a>(out_response: OutResponse, mut router: Router<'a>) {

    let url = router.url();

    out_response.send(ResponseType::Html, format!("Api {:?}", url).as_bytes());
}
