use crate::walker::Walker;
use std::fs::read_to_string;

pub struct PythonWalker {}

impl Walker for PythonWalker {
    fn walk(&self, path: &str) -> Vec<String> {
        return read_to_string(path)
            .expect(&format!("Path not found. {}", path))
            .split("\n")
            .filter(|line| line.starts_with("import") || line.starts_with("from"))
            .map(|import_statement| self.import_statement_to_path(import_statement))
            .collect();
    }
}

impl PythonWalker {
    pub fn new() -> Self {
        Self {}
    }

    fn import_statement_to_path(&self, path: &str) -> String {
        let mut out = path
            .split(" ")
            .nth(1)
            .expect("Malformed import statement found.")
            .replace(".", "/");
        out.push_str(".py");
        return out;
    }
}
