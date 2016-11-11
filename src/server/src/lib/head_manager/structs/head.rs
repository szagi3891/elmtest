use std::path::PathBuf;

use lib::blob_stor::hash::Hash;

pub struct Head {
    head: Hash,
    version: u32,
    //metadata
        //TODO - do nowo wygenerowanego head-a dorzucać informację o czasie oraz loginie użytkownika który wygenerował tą zmianę
}

impl Head {
    pub fn new(hash: Hash, version: u32) -> Head {
        Head {
            head: hash,
            version: version,
        }
    }
    
    //TODO - przenieść tą funkcję, jako funkcję statyczną struktury Head

    pub fn read_last(path_head: &PathBuf) -> Head {

        /*
            czytaj namiar na ostatniego head-a

                jeśli go nie ma, to zainicjuj nowego pustego heada

            jeśli jest
                przeczytaj heada oraz numer wersji
        */
        //TODO - trzeba zainicjować początkową strukturę

        let hash_str = [48; 40];        //40 x '0'
        let version_start = 0;

        let hash = Hash::new(hash_str);

        Head::new(hash, version_start)
    }
}