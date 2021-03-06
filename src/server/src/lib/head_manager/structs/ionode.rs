use std::collections::HashMap;

use lib::blob_stor::BlobStor;
use lib::hash::Hash;
use lib::head_manager::structs::ioerror::IonodeResult;
use lib::head_manager::structs::iopath::Iopath;

enum IonodeType {
    Txt,                //zwykły plik tekstowy
    Mp3,                //muzyczka
}

enum IonodeContent {
    Dir(
        HashMap<String, Hash>
    ),
    File(
        IonodeType,         //Typ pliku
        Hash,               //jego zawartość
    ),
}

pub struct Ionode {
    stor: BlobStor,
    self_hash: Hash,
    content: IonodeContent,
}

impl Ionode {
    pub fn new_empty_dir(stor: &BlobStor) -> Ionode {

        let content = IonodeContent::Dir(HashMap::new());
        
        let empty_serialized = serialize(&content);
        let empty_hash = stor.set(empty_serialized.as_slice());
        
        Ionode {
            stor: stor.clone(),
            self_hash: empty_hash,
            content: content,
        }
    }
    
    pub fn hash(&self) -> Hash {
        self.self_hash.clone()
    }
    
    pub fn deserialize(hash_addr: Hash, stor: &BlobStor, data_in: &[u8]) -> Ionode {
        
        let (header, rest) = read_line(data_in).unwrap();
        
        if header.len() == 1 && header[0] == 48 {           //Katalog
            
            return Ionode {
                stor: stor.clone(),
                self_hash: hash_addr,
                content: deserialize_content_dir(rest),
            };
            
        } else if header.len() == 1 && header[0] == 49 {    //plik
            
            unimplemented!();           //TODO
            
        } else {
            panic!("nieprawidłowe dane");
        }   
    }
    
    pub fn new_file(&self, path: Iopath, data: &[u8]) -> IonodeResult<Ionode> {
        
        match self.content {
            IonodeContent::Dir(ref map_dir) => {

                match path.head() {

                    Some((first_item, rest_path)) => {
                        
                        if rest_path.len() > 0 {
                            
                            unimplemented!();       //TODO
                            /*
                                jeśli więcej niż jeden element, to znajdź element który powinien być katalogiem, i na nim wykonaj rekurencyjne odwołanie do tej metody tylko że skrócone o ten jeden element tej ścieżki
                            */


                        } else {

                            let data_hash = self.stor.set(data);    
                            let mut map_dir_clone = map_dir.clone();

                            if map_dir_clone.insert(first_item, data_hash).is_none() {

                                let new_content = IonodeContent::Dir(map_dir_clone);
                                let new_content_serialize = serialize(&new_content);
                                let new_content_hash = self.stor.set(&new_content_serialize);

                                return Ok(Ionode {
                                    stor: self.stor.clone(),
                                    self_hash: new_content_hash,
                                    content: new_content,
                                });

                            } else {
                                unimplemented!();           //TODO - zwrócić błąd użytkownika że taki element już istnieje
                            }
                        }
                    },

                    None => {
                        panic!("próba uruchomianie tworzenia nowego pliku na pliku");
                    }
                }

            },

            IonodeContent::File(_, _) => {
                unimplemented!();       //TODO - trzeba rzucić błędem że ten węzeł nie jest katalogiem
            }
        }
    }
}

fn deserialize_content_dir(data_in: &[u8]) -> IonodeContent {
    
    let mut map = HashMap::new();

    let mut data_wsk = data_in;

    loop {
        match read_line(data_wsk) {
            Some((line, rest)) => {

                let (name, item) = create_item(line);
                data_wsk = rest;

                match map.insert(name, item) {
                    Some(_) => {
                        panic!("zduplikowane rekordy");
                    },
                    _ => {},
                };
            },
            None => {
                return IonodeContent::Dir(map);
            }
        }
    }
}

fn create_item(line: &[u8]) -> (String, Hash) {
    
    let hash_data = &line[0..20];
    let name = &line[20..];

    let hash = Hash::from_bytes(hash_data);
    
    let mut name_vec = Vec::new();
    name_vec.extend_from_slice(name);

    let name_str = String::from_utf8(name_vec).unwrap();

    (name_str, hash)
}

fn read_line(data_in: &[u8]) -> Option<(&[u8], &[u8])> {
    
    if data_in.len() == 0 {
        return None;
    }

    match data_in.iter().position(|r| *r == 10) {
        Some(index) => {
            let first = &data_in[0..index];
            let rest = &data_in[index+1..];
            Some((first, rest))
        },
        None => {
            panic!("nieprawidłowe dane");
        }
    }
}

fn serialize(ionode_content: &IonodeContent) -> Vec<u8> {
    
    let mut out: Vec<u8> = Vec::new();
    
    match *ionode_content {
        
        IonodeContent::Dir(ref map) => {
            
            out.push(48);                               //informuje że to katalog
            out.push(10);
            
            serialize_dir(map, &mut out);
        },
        
        IonodeContent::File(ref type_node, ref hash) => {
            unimplemented!();                   //TODO
        }
    };
    
    out
}

fn serialize_dir(list: &HashMap<String, Hash>, out: &mut Vec<u8>) {

    let mut sort_keys = Vec::new();

    for (key, _) in list.iter() {
        sort_keys.push(key);
    }

    sort_keys.sort();
    
    for key_name in sort_keys {

        let val = list.get(key_name).unwrap();

        val.serialize(out);

        for item in key_name.as_bytes() {
            out.push(*item);
        }

        out.push(10);
    }
}
