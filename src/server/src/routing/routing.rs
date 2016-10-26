use lib::router::Router;
use lib::outresponse::OutResponse;
use std::collections::HashMap;

use lib::response_type::ResponseType;
use routing::serve_static;

                                        //TODO - move to mod.rs

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


    //http://www.freefavicon.com/freefavicons/objects/iconinfo/a-new-computer-152-4714.html
    
    for (prefix, prefix_path) in (*static_path).iter() {

        if router.eq(prefix.as_str()) {
            let url = router.url();

            serve_static::serve(out_response, prefix_path, url);

            return;
        }
    }

    let url = router.url();
    out_response.send(ResponseType::ServerError, format!("Brak dopasowania w routerze {:?}", url).as_bytes());
}