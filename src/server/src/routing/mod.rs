use lib::router::Router;
use lib::outresponse::OutResponse;
use std::collections::HashMap;

use lib::response_type::ResponseType;
mod serve_static;


pub fn process_router<'a>(
    mut router: Router<'a>,
    data_path: &String,
    static_path: &'a HashMap<String, String>,
    out_response: OutResponse
) {

    if router.eq("api") {
        let url = router.url();

        out_response.send(ResponseType::Html, format!("Api {:?}", url).as_bytes());
        return;
    }
    
    for (prefix, prefix_path) in (*static_path).iter() {

        if router.eq(prefix.as_str()) {
            let url = router.url();

            serve_static::serve(out_response, prefix_path, url);
            return;
        }
    }

    let url = router.url();
    out_response.send(ResponseType::ServerError, format!("Missing in router {:?}", url).as_bytes());
}