use std::sync::RwLock;
use std::sync::Arc;

use lib::blob_stor::hash::Hash;

pub struct Dir {
    inner: Arc<RwLock<DirMode>>,
}

enum DirMode {
    Uninitialized,
    ContentFiles,
    ContentDir,
}

enum DirSetCommand {
    NeedInit,
    SetSuccess,
    NeedSubDir(u8),
}

impl Dir {

    pub fn new_uninit() -> Dir {
        Dir {
            inner: Arc::new(RwLock::new(DirMode::Uninitialized)),
        }
    }

    pub fn get(&mut self, hash: Hash) -> String {
        //TODO
        unimplemented!();
    }
    
    pub fn set(&mut self, hash: Hash, content: &[u8]) {
        
        let mut count_loop = 0;
        
        loop {
            count_loop += 1;
            if (count_loop > 10) {
                panic!("too much recursion");
            }
            
            match (self.set_exec(hash, content)) {
                DirSetCommand::NeedInit => {
                    
                    //TODO - odczytanie początkowej struktury katalogu
                    unimplemented!();
                },

                DirSetCommand::SetSuccess => {
                    return;
                },

                DirSetCommand::NeedSubDir(_) => {

                    //TODO - utworzenie kolejnego podkatalogu
                    unimplemented!();
                },
            }
        }
    }

    fn set_exec(&mut self, hash: Hash, content: &[u8]) -> DirSetCommand {
        
        let guard = self.inner.read().unwrap();

        match *guard {
            DirMode::Uninitialized => DirSetCommand::NeedInit,

            DirMode::ContentFiles => {
                //jeśli licznik jest ok,
                    //TODO - odpal procedurę pisania
                    DirSetCommand::SetSuccess
                //w przeciwnym razie, zwróć informację ze statusem, że ten katalog wymaga przebudowania na ContentDir
            },

            DirMode::ContentDir => {
                //weź podkatalog
                //istnieje
                    //odpal metodę set na tym podkatalogu
                //nie istnieje
                    DirSetCommand::NeedSubDir(0x43)
            },
        }
    }

    fn initialize(&mut self) {
        //czytanie struktury tego katalogu
    }
}