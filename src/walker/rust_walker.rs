use crate::walker::Walker;

pub struct RustWalker {
    file_path: String,
}

impl Walker for RustWalker {
    fn walk(&self) -> String {
        let mut out = String::from("Rust walker walking through project at");
        out.push_str(&self.file_path.clone());
        return out;
    }
}

impl RustWalker {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: String::from(path),
        }
    }
}
