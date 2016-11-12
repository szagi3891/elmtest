use std::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;

use lib::blob_stor::BlobStor;
use lib::blob_stor::hash::Hash;
use lib::list_file::list_file;
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
    
    
        //TODO - tymczasowa funkcja potrzebna do testów na tej strukturze
    pub fn test(&self) {
        self.test_write();
        self.test_read();
    }

    fn test_write(&self) {
        println!("testuję coś se tam");
        
        let empty_dir = Dir::test_new_empty();
        
        let serialized = empty_dir.serialize();
        
        //"0011223344556677889900112233445566778899".as_bytes(), 
        let hash = self.stor.set(&serialized);

        println!("zserializowany obiekt {:?}", serialized);
    }
    
    fn test_read(&self) {

        match self.stor.get_str("0011223344556677889900112233445566778899") {
            Some(dane) => {
                println!("odczytany obiekt {:?}", dane);
                let dir = Dir::deserialize(dane.as_slice());
            },
            None => {
                panic!("brak rekordu");
            }
        }
    }
}
    

fn read_last(path_head: &PathBuf, stor: &BlobStor) -> Head {

    let list = list_file(path_head);

    println!("{:?}", list);

    if list.len() > 0 {
        panic!("TODO - trzeba odczytać heada z dysku");
    }

    /*
        czytaj namiar na ostatniego head-a

            jeśli go nie ma, to zainicjuj nowego pustego heada

        jeśli jest
            przeczytaj heada oraz numer wersji
    */
    //TODO - trzeba zainicjować początkową strukturę

    /*
        wszystkie pliki będą miały numer wersji oraz datę ładnie sformatowaną
        będzie łatwiej posortować
    */

    let empty_dir = Dir::new_empty();
    let empty_serialized = empty_dir.serialize();
    let empty_hash = stor.set(empty_serialized.as_slice());

    
    let hash_str = [48; 40];        //40 x '0'

    let hash = Hash::new(hash_str);

    Head::new(hash, 0)
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


