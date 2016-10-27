use lib::outresponse::OutResponse;
use lib::response_type::ResponseType;
use lib::router::Router;

pub fn serve<'a>(out_response: OutResponse, mut router: Router<'a>) {

    if (router.eq("get")) {

        let url = router.url();

        out_response.send(ResponseType::Html, format!("Api {:?}", url).as_bytes());
        return;
    }
    

        //TODO - użyć niemutowalnej wersji routera do wyświetlania pierwotnego adresu
    let error_message = format!("Missing in router ..TODO..");
    out_response.send(ResponseType::ServerError, error_message.as_bytes());
}
