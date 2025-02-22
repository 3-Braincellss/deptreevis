use crate::walker::Walker;

pub struct RustWalker {}

impl Walker for RustWalker {
    fn walk(&self, _path: &str, _root_dir: &str) -> Vec<String> {
        panic!("Not Implemented");
    }
}

impl RustWalker {
    pub fn new() -> Self {
        Self {}
    }
}
