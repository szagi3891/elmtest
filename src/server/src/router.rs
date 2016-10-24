use std::str::Split;

//TODO - use borrowing

pub struct Router<'a> {
    //chunks: Vec<&'a str>,
    first: Option<&'a str>,
    iterator: Split<'a, char>,
}

impl<'a> Router<'a> {
    pub fn new(addr: &'a str) -> Router<'a> {
        /*
        let map_fn = |arg: &str| -> String {
            arg.into()
        };
        
        //|arg: &str| arg.into()
        Router {
            chunks: parts.iter().map(map_fn).collect()
        }
        */
/*
        let parts: Vec<&str> = addr.split('/').collect();
        Router {
            chunks: parts
        }
*/
        Router {
            first: None,
            iterator:  addr.split('/')
        }
    }
}