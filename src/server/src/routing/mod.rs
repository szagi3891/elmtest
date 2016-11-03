use std::collections::HashMap;

use lib::router::Router;
use lib::outresponse::OutResponse;
use lib::response_type::ResponseType;
use lib::blob_stor::BlobStor;

mod serve_static;
mod serve_old_api;

pub fn process_router<'a>(
    stor: &BlobStor,
    mut router: Router<'a>,
    static_path: &'a HashMap<String, String>,
    out_response: OutResponse
) {

    if router.eq("api") {

        serve_old_api::serve(out_response, router);
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
    let error_message = format!("Missing in router {:?}", url);
    out_response.send(ResponseType::ServerError, error_message.as_bytes());
}