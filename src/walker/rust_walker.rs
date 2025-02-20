use crate::{file_node::FileNode, walker::Walker};

pub struct RustWalker {
    file_path: String,
}

impl Walker for RustWalker {
    fn walk(&self, file: &mut FileNode) {}
}

impl RustWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
