pub mod walker;

pub mod traversal {
    use crate::walker::Walker;
    use std::collections::HashMap;
    use std::collections::VecDeque;

    pub fn traverse(walker: Box<dyn Walker>, path: String) -> HashMap<String, Vec<String>> {
        let mut root_dir: String = path
            .split("/")
            .take(path.split("/").count() - 1)
            .map(String::from)
            .collect::<Vec<String>>()
            .join("/");
        root_dir.push_str("/");
        let mut dependancy_tree: HashMap<String, Vec<String>> = HashMap::new();
        let mut to_visit: VecDeque<String> = VecDeque::from([String::from(path)]);

        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            let mut dependants: Vec<String> = walker.walk(&current);
            for dependant in dependants.iter_mut() {
                let mut new_dependant = root_dir.clone();
                new_dependant.push_str(dependant);
                *dependant = new_dependant;
            }
            dependancy_tree.insert(current, dependants.clone());
            to_visit.append(&mut VecDeque::from_iter(dependants.into_iter()));
        }
        return dependancy_tree;
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
