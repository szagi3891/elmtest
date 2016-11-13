use std::path::PathBuf;
use std::str;
use std::u8;

use lib::blob_stor::hex::to_hex;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Hash {
    hash: [u8; 20],
}

impl Hash {
    pub fn new(hash: [u8; 40]) -> Hash {
        
        Hash {
            hash: convertToHex(&hash[..])
        }
    }
    
    pub fn from_bytes(hash: &[u8]) -> Hash {
        
        if hash.len() != 40 {
            panic!("nieprawidłowa długość {:?}", hash.len());
        }
        
        Hash {
            hash: convertToHex(hash)
        }
    }
    
    pub fn from_vec(data: &Vec<u8>) -> Hash {
        let data_slice = data.as_slice();
        Hash::from_bytes(data_slice)
    }

    //Hash::from_vec(head_data);
    
    pub fn add_to_path(&self, path: &mut PathBuf) {
        
        let slice = &self.hash[..];
                                                //TODO - użyć lepszej metody do konwersji na hex
        let out = to_hex(slice);
        
        assert!(out.len() == 40);
        
        path.push(out);
    }

    pub fn get_prefix(&self, pos: u8) -> u8 {
        self.hash[pos as usize]
    }
    
                                                //TODO - dobrze byłoby to zrobić bez tylu alokacji przy serializowaniu danych
    pub fn to_string(&self) -> String {
        
        to_hex(&self.hash)
    }
    
    pub fn serialize(&self, out: &mut Vec<u8>) {        

        let hash_hex = to_hex(&self.hash);

        for item in hash_hex.as_bytes() {
            out.push(*item);
        }
    }
}

fn convertToHex(hash: &[u8]) -> [u8; 20] {
    
    let mut out = [0; 20];
    
    for index in 0..20 {
        let (_, tail) = hash.split_at(2 * index);
        let (range, _) = tail.split_at(2);
        
        out[index] = fromHex(range);
    }
    
    out
}

fn fromHex(slice: &[u8]) -> u8 {
    
    let slice_str = str::from_utf8(&slice).unwrap();
    u8::from_str_radix(slice_str, 16).unwrap()
}