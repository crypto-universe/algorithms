#[allow(unused_imports)]
use sh_sub;

#[test]
fn shortest_substring_test_1() {
    let result = sh_sub::shortest_substring("jfoabcojwfababcoigwibccbabbcwiopfj");
    println!("{}", result);
    assert!(result == "gwibccbabbcwiopfj"); // letter "p" is unique in this string
}

#[test]
fn shortest_substring_test_2() {
    let result = sh_sub::shortest_substring("defserhjt");
    assert!(result == "defserhjt");
}

#[test]
fn shortest_substring_test_3() {
    let result = sh_sub::shortest_substring("abcdaabceabc");
    assert!(result == "daabce");
}

#[test]
fn shortest_substring_test_4() {
    let result = sh_sub::shortest_substring("abcdaabceabcde");
    assert!(result == "abcde");
}

#[test]
fn shortest_substring_test_5() {
    let result = sh_sub::shortest_substring("V");
    assert!(result == "V");
}

#[test]
fn shortest_substring_test_6() {
    let result = sh_sub::shortest_substring("44444444444444444");
    assert!(result == "4");
}

#[test]
fn shortest_substring_test_7() {
    let result = sh_sub::shortest_substring("");
    assert!(result == "");
}