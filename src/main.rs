pub mod dep_tree_vis_file;
pub mod walker;

use walker::Walker;

use crate::walker::python_walker::PythonWalker;

fn main() {
    let walker = PythonWalker::new("snippets/snip_python_0/run.py");
    println!("{}", walker.walk());
}
