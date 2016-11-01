use std::path::PathBuf;
use std::str;

use lib::blob_stor::hex::to_hex;

pub struct Hash<'a> {
    hash: &'a [u8],
}

impl<'a> Hash<'a> {
    pub fn new(hash: &'a [u8]) -> Hash {
        
        //TODO - trzeba sprawdzić czy to jest poprawny hex
        
        Hash {
            hash: hash
        }
    }
    
    pub fn make_path(&self, path: PathBuf) -> PathBuf {
        let hash_str = str::from_utf8(self.hash).unwrap();
        path.push(hash_str);
        path
    }

    pub fn get_prefix(&self) -> (u8, Hash) {
        
        if (self.hash.len() > 2) {
            
            let char1 = hexToDigit(self.hash[0]);
            let char2 = hexToDigit(self.hash[1]);
            
            let prefix = char1 * 16 + char2;
            let sub_hash = Hash {
                hash: &self.hash[2..]
            };
            
            return (prefix, sub_hash);
            
        } else {
            panic!("Brak danych na których można przeprowadzić tą operację");
        }
    }
}

fn hexToDigit(char: u8) -> u8 {
    15
}