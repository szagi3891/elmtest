use std::str::Split;

pub struct Router<'a> {
    next: Option<&'a str>,
    iterator: Split<'a, char>,
}

impl<'a> Router<'a> {
    pub fn new(addr: &'a str) -> Router<'a> {

        let mut iterator = addr.split('/');
        let fist_skip = iterator.next();

        assert_eq!(fist_skip, Some(""));
        
        let next = iterator.next();

        Router {
            next: next,
            iterator: iterator
        }
    }
    
    pub fn eq(&mut self, chunk: &'a str) -> bool {

        match self.next {
            Some(value) => {
                println!("aaaa ---- {:?}", value);

                if value == chunk {
                    self.next = self.iterator.next();
                    return true;
                }
                
                return false;
            },
            None => {
                false
            }
        }
    }
    
                                                    //TODO - remove alocation
    pub fn url(mut self) -> Option<String> {
        match self.next {
            Some(value) => {
                let mut out = Vec::new();
                
                out.push("".to_string());
                out.push(value.to_string());
                
                loop {
                    match self.iterator.next() {
                        
                        Some(next_value) => {
                            out.push(next_value.to_string());
                        },
                        
                        None => {
                            return Some(out.join("/"));
                        }
                    }
                }
            },
            
            None => None,
        }
    }
}