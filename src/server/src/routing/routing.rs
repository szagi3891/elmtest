use lib::router::Router;
use lib::outresponse::OutResponse;
use std::collections::HashMap;
use std::fs::File;

use lib::get_file_type::get_file_type;
use lib::response_type::ResponseType;

use std::io::Read;

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

            if let Some(ref file_path) = url {
                
                if let Some(_) = file_path.find("..") {
                    panic!("niedozwolona fraza");
                            //TODO - dorobić poprawną obsługę
                            //sprawdzić czy w to odgałęzienie wchodzi program prawidłowo
                            //normalizować ścieżkę i sprawdzać czy początek zgadza się ze ścieżką bazową
                }

                let file_to_open = format!(".{}{}", prefix_path, file_path);
                
                                                                //TODO - remove this line
                println!("open {:?}", file_to_open);

                let response_type_opt = get_file_type(file_to_open.as_str());

                match File::open(file_to_open) {

                    Ok(mut file) => {

                        let mut file_data: Vec<u8> = Vec::new();

                        match file.read_to_end(&mut file_data) {
                            Ok(_) => {
                                
                                match response_type_opt {
                                    Some(response_type) => {
                                        out_response.send(response_type, file_data.as_slice());
                                    }
                                    None => {
                                        out_response.send(ResponseType::ServerError, file_data.as_slice());
                                    }
                                }
                            }
                            Err(err) => {
                                out_response.send(ResponseType::Html, "error read".as_bytes());
                            },
                        }
                    },

                    Err(err) => {
                        out_response.send(ResponseType::Html, "error open".as_bytes());                        
                    },
                };

                return;
            }
            
            out_response.send(ResponseType::Html, format!("dopasowano - brak urla {:?} {:?} {:?}", prefix, prefix_path, url).as_bytes());

            return;
        }
    }

    let url = router.url();
    out_response.send(ResponseType::ServerError, format!("Brak dopasowania w routerze {:?}", url).as_bytes());
}