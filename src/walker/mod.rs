use crate::dep_tree_vis_file::DepTreeVisFile;

pub mod python_walker;
pub mod rust_walker;

pub trait Walkable {
    fn walk(&self, file: &mut DepTreeVisFile);
}
