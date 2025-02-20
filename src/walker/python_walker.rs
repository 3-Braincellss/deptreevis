use crate::{file_node::FileNode, walker::Walker};

pub struct PythonWalker {
    file_path: String,
}

impl Walker for PythonWalker {
    fn walk(&self, file: &mut FileNode) {
        let import_statements: Vec<String> = file
            .read()
            .split("\n")
            .filter(|line| line.starts_with("import") || line.starts_with("from"))
            .map(String::from)
            .collect();
        println!("{:?}", import_statements)
    }
}

impl PythonWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
