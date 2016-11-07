use std::collections::HashMap;
use rustc_serialize::json;

use lib::blob_stor::hash::Hash;

pub struct Dir {
    list: HashMap<String, DirItem>,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
enum KindType {
    File,
    Dir
}

#[derive(Clone)]
struct DirItem {
    kind: KindType,
    hash: Hash,
}

#[derive(RustcDecodable, RustcEncodable)]
struct DirSerializeItem {
    kind: KindType,
    hash: String,
}

impl Dir {
    pub fn new_empty() -> Dir {
        Dir {
            list: HashMap::new()
        }
    }
    
    pub fn test_new_empty() -> Dir {

        let hash_str = [48; 40];        //40 x '0'
        

        let test_hash = Hash::new(hash_str);
        
        let test_item = DirItem {
            kind: KindType::File,
            hash: test_hash,
        };
        
        let mut hash_test = HashMap::new();
        
        let test_item2 = test_item.clone();

        hash_test.insert("cosik.txt".to_string(), test_item);
        hash_test.insert("blablabla.jpg".to_string(), test_item2);
        
        Dir {
            list: hash_test
        }
    }

    pub fn to_string(&self) -> String {
        
        let mut map = HashMap::new();
        
        for (key, val) in self.list.iter() {

            let item = DirSerializeItem {
                kind: val.kind.clone(),
                hash: val.hash.to_string(),
            };

            let key_clone = key.clone();
            map.insert(key_clone, item);
        }
        
        let encoded = json::encode(&map).unwrap();

        encoded
    }
}