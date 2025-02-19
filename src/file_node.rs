use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileNode {
    path: PathBuf,
    children: Vec<Box<FileNode>>,
}

impl FileNode {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            children: Vec::new(),
        }
    }

    pub fn read(self) -> String {
        fs::read_to_string(
            self.path
                .to_str()
                .expect("This path is not utf-8 friendly :("),
        )
        .expect("This path doesnt exist.")
    }
}
