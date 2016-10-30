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
    None,
    Uninitialized(DriverUninit),
    ContentFiles(DriverFiles, FileCounter),
    ContentDir(DriverDir, HashMap<u8, Dir>),
}

enum DirSetCommand {
    NeedInit,
    SetSuccess,
    NeedRebuildToSubDir,
    NeedSubDir(u8),
}

impl Dir {

    pub fn new_uninit(driver: DriverUninit, max_file: u32) -> Dir {
        Dir {
            inner: Arc::new(RwLock::new(DirMode::Uninitialized(driver))),
            max_file: max_file,
        }
    }

    pub fn get(&mut self, hash: Hash) -> String {
        //TODO
        unimplemented!();
    }
    
    pub fn set(&mut self, hash: Hash, content: &[u8]) {
        
        let mut count_loop = 0;
        
        loop {
            count_loop += 1;

            if (count_loop > 10) {
                panic!("too much recursion");
            }
            
            match (self.set_exec(&hash, content)) {
                DirSetCommand::NeedInit => {
                    self.initialize();
                },

                DirSetCommand::SetSuccess => {
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

    fn set_exec(&mut self, hash: &Hash, content: &[u8]) -> DirSetCommand {
        
        let guard = self.inner.read().unwrap();

        match *guard {

            DirMode::None => {
                panic!("incorrect branch");
            },

            DirMode::Uninitialized(_) => DirSetCommand::NeedInit,

            DirMode::ContentFiles(ref file_driver, ref file_counter) => {
                
                let guard = file_counter.get_increment_guard();

                if guard.count() > self.max_file {
                    DirSetCommand::NeedRebuildToSubDir

                } else {
                    file_driver.set(hash, content);
                    guard.inc();
                    DirSetCommand::SetSuccess
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
        
        let mut content = replace(&mut *guard, DirMode::None);
        
        if let DirMode::Uninitialized(driver) = content {

            match driver.init() {
                DriverInitResult::Files(driver_files, files_count) => {
                    
                    replace(&mut *guard, DirMode::ContentFiles(driver_files, FileCounter::new(files_count)));
                    return;
                },

                DriverInitResult::Dirs(map, driver_dir) => {
                    
                    //TODO - odczytanie początkowej struktury katalogu
                    unimplemented!();
                    return;
                },
            }
        }
        
        replace(&mut *guard, content);
    }
}