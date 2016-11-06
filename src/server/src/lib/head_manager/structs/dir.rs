use std::collections::HashMap;

use lib::blob_stor::hash::Hash;

pub struct Dir {
    list: HashMap<String, DirItem>,
}

enum kindType {
    File,
    Dir
}

struct DirItem {
    kind: kindType,
    head: Hash,
}

impl Dir {
    pub fn new_empty() -> Dir {
        Dir {
            list: HashMap::new()
        }
    }
}