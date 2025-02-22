use std::collections::HashMap;

use deptreevis::traversal::traverse;
use deptreevis::walker::python_walker::PythonWalker;
use deptreevis::walker::Walker;

#[test]
fn test_no_imports() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_0/run.py"));
    assert!(dependants.contains_key(&String::from("snippets/snip_python_0/run.py")));
    assert!((dependants.get("snippets/snip_python_0/run.py").unwrap()).is_empty());
}

#[test]
fn test_local_imports() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    let dependants: HashMap<String, Vec<String>> =
        traverse(walker, String::from("snippets/snip_python_1/run.py"));
    let mut expected: HashMap<String, Vec<String>> = HashMap::new();
    expected.insert(
        String::from("snippets/snip_python_1/run.py"),
        vec![String::from("snippets/snip_python_1/src/some/module.py")],
    );
    expected.insert(
        String::from("snippets/snip_python_1/src/some/module.py"),
        vec![String::from("snippets/snip_python_1/src/cool/module.py")],
    );
    expected.insert(
        String::from("snippets/snip_python_1/src/cool/module.py"),
        vec![],
    );
    assert_eq!(expected, dependants);
}

#[test]
#[should_panic(expected = "Circular dependancy detected!")]
fn test_circular_dependancy() {
    let walker: Box<dyn Walker> = Box::new(PythonWalker::new());
    traverse(walker, String::from("snippets/snip_python_2/run.py"));
}
