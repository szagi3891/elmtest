use std::fs::read_dir;
use std::path::Path;
use std::collections::HashMap;
use std::fs::OpenOptions;

use lib::blob_stor::hash::Hash;

pub struct DriverUninit {
    path: String,
}

pub enum DriverInitResult {
    Files(DriverFiles, u32),
    Dirs(DriverDir, HashMap<u8, DriverUninit>)
}

pub struct DriverFiles {
    path: String,
}

pub struct DriverDir {
    path: String,
}

impl DriverUninit {
    pub fn new(path: String) -> DriverUninit {
        DriverUninit {
            path: path
        }
    }
    
    pub fn init(self) -> DriverInitResult {
        let path = self.path;
        
        println!("czytam {:?}", path);
        
        let mut files_count = 0;
        let dir_list = read_dir(Path::new(path.as_str())).unwrap();
        
        let mut map: HashMap<u8, DriverUninit> = HashMap::new();            //TODO - remove type
        
        'nextitem : for path in dir_list {

            let item = path.unwrap();
            let metadata = item.metadata().unwrap();
            
            if (metadata.is_file()) {
                files_count += 1;
                continue 'nextitem;                
            }
            
            if (metadata.is_dir()) {
                let file_name = item.file_name();
                let file_str = file_name.to_str().unwrap();
                
                if file_str.len() == 2 {
                    
                    println!("katalog {}", file_str);
                    
                    //TODO , trzeba
                    unimplemented!();

                    continue 'nextitem;
                }
                
                panic!("incorrect contents of a directory");

            }
            
            panic!("incorrect contents of a directory");
        }
        
        if map.len() > 0 {

            DriverInitResult::Dirs(DriverDir {
                path: path
            }, map)

        } else {

            DriverInitResult::Files(DriverFiles {
                path: path
            }, files_count)
        }
    }
}

impl DriverFiles {
    pub fn set(&self, hash: &Hash, content: &[u8]) {
    
        unimplemented!();
                    //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
    /*
        let path = self.path + "/" + hash.get();

        let file_opt = OpenOptions::new().write(true)
                                     .create_new(true)
                                     .open(path);

        match file_opt {
            Ok(file) => {
                //TODO - plik utworzono, można zapisać dane
                unimplemented!();
            },
            Err(err) => {
                //TODO - trzeba sprawdzić czy ten plik ma taką samą zawartość jak treść którą chcemy zapisać
                unimplemented!();
            }
        }
    */
    }
}