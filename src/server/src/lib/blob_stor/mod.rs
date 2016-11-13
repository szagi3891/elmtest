use std::path::PathBuf;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

use lib::blob_stor::hash::Hash;
use lib::blob_stor::dir::Dir;
use lib::blob_stor::driver::DriverUninit;


mod dir;
pub mod hash;
mod driver;
mod file_counter;
mod hex;

#[derive(Clone)]
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

    pub fn get_str(&self, hash_str: &str) -> Option<Vec<u8>> {
        
        let hash_slice = hash_str.as_bytes();
        let hash = Hash::from_bytes(hash_slice);
        self.root.get(&hash)
    }
    
    pub fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        self.root.get(&hash)
    }
  
/*
    pub fn set_str(&self, content: &str) -> Hash {
        //TODO
    }
*/
    pub fn set(&self, content: &[u8]) -> Hash {
                
        let mut hasher = Sha1::new();
        
        //hasher.input_str(content);
        hasher.input(content);
        
        let hex = hasher.result_str();
        
        let hash = Hash::from_bytes(hex.as_bytes());
        self.root.set(&hash, content);
        
        hash
    }
}
