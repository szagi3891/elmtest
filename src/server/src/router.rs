//TODO - use borrowing

pub struct Router<'a> {
    chunks: Vec<&'a str>,
}

impl<'a> Router<'a> {
    pub fn new(addr: &'a str) -> Router<'a> {
        let parts: Vec<&str> = addr.split('/').collect();
        /*
        let map_fn = |arg: &str| -> String {
            arg.into()
        };
        
        //|arg: &str| arg.into()
        Router {
            chunks: parts.iter().map(map_fn).collect()
        }
        */
        Router {
            chunks: parts
        }
    }
}