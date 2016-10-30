use std::sync::{RwLock, RwLockWriteGuard};
use std::sync::Arc;

pub struct FileCounter {
    inner: Arc<RwLock<u32>>,
}

pub struct FileCounterIncrease<'a> {
    inner: RwLockWriteGuard<'a, u32>
}

impl FileCounter {
    pub fn new(init_count: u32) -> FileCounter {
        FileCounter {
            inner: Arc::new(RwLock::new(init_count))
        }
    }
    
    pub fn get_increment_guard(&self) -> FileCounterIncrease {

        let counter = self.inner.write().unwrap();
        
        FileCounterIncrease {
            inner: counter
        }
    }
}

impl<'a> FileCounterIncrease<'a> {
    
    pub fn count(&self) -> u32 {
        self.inner.clone()
    }
}