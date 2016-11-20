use std::path::PathBuf;

use lib::hash::Hash;
use lib::blob_stor::blob_stor_disk::{BlobStorDisk};

mod dir;
mod driver;
mod file_counter;
mod blob_stor_disk;

trait BlobStorTrait {
    fn set(&self, &[u8]) -> Hash;
    fn get(&self, &Hash) -> Option<Vec<u8>>;
}

pub struct BlobStor {
    inner: Box<BlobStorTrait + Send + Sync>
}

impl BlobStor {
    pub fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        self.get(hash)
    }
    
    pub fn set(&self, content: &[u8]) -> Hash {
        self.set(content)
    }
    
    pub fn clone(&self) -> BlobStor {
        BlobStor {
            inner: self.inner.clone()
        }
    }
}

//pub type BlobStor = Box<BlobStorTrait + Send + Sync>;

pub fn new_blob_stor(base_path: PathBuf, max_file: u32) -> BlobStor {
    BlobStor {
        inner: BlobStorDisk::new(base_path, max_file)
    }
}


/*
pub fn clone_stor(stor: &BlobStor) -> BlobStor {
}
*/

impl BlobStorTrait for BlobStorDisk {
    fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        self.get(hash)
    }
    
    fn set(&self, content: &[u8]) -> Hash {
        self.set(content)
    }
}