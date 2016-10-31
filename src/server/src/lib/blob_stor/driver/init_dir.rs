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

    'nextitem : for path in dir_list {

        let item = path.unwrap();
        let metadata = item.metadata().unwrap();

        if (metadata.is_file()) {
            files_count += 1;
            continue 'nextitem;                
        }

        if (metadata.is_dir()) {
            let file_name = item.file_name();
            let file_str = file_name.to_str().unwrap();

            if file_str.len() == 2 {

                println!("katalog {}", file_str);

                //TODO , trzeba
                unimplemented!();

                continue 'nextitem;
            }

            panic!("incorrect contents of a directory");

        }

        panic!("incorrect contents of a directory");
    }

    if map.len() > 0 {

        DriverInitDirResult::Dirs(map)
        
    } else {

        DriverInitDirResult::Files(files_count)
    }
}