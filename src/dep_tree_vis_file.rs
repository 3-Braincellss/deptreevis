use std::path::Path;

struct DepTreeVisFile {
    path: Box<Path>,
    children: Vec<Box<DepTreeVisFile>>,
}

impl DepTreeVisFile {}
