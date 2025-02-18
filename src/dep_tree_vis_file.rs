use std::path::PathBuf;

pub struct DepTreeVisFile {
    path: PathBuf,
    children: Vec<Box<DepTreeVisFile>>,
}

impl DepTreeVisFile {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            children: Vec::new(),
        }
    }
}
