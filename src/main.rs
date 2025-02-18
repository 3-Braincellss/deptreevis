pub mod dep_tree_vis_file;
pub mod walker;

use dep_tree_vis_file::DepTreeVisFile;
use walker::Walker;

use crate::walker::python_walker::PythonWalker;

fn main() {
    let walker = PythonWalker::new("snippets/snip_python_0/run.py");
    let file = DepTreeVisFile::new("src/walker/python_walker.rs");
    println!("{}", file.read());
    println!("{}", walker.walk());
}
