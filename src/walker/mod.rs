use crate::file_node::FileNode;

pub mod python_walker;
pub mod rust_walker;

pub trait Walker {
    fn walk(&self, file: &mut FileNode);
}
