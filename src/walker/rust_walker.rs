use crate::walker::Walker;

pub struct RustWalker {
    file_path: String,
}

impl Walker for RustWalker {
    fn walk(&self, path: &str) -> Vec<String> {
        panic!("Not Implemented");
    }
}

impl RustWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
