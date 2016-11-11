use std::path::PathBuf;

use lib::blob_stor::hash::Hash;

pub struct Head {
    head: Hash,
    version: u32,
    //prev: Option<Head>            //TODO
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
}