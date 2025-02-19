use crate::{dep_tree_vis_file::DepTreeVisFile, walker::Walkable};

pub struct RustWalker {
    file_path: String,
}

impl Walkable for RustWalker {
    fn walk(&self, file: &mut DepTreeVisFile) {}
}

impl RustWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
