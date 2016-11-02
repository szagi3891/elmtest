use std::fs::create_dir;
use std::path::PathBuf;

mod dir;
mod hash;
mod driver;
mod file_counter;
mod hex;

use lib::blob_stor::hash::Hash;
use lib::blob_stor::dir::Dir;
use lib::blob_stor::driver::DriverUninit;

pub struct BlobStor {
    root: Dir,
}

impl BlobStor {

    pub fn new<'a>(base_path: PathBuf, max_file: u32) -> BlobStor {

        let driver = DriverUninit::new(base_path);
        
        BlobStor {
            root : Dir::new_uninit(driver, max_file),
        }
    }

    pub fn get(&mut self, hash_slice: &[u8]) -> Vec<u8> {
        
        let hash = Hash::from_bytes(hash_slice);
        self.root.get(&hash)
    }
    
    pub fn set(&mut self, hash_slice: &[u8], content: &[u8]) {
        
        let hash = Hash::from_bytes(hash_slice);
        self.root.set(&hash, content)
    }
}
