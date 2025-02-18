use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
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

    pub fn read(self) -> String {
        fs::read_to_string(
            self.path
                .to_str()
                .expect("This path is not utf-8 friendly :("),
        )
        .expect("This path doesnt exist.")
    }
}
