use std::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;

use lib::blob_stor::BlobStor;
use lib::head_manager::structs::head::Head;

mod structs;

pub struct HeadManager {
	inner: Arc<RwLock<Head>>,
    stor: BlobStor
}

impl HeadManager {
	pub fn new(base_path: PathBuf, max_file: u32) -> HeadManager {
        
        /*
            blob - katalog na bloby
            head - katalog w którym będą znajdowały się informacje o headach
                z każdą zmianą, będzie się tworzył nowy plik w tym miejscu z kolejną datą
        */

        //TODO - trzeba zainicjować początkową strukturę
        
        let hash_str = [48; 40];        //40 x '0'
        let version_start = 0;
        
		HeadManager {
			inner: Arc::new(RwLock::new(Head::new(hash_str, version_start))),
			stor: BlobStor::new(base_path, max_file),
		}
	}
    
        //TODO - tymczasowa funkcja potrzebna do testów na tej strukturze
    pub fn test(&self) {
        println!("testuję coś se tam");
    }
}
