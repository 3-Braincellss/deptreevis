use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

use walker::{python_walker::PythonWalker, rust_walker::RustWalker, Walker};
pub mod walker;

enum FileExtension {
    Python,
    Rust,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.iter().count() == 2,
        "Usage: ./deptreevis <PROJECT-PATH>"
    );
    let path = args.get(1).unwrap();
    let walker: Box<dyn Walker> = file_extension_to_walker(path);
    println!("{:?}", traverse(walker, path));
}

fn traverse(walker: Box<dyn Walker>, path: &str) -> HashMap<String, Vec<String>> {
    let mut dependancy_tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut to_visit: VecDeque<String> = VecDeque::from([String::from(path)]);

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();
        let dependants = walker.walk(&current);
        dependancy_tree.insert(current, dependants.clone());
        to_visit.append(&mut VecDeque::from_iter(dependants.into_iter()));
    }
    return dependancy_tree;
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
