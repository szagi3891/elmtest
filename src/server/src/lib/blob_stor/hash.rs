pub struct Hash<'a> {
    hash: &'a [u8],
    pos: u8,                    //TODO - mayby use better implementation
}

impl<'a> Hash<'a> {
    pub fn new(hash: &'a [u8]) -> Hash {
        Hash {
            hash: hash,
            pos: 0,
        }
    }
}