use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::ErrorKind;

use lib::blob_stor::driver::get_file::get_file;

pub fn set_file(path: &Path, content: &[u8]) {
                //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new

    let mut file_opt = OpenOptions::new().write(true)
                                 .create_new(true)
                                 .open(&path);

    match file_opt {
        Ok(mut file) => {

            file.write_all(content).unwrap();                
        },
        Err(err) => {
            
            if err.kind() == ErrorKind::AlreadyExists {
                
                if verify(path, content) {
                    return;
                }

                panic!("error verify content {:?}", path);
            }
            
            panic!("error write {:?} -> {:?}", path, err.kind());
        }
    }
}

fn verify(path: &Path, content: &[u8]) -> bool {

    let buf = get_file(path);

    return buf.as_slice() == content;
}