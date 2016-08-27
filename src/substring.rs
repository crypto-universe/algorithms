#![allow(non_snake_case)]

//Knuth–Morris–Pratt algorithm
#[allow(dead_code)]
mod KMP {
    pub fn substring(needle: &[u8], haystack: &[u8]) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut prefix: Vec<usize> = Vec::with_capacity(needle.len());

        prefix.push(0);
        let mut last_prefix: usize = 0;

        //Init vector of prefixes
        for &c in &needle[1 .. ] {
            while (last_prefix > 0) && (needle[last_prefix] != c) {
                last_prefix = prefix[last_prefix-1];
            }
            if needle[last_prefix] == c {
                last_prefix += 1;
            }
            prefix.push(last_prefix);
        }

        last_prefix = 0;
        for (i, &d) in haystack.iter().enumerate() {
            while (last_prefix > 0) && (needle[last_prefix] != d) {
                last_prefix = prefix[last_prefix-1];
            }
            if needle[last_prefix] == d {
                last_prefix += 1;
            }
            if last_prefix == needle.len() {
                result.push(i+1-needle.len());
                last_prefix = prefix[last_prefix-1];
            }
        }
        result
    }
}

#[test]
fn substring_test_1() {
    let result = KMP::substring(b"abc", b"jfoabcojwfababcoigwibccbabbcwiopfj");
    assert!(result == vec![3, 12]);
}

#[test]
fn substring_test_2() {
    let result = KMP::substring(b"aabaa", b"aabaabaaaabaabaaab");
    assert!(result == vec![0, 3, 8, 11]);
}

#[test]
fn substring_test_3() {
    let result = KMP::substring(b"def", b"def");
    assert!(result == vec![0]);
}

#[test]
fn substring_test_4() {
    let result = KMP::substring(b"sss", b"sssssss");
    assert!(result == vec![0, 1, 2, 3, 4]);
}

#[test]
fn substring_test_5() {
    let result = KMP::substring(b"defserhjt", b"def");
    assert!(result == Vec::new());
}