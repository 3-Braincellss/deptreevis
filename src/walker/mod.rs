use crate::file::get_file_extension;
use crate::file::FileExtension;
use crate::walker::python_walker::PythonWalker;
use crate::walker::rust_walker::RustWalker;
pub mod python_walker;
pub mod rust_walker;

pub trait Walker {
    fn walk(&self, path: &str) -> Vec<String>;
}

pub fn file_extension_to_walker(path: &str) -> Box<dyn Walker> {
    let extension = get_file_extension(path);
    return match extension {
        FileExtension::Python => Box::new(PythonWalker::new(path)),
        FileExtension::Rust => Box::new(RustWalker::new(path)),
    };
}
