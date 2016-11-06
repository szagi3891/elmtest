use lib::blob_stor::hash::Hash;

pub struct Head {
    head: Hash,
    version: u32,
    //metadata
        //TODO - do nowo wygenerowanego head-a dorzucać informację o czasie oraz loginie użytkownika który wygenerował tą zmianę
}

impl Head {
    pub fn new(hash: [u8; 40], version: u32) -> Head {
        Head {
            head: Hash::new(hash),
            version: version,
        }
    }
}