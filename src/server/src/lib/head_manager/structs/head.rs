use lib::blob_stor::BlobStor;
use lib::blob_stor::hash::Hash;

pub struct Head {
    self_hash: Hash,
    stor: BlobStor,
    version: u32,                                   //TODO - do zastanowienia w którym miejscy powinna być przechowywana wersja head-a
                                                    //możliwe że ta informacja przy serializacji nie powinna być zapisywana
                                                    //i tylko z nazwy pliku czerpana być powinna
    root: Hash,
    //prev: Option<Head>            //TODO
    //metadata
        //TODO - do nowo wygenerowanego head-a dorzucać informację o czasie oraz loginie użytkownika który wygenerował tą zmianę
}

impl Head {
    
    pub fn new(stor: &BlobStor, version: u32, root: Hash) -> Head {
        
        let head_serialize = serialize(version, &root);
        let head_hash = stor.set(head_serialize.as_slice());

        Head {
            self_hash: head_hash,
            stor: *stor.clone(),
            root: root,
            version: version,
        }
    }
    
    pub fn get_hash(&self) -> Hash {
        self.self_hash.clone()
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self.version, &self.root)
    }
    
    pub fn deserialize(self_hash: Hash, stor: &BlobStor, data_in: &[u8]) -> Head {
        
        if data_in.len() == 0 {
            panic!("spodziewano się danych");
        }

        let mut iter = data_in.split(|char| *char == 10);
        
        let line_version = iter.next();
        let line_head = iter.next();
        let rest1 = iter.next();
        let rest2 = iter.next();
        
        match (line_version, line_head, rest1.unwrap().len(), rest2) {
            (Some(version), Some(head), 0, None) => {
                
                let mut version_vec = Vec::new();
                version_vec.extend_from_slice(version);

                let version_str = String::from_utf8(version_vec).unwrap();
                
                let version_number = u32::from_str_radix(version_str.as_str(), 10).unwrap();

                Head {
                    self_hash: self_hash,
                    stor: *stor.clone(),
                    root: Hash::from_bytes(head),
                    version: version_number,
                }
            },
            _ => {
                panic!("problemy w deserializacji head-a");
            }
        }
    }
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