use std::path::PathBuf;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

use lib::hash::Hash;
use lib::hex::{convert_from_hex};
use lib::blob_stor::dir::Dir;
use lib::blob_stor::driver::DriverUninit;


#[derive(Clone)]
pub struct BlobStorDisk {
    root: Dir,
}

impl BlobStorDisk {

    pub fn new(base_path: PathBuf, max_file: u32) -> BlobStorDisk {

        let driver = DriverUninit::new(base_path);

        BlobStorDisk {
            root : Dir::new_uninit(driver, max_file),
        }
    }

    pub fn get_str(&self, hash_str: &str) -> Option<Vec<u8>> {
        
        let hash_slice = hash_str.as_bytes();
        let hash_bin = convert_from_hex(hash_slice);
        let hash = Hash::new(hash_bin);
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
        
        let hash_bin = convert_from_hex(hex.as_bytes());
        
        let hash = Hash::new(hash_bin);
        self.root.set(&hash, content);
        
        hash
    }
}
