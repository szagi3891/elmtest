use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::read_dir;

pub enum DriverInitDirResult {
    Files(u32),
    Dirs(HashMap<u8, PathBuf>)
}

pub fn init_dir(path: &Path) -> DriverInitDirResult {

    let mut files_count = 0;
    let dir_list = read_dir(path).unwrap();

    let mut map: HashMap<u8, PathBuf> = HashMap::new();            //TODO - remove type

    for path in dir_list {

        let item = path.unwrap();
        let metadata = item.metadata().unwrap();

        if (metadata.is_file()) {

            files_count += 1;

        } else if (metadata.is_dir()) {

            let file_name = item.file_name();
            let file_str = file_name.to_str().unwrap();

            if file_str.len() == 2 {

                //TODO - sprawdzić czy na pewno nazwy są małymi literami
                
                let prefix = u8::from_str_radix(file_str, 16).unwrap();
                
                assert_eq!(map.insert(prefix, item.path()), None);

            } else {
                panic!("incorrect contents of a directory");
            }

        } else {
            panic!("incorrect contents of a directory");
        }
    }

    if map.len() > 0 {
        DriverInitDirResult::Dirs(map)
    } else {
        DriverInitDirResult::Files(files_count)
    }
}