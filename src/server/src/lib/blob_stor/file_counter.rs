use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use std::sync::Arc;

pub struct FileCounter {
    inner: Arc<RwLock<u32>>,
}

pub struct FileCounterIncrease<'a> {
    inner: RwLockWriteGuard<'a, u32>
}


pub struct FileCounterReaderLock<'a> {
    inner: RwLockReadGuard<'a, u32>
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
    
    pub fn get_reader_lock(&self) -> FileCounterReaderLock {

        let counter = self.inner.read().unwrap();
        
        FileCounterReaderLock {
            inner: counter
        }
    }
}

impl<'a> FileCounterIncrease<'a> {
    
    pub fn count(&self) -> u32 {
        self.inner.clone()
    }
    
    pub fn inc(mut self) {
        *self.inner += 1;
    }
}

impl<'a> FileCounterReaderLock<'a> {
    pub fn free(self) {
    }
}