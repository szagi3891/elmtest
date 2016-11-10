use std::collections::HashMap;
use std::io;

use lib::blob_stor::hash::Hash;

pub struct Dir {
    list: HashMap<String, DirItem>,
}

                //#[derive(RustcDecodable, RustcEncodable, )]

#[derive(Clone)]
enum KindType {
    File,
    Dir
}

#[derive(Clone)]
struct DirItem {
    kind: KindType,
    hash: Hash,
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

    pub fn serialize(&self, out: &mut Vec<u8>) {
        /*
        let sign = "dir".as_bytes();
        
        for item in sign {
            out.push(*item)
        }
        
        out.push(10);
        */

        for (key, val) in self.list.iter() {

            val.hash.serialize(out);
            out.push(32);
            
            match val.kind {
                KindType::File => {
                    out.push(48);
                },
                KindType::Dir => {
                    out.push(49);
                },
            }

            out.push(32);
            
            for item in key.as_bytes() {
                out.push(*item);
            }

            out.push(10);
        }
    }
    
    pub fn deserialize(data_in: &[u8]) -> Dir {
        //TODO - tymczasowo
        Dir {
            list: HashMap::new()
        }
    }
}