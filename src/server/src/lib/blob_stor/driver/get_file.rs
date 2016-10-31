use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

pub fn get_file(path: &Path) -> Vec<u8> {
        
    let mut buf = Vec::new();

    let mut file_opt = OpenOptions::new().read(true).open(&path);

    match file_opt {
        Ok(mut file) => {

            match file.read_to_end(&mut buf) {

                Ok(_) => {
                    return buf;
                },

                Err(err) => {
                    panic!("error read {:?} -> {:?}", path, err.kind());
                }
            }
        },

        Err(err) => {
            panic!("error in read {:?}", err)
        }
    }
}
