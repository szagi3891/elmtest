pub struct Iopath {
    path: Vec<String>,
}

impl Iopath {
    
    pub fn new(path: Vec<String>) -> Iopath {
        Iopath {
            path: path
        }
    }
    
    pub fn head(&self) -> Option<(String, Iopath)> {

        if self.path.len() > 0 {

            let head_item = self.path[0].clone();
            let mut out = Vec::new();

            for i in 1..self.path.len() {
                out.push(self.path[i].clone());
            }

            Some((head_item, Iopath::new(out)))

        } else {
            None
        }
    }
    
    pub fn len(&self) -> usize {
        self.path.len()
    }
}