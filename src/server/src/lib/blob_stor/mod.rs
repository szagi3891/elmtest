use std::path::PathBuf;

use lib::hash::Hash;
use lib::blob_stor::blob_stor_disk::{BlobStorDisk};

mod dir;
mod driver;
mod file_counter;
mod blob_stor_disk;

//TODO
//http://www.ncameron.org/blog/abstract-return-types-aka-%60impl-trait%60/


#[derive(Clone)]
enum BlobStorEnum {
    Disk(BlobStorDisk),
    //TODO - dodać driver mokujący na potrzeby testów
}


#[derive(Clone)]
pub struct BlobStor {
    inner: BlobStorEnum
}


impl BlobStor {

    pub fn new_blob_stor(base_path: PathBuf, max_file: u32) -> BlobStor {
        BlobStor {
            inner: BlobStorEnum::Disk(
                BlobStorDisk::new(base_path, max_file)
            )
        }
    }
    
    pub fn get(&self, hash: &Hash) -> Option<Vec<u8>> {
        match self.inner {
            BlobStorEnum::Disk(ref driver) => {
                driver.get(hash)
            }
        }
    }
    
    pub fn set(&self, content: &[u8]) -> Hash {
        match self.inner {
            BlobStorEnum::Disk(ref driver) => {
                driver.set(content)
            }
        }
    }
}