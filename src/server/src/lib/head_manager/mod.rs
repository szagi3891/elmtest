use std::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;

use lib::blob_stor::BlobStor;

pub struct HeadManager {
	inner: Arc<RwLock<[u8; 20]>>,
	stor: BlobStor
}

impl HeadManager {
	pub fn new(base_path: PathBuf, max_file: u32) -> HeadManager {
		HeadManager {
			inner: Arc::new(RwLock::new([0; 20])),
			stor: BlobStor::new(base_path, max_file),
		}
	}
}

/*
typy danych

head:
root -> hash
prev -> hash | null
time -> u64 - czas utworzenia

dir:
nazwa elementu -> hash
nazwa elementu -> hash

plik:
dane ...

metody :

1
get_head -> HASH
    pobiera aktualnego head-a

2
upload content: [u8] -> UUID
    upload jakiegoś kontentu na serwer -> zwraca uuid tego kontentu

3
get_item hash: Hash -> [u8]
    pobiera bloba                       Dir | File

4
update_item head: Hash, path: Path, new_content: UUID -> void
    aktualizacja item-a na serwerze

5
delete_item head: Hash, path: Path, item: string -> void
    usunięcie itema z jakiegoś katalogu

6
new_file: head: Hash, path: Path, item: string, content: UUID -> void
    utworzenie nowego pliku we wskazanej lokalizacji
 
7
new_dir: head: Hash, path: Path, item: string -> void
    utworzenie nowego katalogu we wskazanej lokalizacji

8
zmiana historii - do head2
head -> head1 -> head2 -> head3 -> null
nowa ścieżka
head' -> head1' -> head2' -> null

*/