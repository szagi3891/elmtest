use std::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;
use chrono::offset::utc::UTC;
use std::collections::HashSet;

use lib::blob_stor::{BlobStor};
use lib::hash::Hash;
use lib::hex::{convert_from_hex};
use lib::fs::list_file::list_file;
use lib::fs::save_file::save_file;
use lib::fs::get_file::get_file;
use lib::head_manager::structs::head::Head;
use lib::head_manager::structs::ionode::Ionode;
use lib::head_manager::structs::iopath::Iopath;

mod structs;

pub struct HeadManager {
	inner: Arc<RwLock<Head>>,
    stor: BlobStor,
    path_head: PathBuf,
}

impl HeadManager {
	pub fn new(base_path: PathBuf, max_file: u32) -> HeadManager {
        
        let path_blob = make_path(&base_path, "blob");              //katalog na bloby
        let path_head = make_path(&base_path, "head");              //katalog z aktualnymi head-ami
        
        let stor = BlobStor::new_blob_stor(path_blob, max_file);

        let last_head = read_last(&path_head, &stor);

		HeadManager {
			inner: Arc::new(RwLock::new(last_head)),
			stor: stor,
            path_head: path_head,
		}
	}

    /*
    HeadManagerError
        wszystkie błędy użytkownika, które potem raportujemy jako odpowiedź
    */
/*
    pub get_list(head: Hash, path: Vec<String>) -> Result<Vec<String>, HeadManagerError> {
        //pobiera listę plików w żądanej ścieżce
    }
*/
    
    
    //#[test]
    //może być prywatna taka metoda

        //TODO - tymczasowa funkcja potrzebna do testów na tej strukturze
    pub fn test(&self) {
        let hash = self.test_write();
        self.test_read(hash);
    }

    fn test_write(&self) -> Hash {
        
        let empty_dir = Ionode::new_empty_dir(&self.stor);
        let hash = empty_dir.hash();

        let path = Iopath::new(vec!("testowy".to_string()));
        let new_content = "content inny".as_bytes();
        let new_item = empty_dir.new_file(path, new_content);

        hash
    }
    
    fn test_read(&self, hash: Hash) {

        match self.stor.get(&hash) {
            Some(dane) => {
                let dir = Ionode::deserialize(hash, &self.stor, dane.as_slice());
            },
            None => {
                panic!("brak rekordu");
            }
        }
    }
}
    

fn read_last(path_head: &PathBuf, stor: &BlobStor) -> Head {

    let list = list_file(path_head);

    if list.len() > 0 {
        
        let (_, the_last_path) = find_the_latest(list);

        let head_hash = read_head_hash(the_last_path);
        let head_data = stor.get(&head_hash).unwrap();
        
        return Head::deserialize(head_hash, stor, head_data.as_slice());
    }

    let empty_dir = Ionode::new_empty_dir(stor);

    let start_version = 1;
    
    let head = Head::new(stor, start_version, empty_dir.hash());
    
    let current = UTC::now().format("%Y-%m-%d_%H-%M-%S");
    let file_name = format!("{}__{}", start_version, current);
    
    let mut path_save = path_head.clone();
    path_save.push(file_name);

    save_file(path_save.as_path(), head.get_hash().to_hex().as_bytes()).unwrap();
    
    head
}

fn read_head_hash(the_last_path: PathBuf) -> Hash {
    
    let data = get_file(the_last_path.as_path()).unwrap();
    let data_slice = data.as_slice();   
    let data_bytest = convert_from_hex(data_slice);
    
    Hash::new(data_bytest)
}


fn find_the_latest(list: Vec<PathBuf>) -> (u32, PathBuf) {

    let mut out: Option<(u32, PathBuf)> = None;
    let mut already_occurred: HashSet<u32> = HashSet::new();
    
    for path_item in list {

        let path_item_clone = path_item.clone();            //TODO - do usunięcia ten klon
        let file_name = path_item_clone.file_name().unwrap().to_str().unwrap();

        let mut chunks = file_name.split("__");

        let prefix1 = chunks.next();
        let prefix2 = chunks.next();
        let end = chunks.next();

        match (prefix1, prefix2, end) {
            (Some(prefix), Some(_), None) => {

                let prefix_value = u32::from_str_radix(prefix, 10).unwrap();

                if already_occurred.insert(prefix_value) == false {
                    panic!("zduplikowany numer wersji: {:?}", prefix_value);
                }
                
                let need_replace = match out {
                    None => true,
                    Some((max, _)) => max < prefix_value,
                };

                if need_replace {
                    out = Some((prefix_value, path_item));
                }
            },
            _ => {
                panic!("nazwa pliku nie pasuje do wzorca: {:?}", file_name);
            },
        };
    }
    
    out.unwrap()
}

fn make_path(base_path: &PathBuf, sub_dir: &str) -> PathBuf {
    let mut path = base_path.clone();
    path.push(sub_dir);
        
    let result = fs::create_dir(&path);
    
    match result {
        Ok(()) => {},
        Err(error) => {
            match error.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("problem przy tworzeniu katalogu");
                }
            }
        }
    }
    
    return path;
}


