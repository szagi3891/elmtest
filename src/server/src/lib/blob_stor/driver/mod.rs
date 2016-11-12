use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

use lib::blob_stor::hash::Hash;
use lib::blob_stor::driver::init_dir::{init_dir, DriverInitDirResult};
use lib::blob_stor::driver::set_file::set_file;
use lib::blob_stor::driver::get_file::get_file;
use lib::fs::list_file::list_file;
use lib::blob_stor::hex::to_hex;

mod init_dir;
mod set_file;
mod get_file;

pub struct DriverUninit {
    path: PathBuf,
    level: u8,
}

pub struct DriverFiles {
    path: PathBuf,
    level: u8,
}

pub struct DriverDir {
    path: PathBuf,
    level: u8,
}

pub enum DriverInitResult {
    Files(DriverFiles, u32),
    Dirs(DriverDir, HashMap<u8, DriverUninit>)
}

impl DriverUninit {
    pub fn new(path: PathBuf) -> DriverUninit {
        DriverUninit {
            path: path,
            level: 0,
        }
    }
    
    pub fn init(&mut self) -> DriverInitResult {
        
        match init_dir(self.path.as_path()) {
            
            DriverInitDirResult::Files(files_count) => {
                
                let driver = DriverFiles {
                    path: self.path.clone(),
                    level: self.level,
                };
                
                DriverInitResult::Files(driver, files_count)
            },
            
            DriverInitDirResult::Dirs(mut map) => {
                
                let driver = DriverDir {
                    path: self.path.clone(),
                    level: self.level,
                };
                
                /*
                    TODO
                        
                    rozszerzyć funkcję init_dir o drugi argument który będzie generyczną funkcją mapującą
                    dzięki temu ta funkcja będzie mogła od razu zwrócić właściwą mapę z odpowiednimi typami
                */
                
                let mut new_map = HashMap::new();
                
                for (key, path) in map.drain() {
                    
                    let driver = DriverUninit{
                        path: path,
                        level: self.level+1
                    };
                    
                    new_map.insert(key, driver);
                }
                
                DriverInitResult::Dirs(driver, new_map)
            },
        }
    }
}

impl DriverFiles {
    pub fn set(&self, hash: &Hash, content: &[u8]) -> bool {

        let mut path = self.path.clone();
        hash.add_to_path(&mut path);

        set_file(path.as_path(), content)
    }
    
    pub fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        
        let mut path = self.path.clone();
        hash.add_to_path(&mut path);

        get_file(path.as_path())
    }
    
    //TODO - wynieść tą metodę do osobnego pliku

    pub fn transformToDir(&self) -> (DriverDir, HashMap<u8, (DriverFiles, u32)>) {
        
        let list = list_file(self.path.as_path());
        
        let mut counters: HashMap<u8, (DriverFiles, u32)> = HashMap::new();
        
        for item_from_path in list {

            let item_name = item_from_path.file_name().unwrap().to_str().unwrap();
            
            let hash = Hash::from_bytes(item_name.as_bytes());
            let prefix = hash.get_prefix(self.level);
            
            let mut path_to = self.path.clone();
            path_to.push(to_hex(&[prefix]));
            
            let mut item_path_to = path_to.clone();
            item_path_to.push(item_name);
            
            let need_create = match counters.get_mut(&prefix) {
                Some(&mut (_, ref mut count)) => {
                    *count += 1;
                    false
                },
                None => true,
            };
            
            if need_create {
                
                fs::create_dir(&path_to).unwrap();
                
                let driver = DriverFiles {
                    path: path_to.clone(),
                    level: self.level + 1,
                };
                
                counters.insert(prefix, (driver, 1));
            }

            fs::rename(&item_from_path, &item_path_to).unwrap();
        }
    
        let self_driver = DriverDir {
            path: self.path.clone(),
            level: self.level,
        };

        return (self_driver, counters);
    }
}

impl DriverDir {
    pub fn get_level(&self) -> u8 {
        self.level
    }
    
    pub fn create_dir(&self, prefix: u8) -> DriverFiles {
        
        let mut path = self.path.clone();
        path.push(to_hex(&[prefix]));
        
        fs::create_dir(path.as_path()).unwrap();

        DriverFiles {
            path: path,
            level: self.level + 1,
        }
    }
                
}