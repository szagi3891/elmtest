use std::fmt::Write;
/*
pub fn to_hex(input: &[u8]) -> String {
    
    let mut out = String::new();

    for &byte in input {
        write!(&mut out, "{:X} ", byte).unwrap();
    }

    out
}
*/

                            //https://doc.rust-lang.org/beta/std/fmt/#formatting-traits

pub fn to_hex(input: &[u8]) -> String {
    
    let mut out = String::new();

    for &byte in input {
        write!(&mut out, "{:x}", byte).unwrap();
    }

    out
}