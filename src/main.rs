pub mod dep_tree_vis_file;
pub mod walker;

use dep_tree_vis_file::DepTreeVisFile;
use walker::Walkable;

use crate::walker::python_walker::PythonWalker;

fn main() {
    let walker = PythonWalker::new("/home/pom/Repos/deptreevis/snippets/sinp_python_0/run.py");
    let mut file = DepTreeVisFile::new("/home/pom/Repos/deptreevis/snippets/sinp_python_0/run.py");
    walker.walk(&mut file);
}
