use lib::blob_stor::BlobStor;
use lib::blob_stor::hash::Hash;

pub struct Head {
    stor: BlobStor,
    version: u32,
    root: Hash,
    //prev: Option<Head>            //TODO
    //metadata
        //TODO - do nowo wygenerowanego head-a dorzucać informację o czasie oraz loginie użytkownika który wygenerował tą zmianę
}

impl Head {
    
    pub fn new_from_disk(stor: &BlobStor, version: u32, root: Hash) -> Head {
        Head {
            stor: stor.clone(),
            root: root,
            version: version,
        }
    }
    
    pub fn new(stor: &BlobStor, version: u32, root: Hash) -> Head {
        
        let head_serialize = serialize(version, &root);
        let head_hash = stor.set(head_serialize.as_slice());

        Head {
            stor: stor.clone(),
            root: root,
            version: version,
        }
    }
    
    pub fn get_hash(&self) -> Hash {
        self.root.clone()
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self.version, &self.root)
    }

    /*
    pub fn deserialize(data_in: &[u8]) -> Dir {
        //TODO
    }
    */
}

fn serialize(version: u32, root: &Hash) -> Vec<u8> {
        
    let mut out = Vec::new();
                                                    //pierwsza linia - version
    let version_str = version.to_string();

    for item in version_str.as_bytes() {
        out.push(*item);
    }

    out.push(10);

                                                    //druga linia - head
    root.serialize(&mut out);
    out.push(10);

    out
}