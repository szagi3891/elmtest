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
        
        let mut map = HashMap::new();
        
        let mut data_wsk = data_in;

        loop {
            match read_line(data_wsk) {
                Some((line, rest)) => {

                    let (name, item) = create_item(line);
                    data_wsk = rest;

                    match map.insert(name, item) {
                        None => {
                            panic!("zduplikowane rekordy");
                        },
                        _ => {},
                    };
                },
                None => {
                    return Dir {
                        list: map
                    };
                }
            }
        }        
    }
}

fn read_line(data_in: &[u8]) -> Option<(&[u8], &[u8])> {
    
    if (data_in.len() == 0) {
        return None;
    }

    match data_in.iter().position(|r| *r == 10) {
        Some(index) => {
            let first = &data_in[0..index-1];
            let rest = &data_in[index..];
            Some((first, rest))
        },
        None => {
            panic!("nieprawidłowe dane");
        }
    }
}


fn create_item(line: &[u8]) -> (String, DirItem) {
    
    let hash_data = &line[0..40];
    //40 pomijamy - to jest spacja
    let item_type = line[41];
    //42 pomijamy - spacja
    let name = &line[43..];

    let hash = Hash::from_bytes(hash_data);
    
    let kind = match item_type {
        48 => KindType::File,
        49 => KindType::Dir,
        _ => panic!("nieprawidłowe dane"),
    };
    
    let mut name_vec = Vec::new();
    name_vec.extend_from_slice(name);

    let name_str = String::from_utf8(name_vec).unwrap();
    
    (name_str, DirItem{
        kind: kind,
        hash: hash,
    })
}
