#[allow(unused_imports)]
use kmp;

#[test]
fn substring_test_1() {
    let result = kmp::substring(b"abc", b"jfoabcojwfababcoigwibccbabbcwiopfj");
    assert!(result == vec![3, 12]);
}

#[test]
fn substring_test_2() {
    let result = kmp::substring(b"aabaa", b"aabaabaaaabaabaaab");
    assert!(result == vec![0, 3, 8, 11]);
}

#[test]
fn substring_test_3() {
    let result = kmp::substring(b"def", b"def");
    assert!(result == vec![0]);
}

#[test]
fn substring_test_4() {
    let result = kmp::substring(b"sss", b"sssssss");
    assert!(result == vec![0, 1, 2, 3, 4]);
}

#[test]
fn substring_test_5() {
    let result = kmp::substring(b"defserhjt", b"def");
    assert!(result == Vec::new());
}