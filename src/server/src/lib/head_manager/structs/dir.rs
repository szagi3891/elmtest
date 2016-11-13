use std::collections::HashMap;

use lib::blob_stor::BlobStor;
use lib::blob_stor::hash::Hash;

pub struct Dir {
    stor: BlobStor,
    hash: Hash,
    list: HashMap<String, DirItem>,
}

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
    
    pub fn new_empty(stor: &BlobStor) -> Dir {

        //let empty_serialized = empty_dir.serialize();
        //empty_serialized.as_slice()

        let empty_hash = stor.set("".as_bytes());
        
        Dir {
            stor: stor.clone(),
            hash: empty_hash,
            list: HashMap::new()
        }
    }
    
    pub fn hash(&self) -> Hash {
        self.hash.clone()
    }
    
    pub fn serialize(&self) -> Vec<u8> {

        let mut sort_keys = Vec::new();
        
        for (key, _) in self.list.iter() {
            sort_keys.push(key);
        }
        
        sort_keys.sort();

        let mut out: Vec<u8> = Vec::new();

        for key_name in sort_keys {
            
            let val = self.list.get(key_name).unwrap();

            val.hash.serialize(&mut out);
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
            
            for item in key_name.as_bytes() {
                out.push(*item);
            }

            out.push(10);
        }

        out
    }
    
    pub fn deserialize(hash_addr: Hash, stor: BlobStor, data_in: &[u8]) -> Dir {
        
        let mut map = HashMap::new();
        
        let mut data_wsk = data_in;

        loop {
            match read_line(data_wsk) {
                Some((line, rest)) => {

                    let (name, item) = create_item(line);
                    data_wsk = rest;

                    match map.insert(name, item) {
                        Some(_) => {
                            panic!("zduplikowane rekordy");
                        },
                        _ => {},
                    };
                },
                None => {
                    return Dir {
                        stor: stor,
                        hash: hash_addr,
                        list: map
                    };
                }
            }
        }        
    }
}

fn read_line(data_in: &[u8]) -> Option<(&[u8], &[u8])> {
    
    if data_in.len() == 0 {
        return None;
    }

    match data_in.iter().position(|r| *r == 10) {
        Some(index) => {
            let first = &data_in[0..index];
            let rest = &data_in[index+1..];
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
