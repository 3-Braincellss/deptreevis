use crate::walker::Walker;
use std::fs::read_to_string;

pub struct PythonWalker {
    file_path: String,
}

impl Walker for PythonWalker {
    fn walk(&self, path: &str) -> Vec<String> {
        return read_to_string(path)
            .expect(&format!("Path not found. {}", path))
            .split("\n")
            .filter(|line| line.starts_with("import") || line.starts_with("from"))
            .map(String::from)
            .collect();
    }
}

impl PythonWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }

    fn import_statement_to_path(&self, path: &str) -> String {
        return path
            .split(" ")
            .nth(1)
            .expect("Malformed import statement found.")
            .replace(".", "/");
    }
}
