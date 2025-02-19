use std::env;

use file_node::FileNode;
use walker::{python_walker::PythonWalker, rust_walker::RustWalker, Walker};
pub mod file_node;
pub mod walker;

enum FileExtension {
    Python,
    Rust,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No path supplied.");
    let walker: Box<dyn Walker> = file_extension_to_walker(path);
    println!("{:?}", walker.walk());
}

fn get_file_extension(path: &str) -> FileExtension {
    let extension = path.split(".").last().unwrap();
    return match extension {
        "py" => FileExtension::Python,
        "rs" => FileExtension::Rust,
        _ => panic!("Unsupported extension, {}", extension),
    };
}

fn file_extension_to_walker(path: &str) -> Box<dyn Walker> {
    let extension = get_file_extension(path);
    return match extension {
        FileExtension::Python => Box::new(PythonWalker::new(path)),
        FileExtension::Rust => Box::new(RustWalker::new(path)),
    };
}
