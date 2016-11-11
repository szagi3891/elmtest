use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::ErrorKind;

use lib::blob_stor::driver::get_file::get_file;

pub fn set_file(path: &Path, content: &[u8]) -> bool {
                //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new

    let mut file_opt = OpenOptions::new().write(true)
                                 .create_new(true)
                                 .open(&path);

    match file_opt {
        Ok(mut file) => {

            file.write_all(content).unwrap();
            file.flush().unwrap();

            return true;
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
    };
}

fn verify(path: &Path, content: &[u8]) -> bool {

    println!("GET {:?}", path);
    
    let buf = get_file(path);
/*
    println!("-------------");
    println!("1 -> {:?}", buf.as_slice());
    print_slice(buf.as_slice());
    println!("-------------");
    println!("2 -> {:?}", content);
    print_slice(content);
    println!("-------------");
*/
    return buf.as_slice() == content;
}

/*
fn print_slice(content: &[u8]) {
    let mut vec = Vec::new();
    vec.extend_from_slice(content);
    let str = String::from_utf8(vec);
    println!("string -> {:?}", str);
}
*/