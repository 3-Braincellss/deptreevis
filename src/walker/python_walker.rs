use crate::walker::Walker;

pub struct PythonWalker {
    file_path: String,
}

impl Walker for PythonWalker {
    fn walk(&self) -> String {
        let mut out = String::from("Python walker walking through project at ");
        out.push_str(&self.file_path.clone());
        return out;
    }
}

impl PythonWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
