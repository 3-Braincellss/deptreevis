use crate::walker::Walker;

pub struct RustWalker {}

impl Walker for RustWalker {
    fn walk(&self, path: &str) -> Vec<String> {
        panic!("Not Implemented");
    }
}

impl RustWalker {
    pub fn new() -> Self {
        Self {}
    }
}
