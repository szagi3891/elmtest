use hyper::status::StatusCode;
use lib::router::Router;
use lib::outresponse::OutResponse;
use std::collections::HashMap;
use std::fs::File;

use std::io::Read;

                                        //TODO - move to mod.rs

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

/*
    typy:
    favicon
    text ok     dodać kodowanie utf8
    text error  dodać kodowanie utf8
    obrazek
    css
    js
*/
    
    //http://www.freefavicon.com/freefavicons/objects/iconinfo/a-new-computer-152-4714.html
    
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
                
                                                                //TODO - remove this line
                println!("open {:?}", file_to_open);
                
                match File::open(file_to_open) {

                    Ok(mut file) => {

                        let mut file_data: Vec<u8> = Vec::new();

                        match file.read_to_end(&mut file_data) {
                            Ok(_) => {
                                
                                out_response.send(file_data.as_slice());
                                return;
                            }
                            Err(err) => {
                                out_response.send("error read".as_bytes());
                                return;
                            },
                        }
                    },

                    Err(err) => {
                        out_response.send("error open".as_bytes());
                        return;
                    },
                };
            }
            
            out_response.send(format!("dopasowano - brak urla {:?} {:?} {:?}", prefix, prefix_path, url).as_bytes());
            return;
        }
    }

    let url = router.url();
    out_response.send(format!("Hello World! {:?}", url).as_bytes());
}