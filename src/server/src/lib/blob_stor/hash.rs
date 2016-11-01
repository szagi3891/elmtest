use std::path::PathBuf;
use std::str;
use std::u8;

use lib::blob_stor::hex::to_hex;

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
        
        if (hash.len() != 40) {
            panic!("nieprawidłowa długość {:?}", hash.len());
        }
        
        Hash {
            hash: convertToHex(hash)
        }
    }
    
    pub fn add_to_path(&self, path: &mut PathBuf) {
        
        unimplemented!();
        
        /*
        let hash_str = str::from_utf8(self.hash).unwrap();
        path.push(hash_str);
        */
        //path
    }

    pub fn get_prefix(&self, pos: usize) -> u8 {
        self.hash[pos]
    }
}

fn convertToHex(hash: &[u8]) -> [u8; 20] {
    
    let mut out = [0; 20];
    
    for index in 0..19 {
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

/*
fn hexToDigit(char: u8) -> u8 {
    15
    to_digit(16)
    assert_eq!(u32::from_str_radix("A", 16), Ok(10));
}
*/

/*
extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::hex::FromHex;

fn hex2string(x: &str) -> Option<String> {
    x.from_hex().ok().and_then(|x| String::from_utf8(x).ok())
}

fn main() {
    println!("{}", hex2string("41706f70686973").unwrap());
}
*/