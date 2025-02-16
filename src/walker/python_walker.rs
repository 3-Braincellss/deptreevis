use crate::walker::Walker;

pub struct PythonWalker {
    file_path: String,
}

impl Walker for PythonWalker {
    fn walk(&self) -> String {
        String::from("Python walking logic here.")
    }
}

impl PythonWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
