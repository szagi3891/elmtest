use hyper::status::StatusCode;
use router::Router;
use outresponse::OutResponse;
use std::collections::HashMap;
use std::fs::File;

use std::io::Read;

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
            
            let url = router.url();

            if let Some(ref file_path) = url {
                
                if let Some(_) = file_path.find("..") {
                    panic!("niedozwolona fraza");
                            //TODO - dorobić poprawną obsługę
                            //sprawdzić czy w to odgałęzienie wchodzi program prawidłowo
                }

                let file_to_open = format!(".{}{}", prefix_path, file_path);

                println!("open {:?}", file_to_open);
                                                                                        //TODO - error handling
                let mut file = File::open(file_to_open).unwrap();
                
                let mut s = String::new();
                file.read_to_string(&mut s).unwrap();
                
                out_response.send(s.as_bytes());
                return;
            }
            
            out_response.send(format!("dopasowano - brak urla {:?} {:?} {:?}", prefix, prefix_path, url).as_bytes());
            return;
        }
    }

    let url = router.url();
    out_response.send(format!("Hello World! {:?}", url).as_bytes());
}