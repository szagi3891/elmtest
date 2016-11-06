use std::collections::HashMap;
use rustc_serialize::json;

use lib::blob_stor::hash::Hash;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Dir {
    list: HashMap<String, DirItem>,
}

#[derive(RustcDecodable, RustcEncodable)]
enum KindType {
    File,
    Dir
}

#[derive(RustcDecodable, RustcEncodable)]
struct DirItem {
    kind: KindType,
    head: Hash,
}


/*
https://serde.rs/enum-number.html
    serializowanie enuma
*/

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
            head: test_hash,
        };
        
        let mut hash_test = HashMap::new();
        
        hash_test.insert("cosik.txt".to_string(), test_item);
        
        Dir {
            list: hash_test
        }
    }
    
    pub fn to_string(&self) -> String {
        
        let encoded = json::encode(&self).unwrap();
        
        encoded
    }
}