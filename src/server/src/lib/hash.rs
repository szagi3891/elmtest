use std::path::PathBuf;

use lib::hex::to_hex;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Hash {
    hash: [u8; 20],
}

impl Hash {
    pub fn new(hash: [u8; 20]) -> Hash {
        
        Hash {
            hash: hash
        }
    }
                                                                //TODO zmienić potem nazwę na from_bytes
    pub fn from_bytes(hash: &[u8]) -> Hash {
        
        if hash.len() != 20 {
            panic!("nieprawidłowa długość {:?}", hash.len());
        }
        
        let mut out = [0; 20];
        out.copy_from_slice(&hash);
        
        Hash {
            hash: out
        }
    }

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
    pub fn to_hex(&self) -> String {
        
        to_hex(&self.hash)
    }
    
                                                //TODO - zmienić potem znowu na seiralize
    pub fn serialize(&self, out: &mut Vec<u8>) {        

        for item in self.hash.iter() {
            out.push(*item);
        }
    }
}
