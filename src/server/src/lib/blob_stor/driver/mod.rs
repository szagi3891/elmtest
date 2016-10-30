use std::fs::read_dir;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use lib::blob_stor::hash::Hash;

pub struct DriverUninit {
    path: PathBuf,
}

pub enum DriverInitResult {
    Files(DriverFiles, u32),
    Dirs(DriverDir, HashMap<u8, DriverUninit>)
}

pub struct DriverFiles {
    path: PathBuf,
}

pub struct DriverDir {
    path: PathBuf,
}

impl DriverUninit {
    pub fn new(path: PathBuf) -> DriverUninit {
        DriverUninit {
            path: path
        }
    }
    
    pub fn init(self) -> DriverInitResult {
        let path = self.path;
        
        println!("czytam {:?}", path);
        
        let mut files_count = 0;
        let dir_list = read_dir(path.as_path()).unwrap();
        
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

                    //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new

        let mut path = self.path.clone();
        path.push(hash.as_str());


        let mut file_opt = OpenOptions::new().write(true)
                                     .create_new(true)
                                     .open(path);

        match file_opt {
            Ok(mut file) => {
                
                file.write_all(content).unwrap();                
            },
            Err(err) => {
                //TODO - trzeba sprawdzić czy ten plik ma taką samą zawartość jak treść którą chcemy zapisać
                unimplemented!();
            }
        }
    }
}