use std::sync::RwLock;
use std::sync::Arc;

pub struct BlobStor<'a> {
    base_path: &'a str,
    root: Dir,
}

struct Dir {
    inner: Arc<RwLock<DirMode>>,
}

enum DirMode {
    Uninitialized,
    ContentFiles,
    ContentDir,
}

impl<'a> BlobStor<'a> {

    pub fn new(base_path: &'a str) -> BlobStor<'a> {
        
        //TODO - sprawdź czy ten katalog istnieje - jeśli nie to go stwórz
        
        BlobStor {
            base_path: base_path,
            root : Dir {
                inner: Arc::new(RwLock::new(DirMode::Uninitialized)),
            },
        }
    }

    pub fn get(&mut self, hash: &'a [u8]) -> String {
        self.root.get(hash)
    }
    
    pub fn set(&mut self, hash: &[u8], content: &[u8]) {
        self.root.set(hash, content)
    }
}

impl Dir {

    fn get(&mut self, hash: &[u8]) -> String {
        return "dasdas".to_string();
    }
    
    fn set(&mut self, hash: &[u8], content: &[u8]) {
    }
}