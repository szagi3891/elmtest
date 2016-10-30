use std::sync::RwLock;
use std::sync::Arc;

pub struct BlobStor<'a> {
    base_path: &'a str,
    root: Dir,
}

struct Dir {
    inner: Arc<RwLock<DirMode>>,
}

enum DirMode {
    Uninitialized,
    ContentFiles,
    ContentDir,
}

enum DirSetCommand {
    NeedInit,
    ReedSuccess,
    NeedSubDir(u8),
}

impl<'a> BlobStor<'a> {

    pub fn new(base_path: &'a str) -> BlobStor<'a> {
        
        //TODO - sprawdź czy ten katalog istnieje - jeśli nie to go stwórz
        
        BlobStor {
            base_path: base_path,
            root : Dir {
                inner: Arc::new(RwLock::new(DirMode::Uninitialized)),
            },
        }
    }

    pub fn get(&mut self, hash: &'a [u8]) -> String {
        self.root.get(hash)
    }
    
    pub fn set(&mut self, hash: &[u8], content: &[u8]) {
        self.root.set(hash, content)
    }
}

impl Dir {

    fn get(&mut self, hash: &[u8]) -> String {
        //TODO
        return "dasdas".to_string();
    }
    
    fn set(&mut self, hash: &[u8], content: &[u8]) {
        
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

                DirSetCommand::ReedSuccess => {
                    return;
                },

                DirSetCommand::NeedSubDir(_) => {

                    //TODO - utworzenie kolejnego podkatalogu
                    unimplemented!();
                },
            }
        }
    }

    fn set_exec(&mut self, hash: &[u8], content: &[u8]) -> DirSetCommand {
        
        let guard = self.inner.read().unwrap();

        match *guard {
            DirMode::Uninitialized => DirSetCommand::NeedInit,
            DirMode::ContentFiles => {
                //jeśli licznik jest ok,
                    //TODO - odpal procedurę czytania
                    DirSetCommand::ReedSuccess
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