use std::collections::HashMap;
use std::fs;

use deptreevis::traversal::traverse;
use deptreevis::walker::python_walker::PythonWalker;
use deptreevis::walker::Walker;

#[test]
fn test_no_imports() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_0/run.py"));
    assert!(dependants.contains_key(&create_abs_path("snippets/snip_python_0/run.py")));
    assert!((dependants
        .get(&create_abs_path("snippets/snip_python_0/run.py"))
        .unwrap())
    .is_empty());
}

#[test]
fn test_local_imports() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_1/run.py"));
    let mut expected: HashMap<String, Vec<String>> = HashMap::new();
    expected.insert(
        create_abs_path("snippets/snip_python_1/run.py"),
        vec![create_abs_path("snippets/snip_python_1/src/some/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_1/src/some/module.py"),
        vec![create_abs_path("snippets/snip_python_1/src/cool/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_1/src/cool/module.py"),
        vec![],
    );
    assert_eq!(expected, dependants);
}

#[test]
fn test_circular_dependancy() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants = traverse(walker, String::from("snippets/snip_python_2/run.py"));
    let mut expected = HashMap::new();
    expected.insert(
        create_abs_path("snippets/snip_python_2/run.py"),
        vec![create_abs_path("snippets/snip_python_2/src/some/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_2/src/some/module.py"),
        vec![create_abs_path("snippets/snip_python_2/src/cool/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_2/src/cool/module.py"),
        vec![create_abs_path("snippets/snip_python_2/src/some/module.py")],
    );
    assert_eq!(expected, dependants);
}

#[test]
fn test_relative_imports() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_3/run.py"));
    let mut expected: HashMap<String, Vec<String>> = HashMap::new();
    expected.insert(
        create_abs_path("snippets/snip_python_3/run.py"),
        vec![create_abs_path("snippets/snip_python_3/src/some/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_3/src/some/module.py"),
        vec![create_abs_path("snippets/snip_python_3/src/cool/module.py")],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_3/src/cool/module.py"),
        vec![],
    );
    assert_eq!(expected, dependants);
}

#[test]
fn test_deeply_nested_relative_import() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_4/run.py"));
    let mut expected: HashMap<String, Vec<String>> = HashMap::new();
    expected.insert(
        create_abs_path("snippets/snip_python_4/run.py"),
        vec![create_abs_path(
            "snippets/snip_python_4/src/shallow/module.py",
        )],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_4/src/shallow/module.py"),
        vec![create_abs_path(
            "snippets/snip_python_4/src/really/really/really/really/nested/module.py",
        )],
    );
    expected.insert(
        create_abs_path("snippets/snip_python_4/src/really/really/really/really/nested/module.py"),
        vec![],
    );
    assert_eq!(expected, dependants);
}

fn create_abs_path(path: &str) -> String {
    return String::from(fs::canonicalize(path).unwrap().to_str().unwrap());
}
