use hyper::status::StatusCode;
use router::Router;
use outresponse::OutResponse;
use std::collections::HashMap;

pub fn process_router<'a>(
    mut router: Router<'a>,
    data_path: &String,
    static_path: &'a HashMap<String, String>,
    out_response: OutResponse
) {
    
    /*
    (
        status code,
        mime odpowiedzi
        odpowiedź - ciąg bajtów
    )
    */

    //*res.status_mut() = StatusCode::MethodNotAllowed
    //res.headers_mut().set(ContentLength(body.len() as u64));

    if router.eq("api") {
        let url = router.url();

        out_response.send(format!("Api {:?}", url).as_bytes());
        return;
    }

    for (prefix, prefix_path) in (*static_path).iter() {

        if router.eq(prefix.as_str()) {
            out_response.send(format!("dopasowano {:?} {:?}", prefix, prefix_path).as_bytes());
            return;
        }
    }

    let url = router.url();
    out_response.send(format!("Hello World! {:?}", url).as_bytes());
}