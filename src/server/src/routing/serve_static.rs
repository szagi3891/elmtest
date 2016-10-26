use lib::get_file_type::get_file_type;
use lib::outresponse::OutResponse;
use lib::response_type::ResponseType;

use std::fs::File;
use std::io::Read;

pub fn serve(out_response: OutResponse, prefix_path: &String, url: Option<String>) {    

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
                                out_response.send(ResponseType::ServerError, "missing extension".as_bytes());
                            }
                        }
                    }
                    Err(err) => {
                        out_response.send(ResponseType::ServerError, "error read".as_bytes());
                    },
                }
            },

            Err(err) => {
                out_response.send(ResponseType::NotFound, "File not fount".as_bytes());                        
            },
        };

        return;
    }

    out_response.send(ResponseType::NotFound, "File not fount".as_bytes());   

}