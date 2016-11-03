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
    Success(Option<Vec<u8>>),
}

impl Dir {

    pub fn new_uninit(driver: DriverUninit, max_file: u32) -> Dir {
        Dir::new_from_mode(DirMode::Uninitialized(driver), max_file)
    }

    fn new_from_mode(dir_mode: DirMode, max_file: u32) -> Dir {
        Dir {
            inner: Arc::new(RwLock::new(dir_mode)),
            max_file: max_file,
        }
    }

    pub fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        
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
    
    pub fn set(&self, hash: &Hash, content: &[u8]) {
        
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
                    self.transformToDirDriver();
                },
                
                DirSetCommand::NeedSubDir(prefix) => {
                    self.createSubDir(prefix);
                }
            }
        }
    }

    fn get_exec(&self, hash: &Hash) -> DirGetCommand {
        
        let guard = self.inner.read().unwrap();
        
        match *guard {
            DirMode::Uninitialized(_) => DirGetCommand::NeedInit,
            
            DirMode::ContentFiles(ref file_driver, ref file_counter) => {
                let reader_lock = file_counter.get_reader_lock();
                let content = file_driver.get(hash);
                reader_lock.free();
                DirGetCommand::Success(Some(content))
            }
            
            DirMode::ContentDir(ref dir_driver, ref map) => {
                let level = dir_driver.get_level();
                let prefix = hash.get_prefix(level);
                
                let result = match map.get(&prefix) {
                    Some(item) => item.get(hash),
                    None => None,
                };

                DirGetCommand::Success(result)
            }
        }
    }

    fn set_exec(&self, hash: &Hash, content: &[u8]) -> DirSetCommand {
        
        let mut guard = self.inner.read().unwrap();

        match *guard {

            DirMode::Uninitialized(_) => DirSetCommand::NeedInit,

            DirMode::ContentFiles(ref file_driver, ref file_counter) => {
                
                let guard = file_counter.get_increment_guard();

                if guard.count() >= self.max_file {
                    DirSetCommand::NeedRebuildToSubDir

                } else {
                    if file_driver.set(hash, content) {
                        guard.inc();
                    }

                    DirSetCommand::Success
                }
            },

            DirMode::ContentDir(ref dir_driver, ref map) => {
                
                let level = dir_driver.get_level();
                let prefix = hash.get_prefix(level);

                match map.get(&prefix) {
                    Some(sub_dir) => {
                        sub_dir.set(hash, content);
                        DirSetCommand::Success
                    },
                    None => DirSetCommand::NeedSubDir(prefix)
                }
            },
        }
    }

    fn initialize(&self) {
        
        let mut guard = self.inner.write().unwrap();
        
        let new_content_opt = match *guard {
        
            DirMode::Uninitialized(ref mut driver) => {

                match driver.init() {
                    
                    DriverInitResult::Files(driver_files, files_count) => {
                        Some(DirMode::ContentFiles(driver_files, FileCounter::new(files_count)))
                    },

                    DriverInitResult::Dirs(driver_dir, mut map_drivers) => {
                        
                        let mut map_dir = HashMap::new();
                
                        for (key, item_driver) in map_drivers.drain() {

                            let dir_mode = DirMode::Uninitialized(item_driver);
                            let dir_item = Dir::new_from_mode(dir_mode, self.max_file);
                            
                            map_dir.insert(key, dir_item);
                            
                                            //TODO - sprawdzać czy na wyjściu jest None
                            /*
                            if Some(xx) = map_dir.insert(key, dir_item) {
                                panic!("spodziewano się None");
                            }
                            */
                        }
                        
                        Some(DirMode::ContentDir(driver_dir, map_dir))
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
    
    fn transformToDirDriver(&self) {
        let mut guard = self.inner.write().unwrap();
        
        let new_content_opt = match *guard {
            DirMode::ContentFiles(ref file_driver, _) => {
                
                                                        //TODO - Trzeba usprawnić tą funkcję, żeby od razy właściwa mapa była zwaracana
                let (dir_driver, mut map) = file_driver.transformToDir();
                
                let mut map_dir = HashMap::new();
                
                for (key, (file_driver, count)) in map.drain() {
                    
                    let value = DirMode::ContentFiles(file_driver, FileCounter::new(count));
                    let dir = Dir::new_from_mode(value, self.max_file);
                    map_dir.insert(key, dir);
                }
                
                Some(DirMode::ContentDir(dir_driver, map_dir))
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

    fn createSubDir(&self, prefix: u8) {
        let mut guard = self.inner.write().unwrap();
        
        let new_content_opt = match *guard {
            DirMode::ContentDir(ref dir_driver, ref mut map) => {
                
                match map.get(&prefix) {
                    Some(_) => {
                        return;
                    },
                    _ => {}
                };
                
                let files_driver = dir_driver.create_dir(prefix);
                let dir_mode = DirMode::ContentFiles(files_driver, FileCounter::new(0));
                let dir_item = Dir::new_from_mode(dir_mode, self.max_file);
                
                map.insert(prefix, dir_item);

                None
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