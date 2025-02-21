pub mod python_walker;
pub mod rust_walker;

pub trait Walker {
    fn walk(&self, path: &str) -> Vec<String>;
}
