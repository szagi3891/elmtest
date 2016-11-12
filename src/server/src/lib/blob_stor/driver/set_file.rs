use std::path::Path;
//use std::fs::OpenOptions;
//use std::io::Write;
use std::io::ErrorKind;

use lib::blob_stor::driver::get_file::get_file;
use lib::fs::save_file::save_file;

pub fn set_file(path: &Path, content: &[u8]) -> bool {

    match save_file(path, content) {
        Ok(()) => {
            true
        },
        Err(err) => {
            
            if err.kind() == ErrorKind::AlreadyExists {
                
                if verify(path, content) {
                    return false;
                }

                panic!("error verify content {:?}", path);
            }
            
            panic!("error write {:?} -> {:?}", path, err.kind());
        }
    }
}

fn verify(path: &Path, content: &[u8]) -> bool {

    match get_file(path) {
        
        Some(buf) => {
            return buf.as_slice() == content;
        },
        
        None => {
            panic!("Nigdy program nie powinien wejść w to odgałęzienie");
        },
    }
}

/*
fn print_slice(content: &[u8]) {
    let mut vec = Vec::new();
    vec.extend_from_slice(content);
    let str = String::from_utf8(vec);
    println!("string -> {:?}", str);
}
*/