pub mod walker;

pub mod traversal {
    use crate::walker::Walker;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use std::fs;
    use std::path::PathBuf;

    pub fn traverse(walker: Box<dyn Walker>, path: String) -> HashMap<String, Vec<String>> {
        let mut root_dir_path: PathBuf = fs::canonicalize(path.clone()).unwrap();
        root_dir_path.pop();
        let root_dir = String::from(root_dir_path.to_str().unwrap());
        let mut dependency_tree: HashMap<String, Vec<String>> = HashMap::new();
        let mut to_visit: VecDeque<String> = VecDeque::from([String::from(
            fs::canonicalize(path).unwrap().to_str().unwrap(),
        )]);
        let mut seen: HashSet<String> = HashSet::new();

        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current.clone());
            let mut dependants: Vec<String> = walker.walk(&current, &root_dir);
            for dependant in dependants.iter_mut() {
                if dependant.starts_with(&root_dir) {
                    continue;
                }
                let mut new_dependant = root_dir.clone();
                new_dependant.push_str(dependant);
                *dependant = new_dependant;
            }
            dependency_tree.insert(current, dependants.clone());
            to_visit.append(&mut VecDeque::from_iter(dependants.into_iter()));
        }
        return dependency_tree;
    }
}

pub mod file {
    pub enum FileExtension {
        Python,
        Rust,
    }
    pub fn get_file_extension(path: &str) -> FileExtension {
        let extension = path.split(".").last().unwrap();
        return match extension {
            "py" => FileExtension::Python,
            "rs" => FileExtension::Rust,
            _ => panic!("Unsupported extension, {}", extension),
        };
    }
}
