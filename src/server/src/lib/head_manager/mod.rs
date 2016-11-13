use std::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;
use chrono::offset::utc::UTC;

use lib::blob_stor::BlobStor;
use lib::blob_stor::hash::Hash;
use lib::fs::list_file::list_file;
use lib::fs::save_file::save_file;
use lib::fs::get_file::get_file;
use lib::head_manager::structs::head::Head;
use lib::head_manager::structs::dir::Dir;

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
        
        let stor = BlobStor::new(path_blob, max_file);

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
    /*
        wczytaj obiekt head
            przeczytaj namiar na root-a
                wczytaj ten obiekt
                    itdd, aż do otrzymania
    */
    
    
    /*
        //TODO - tymczasowa funkcja potrzebna do testów na tej strukturze
    pub fn test(&self) {
        let hash = self.test_write();
        self.test_read(&hash);
    }

    fn test_write(&self) -> Hash {
        println!("testuję coś se tam");
        
        let empty_dir = Dir::test_new_empty();
        
        let serialized = empty_dir.serialize();
        
        let hash = self.stor.set(&serialized);

        println!("zserializowany obiekt {:?}", serialized);
        
        hash
    }
    
    fn test_read(&self, hash: &Hash) {

        match self.stor.get(hash) {
            Some(dane) => {
                println!("odczytany obiekt {:?}", dane);
                let dir = Dir::deserialize(dane.as_slice());
            },
            None => {
                panic!("brak rekordu");
            }
        }
    }
    */
}
    

fn read_last(path_head: &PathBuf, stor: &BlobStor) -> Head {

    let list = list_file(path_head);

    if list.len() > 0 {
        
        let (version, the_last_path) = find_the_latest(list);

        let last_data = get_file(the_last_path.as_path()).unwrap();
        let head_hash = Hash::from_vec(&last_data);

        let head_data = stor.get(&head_hash).unwrap();
        
        return Head::deserialize(stor, head_data.as_slice());
    }

    let empty_dir = Dir::new_empty(stor);

    let start_version = 1;
    
    let head = Head::new(stor, start_version, empty_dir.hash());
    
    let current = UTC::now().format("%Y-%m-%d_%H-%M-%S");
    let file_name = format!("{}__{}", start_version, current);
    
    let mut path_save = path_head.clone();
    path_save.push(file_name);

    save_file(path_save.as_path(), head.get_hash().to_string().as_bytes()).unwrap();
    
    head
}


fn find_the_latest(list: Vec<PathBuf>) -> (u32, PathBuf) {

    let mut out: Option<(u32, PathBuf)> = None;
    
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


