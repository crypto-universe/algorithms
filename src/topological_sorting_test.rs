#[allow(unused_imports)]
use topological_sorting;

#[test]
fn khans_sort_test1() {
    let mut dag: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    dag.insert(0, vec![1]);
        //(1, vec![]),
    dag.insert(2, vec![0, 3]);
    dag.insert(3, vec![1]);
    
    let result: Vec<usize> = topological_sorting::kahns_sort(&dag, true).unwrap();
    assert_eq!(result.as_slice(), &[2, 3, 0, 1]);
}

#[test]
fn khans_sort_test2() {
    let mut dag: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    dag.insert(11, vec![2, 9, 10]);
    dag.insert(8, vec![9]);
    dag.insert(3, vec![8, 10]);
    dag.insert(7, vec![8, 11]);
    dag.insert(5, vec![11]);
    
    let result: Vec<usize> = topological_sorting::kahns_sort(&dag, true).unwrap();
    assert_eq!(result.as_slice(), &[7, 5, 11, 2, 3, 10, 8, 9]);
}

#[test]
fn khans_sort_test3() {
    let mut dag: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    dag.insert(0, vec![1]);
    dag.insert(1, vec![3]); //cycle 1 <=> 3
    dag.insert(2, vec![0, 3]);
    dag.insert(3, vec![1]);
    
    let result: &str = topological_sorting::kahns_sort(&dag, true).unwrap_err();
    assert_eq!(result, "Provided graph has cycles!");
}

#[test]
fn khans_sort_test4() {
    let mut dag: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    dag.insert("A", vec!["C", "D"]);
    dag.insert("C", vec!["F"]);
    dag.insert("E", vec!["F"]);
    dag.insert("B", vec!["D", "E"]);
    dag.insert("D", vec!["F"]);
    
    let result: Vec<&str> = topological_sorting::kahns_sort(&dag, true).unwrap();
    assert_eq!(result, &["B", "E", "A", "D", "C", "F"]);
}

#[test]
fn khans_sort_test5() {
    let mut dag: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    dag.insert(133, vec![133]);
    
    let result: &str = topological_sorting::kahns_sort(&dag, false).unwrap_err();
    assert_eq!(result, "Provided graph has cycles!");
}