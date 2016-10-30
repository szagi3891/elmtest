use std::path::PathBuf;

use lib::blob_stor::hex::to_hex;

pub struct Hash<'a> {
    hash: &'a str,
    pos: u8,                    //TODO - mayby use better implementation
}

impl<'a> Hash<'a> {
    pub fn new(hash: &'a str) -> Hash {
        Hash {
            hash: hash,
            pos: 0,
        }
    }
    
    pub fn as_str(&self) -> PathBuf {            //TODO - uwzględnić pozycję
        let mut buff = PathBuf::new();
        buff.push(self.hash);
        buff
    }
}