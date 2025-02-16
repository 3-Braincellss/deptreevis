pub mod walker;

use walker::Walker;

use crate::walker::python_walker::PythonWalker;

fn main() {
    let walker = PythonWalker::new("hey");
    println!("{}", walker.walk());
}
