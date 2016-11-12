use std::path::PathBuf;

use lib::blob_stor::hash::Hash;

pub struct Head {
    version: u32,
    root: Hash,
    //prev: Option<Head>            //TODO
    //metadata
        //TODO - do nowo wygenerowanego head-a dorzucać informację o czasie oraz loginie użytkownika który wygenerował tą zmianę
}

impl Head {
    pub fn new(root: Hash, version: u32) -> Head {
        Head {
            root: root,
            version: version,
        }
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        
                                                        //pierwsza linia - version
        let version_str = self.version.to_string();
        
        for item in version_str.as_bytes() {
            out.push(*item);
        }

        out.push(10);
        
                                                        //druga linia - head
        self.root.serialize(&mut out);
        out.push(10);
        
        out
    }
    
    /*
    pub fn deserialize(data_in: &[u8]) -> Dir {
        //TODO
    }
    */
}