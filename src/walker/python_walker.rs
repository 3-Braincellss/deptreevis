use crate::walker::Walker;
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

pub struct PythonWalker {}

impl Walker for PythonWalker {
    fn walk(&self, path: &str, root_dir: &str) -> Vec<String> {
        return read_to_string(path)
            .expect(&format!("Path not found. {}", path))
            .split("\n")
            .filter(|line| line.starts_with("import") || line.starts_with("from"))
            .map(|import_statement| {
                if import_statement.starts_with("from .") {
                    self.import_statement_to_relative_path(import_statement, path)
                } else {
                    self.import_statement_to_path(import_statement, root_dir)
                }
            })
            .map(|path| String::from(path.to_str().unwrap()))
            .collect();
    }
}

impl PythonWalker {
    pub fn new() -> Self {
        Self {}
    }

    fn import_statement_to_path(&self, import_statement: &str, root_dir: &str) -> PathBuf {
        let mut out = import_statement
            .split(" ")
            .nth(1)
            .expect("Malformed import statement found.")
            .replace(".", "/");
        out.push_str(".py");
        let mut out_path = PathBuf::from(root_dir);
        out_path.push(out);
        return out_path;
    }

    fn import_statement_to_relative_path(&self, import_statement: &str, path: &str) -> PathBuf {
        let mut out_path: PathBuf = PathBuf::from(path);
        out_path.pop();
        let mut import_path = import_statement
            .split(" ")
            .nth(1)
            .expect("Malformed import statement found.")[1..]
            .to_string();
        Self::go_up_level(&import_path, &mut out_path);
        import_path = import_path.replace(".", "/");
        import_path.push_str(".py");
        for module in import_path.split("/") {
            out_path.push(module);
        }
        return fs::canonicalize(out_path.clone()).unwrap();
    }

    fn go_up_level(import_path: &str, path: &mut PathBuf) {
        let mut chars = import_path.chars();
        while chars.next() == Some('.') {
            path.push("..");
        }
    }
}
