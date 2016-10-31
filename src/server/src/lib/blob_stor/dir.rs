use std::sync::RwLock;
use std::sync::Arc;
use std::mem::replace;
use std::collections::HashMap;

use lib::blob_stor::hash::Hash;
use lib::blob_stor::driver::{DriverUninit, DriverInitResult, DriverFiles, DriverDir};
use lib::blob_stor::file_counter::FileCounter;

pub struct Dir {
    inner: Arc<RwLock<DirMode>>,
    max_file: u32,
}

enum DirMode {
    Uninitialized(DriverUninit),
    ContentFiles(DriverFiles, FileCounter),
    ContentDir(DriverDir, HashMap<u8, Dir>),
}

enum DirSetCommand {
    NeedInit,
    Success,
    NeedRebuildToSubDir,
    NeedSubDir(u8),
}

enum DirGetCommand {
    NeedInit,
    Success(Vec<u8>),
}

impl Dir {

    pub fn new_uninit(driver: DriverUninit, max_file: u32) -> Dir {
        Dir {
            inner: Arc::new(RwLock::new(DirMode::Uninitialized(driver))),
            max_file: max_file,
        }
    }

    pub fn get(&mut self, hash: Hash) -> Vec<u8> {
        
        let mut count_loop = 0;
        
        loop {
            count_loop += 1;

            if (count_loop > 20) {
                panic!("too much recursion");
            }
            
            match (self.get_exec(&hash)) {
                DirGetCommand::NeedInit => {
                    self.initialize();
                },

                DirGetCommand::Success(result) => {
                    return result;
                }
            }
        }
    }
    
    pub fn set(&mut self, hash: Hash, content: &[u8]) {
        
        let mut count_loop = 0;
        
        loop {
            count_loop += 1;

            if (count_loop > 20) {
                panic!("too much recursion");
            }
            
            match (self.set_exec(&hash, content)) {
                DirSetCommand::NeedInit => {
                    self.initialize();
                },

                DirSetCommand::Success => {
                    return;
                },

                DirSetCommand::NeedRebuildToSubDir => {

                    //TODO - trzeba przebudować ten katalog zawierający pliki, na katalog zawierający podkatalogi
                    unimplemented!();
                },
                
                DirSetCommand::NeedSubDir(_) => {
                    //TODO - trzeba utworzyć podkatalog o wskazanej nazwie
                    unimplemented!();
                }
            }
        }
    }

    fn get_exec(&mut self, hash: &Hash) -> DirGetCommand {
        
        let guard = self.inner.read().unwrap();
        
        match *guard {
            DirMode::Uninitialized(_) => DirGetCommand::NeedInit,
            
            DirMode::ContentFiles(ref file_driver, ref file_counter) => {
                DirGetCommand::Success(file_driver.get(hash))
            }
            
            DirMode::ContentDir(_, _) => {
                unimplemented!();
            }
        }
    }

    fn set_exec(&mut self, hash: &Hash, content: &[u8]) -> DirSetCommand {
        
        let guard = self.inner.read().unwrap();

        match *guard {

            DirMode::Uninitialized(_) => DirSetCommand::NeedInit,

            DirMode::ContentFiles(ref file_driver, ref file_counter) => {
                
                let guard = file_counter.get_increment_guard();

                if guard.count() > self.max_file {
                    DirSetCommand::NeedRebuildToSubDir

                } else {
                    file_driver.set(hash, content);
                    guard.inc();
                    DirSetCommand::Success
                }
            },

            DirMode::ContentDir(_, _) => {
                //weź podkatalog
                //istnieje
                    //odpal metodę set na tym podkatalogu
                //nie istnieje
                    DirSetCommand::NeedSubDir(0x43)         //TODO
            },
        }
    }

    fn initialize(&mut self) {
        
        let mut guard = self.inner.write().unwrap();
        
        let new_content_opt = match *guard {
        
            DirMode::Uninitialized(ref mut driver) => {

                match driver.init() {
                    
                    DriverInitResult::Files(driver_files, files_count) => {

                        Some(DirMode::ContentFiles(driver_files, FileCounter::new(files_count)))
                    },

                    DriverInitResult::Dirs(map, driver_dir) => {

                        //TODO - odczytanie początkowej struktury katalogu
                        unimplemented!();
                    },
                }
            },
            _ => None,
        };
        
        match new_content_opt {
            Some(mut new_content) => {
                replace(&mut *guard, new_content);
            },
            None => {},
        };
    }
}