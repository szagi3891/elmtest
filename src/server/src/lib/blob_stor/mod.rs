mod dir;
mod hash;

use lib::blob_stor::hash::Hash;
use lib::blob_stor::dir::Dir;

pub struct BlobStor<'a> {
    base_path: &'a str,
    root: Dir,
}

impl<'a> BlobStor<'a> {

    pub fn new(base_path: &'a str) -> BlobStor<'a> {
        
        //TODO - sprawdź czy ten katalog istnieje - jeśli nie to go stwórz
        
        BlobStor {
            base_path: base_path,
            root : Dir::new_uninit(),
        }
    }

    pub fn get(&mut self, hash: &'a [u8]) -> String {
        self.root.get(Hash::new(hash))
    }
    
    pub fn set(&mut self, hash: &[u8], content: &[u8]) {
        self.root.set(Hash::new(hash), content)
    }
}
