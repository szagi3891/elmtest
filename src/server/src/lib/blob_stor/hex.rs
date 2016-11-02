use std::fmt::Write;

                            //https://doc.rust-lang.org/beta/std/fmt/#formatting-traits

                            //TODO - wymienić implementację na prostszą
pub fn to_hex(input: &[u8]) -> String {
    
    let mut out = String::new();

    for &byte in input {
        write!(&mut out, "{:02x}", byte).unwrap();
    }

    out
}