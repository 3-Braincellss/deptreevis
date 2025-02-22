use deptreevis::traversal;
use deptreevis::walker::file_extension_to_walker;
use deptreevis::walker::Walker;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.iter().count() == 2,
        "Usage: ./deptreevis <PROJECT-PATH>"
    );
    let path = String::from(args.get(1).unwrap());
    let walker: Box<dyn Walker> = file_extension_to_walker(&path);
    println!("{:?}", traversal::traverse(walker, path));
}
