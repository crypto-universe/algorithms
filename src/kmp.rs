// Knuth–Morris–Pratt algorithm
pub fn substring(needle: &[u8], haystack: &[u8]) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut prefix: Vec<usize> = Vec::with_capacity(needle.len());

    prefix.push(0);
    let mut last_prefix: usize = 0;

    // Init vector of prefixes
    for &c in &needle[1..] {
        while (last_prefix > 0) && (needle[last_prefix] != c) {
            last_prefix = prefix[last_prefix - 1];
        }
        if needle[last_prefix] == c {
            last_prefix += 1;
        }
        prefix.push(last_prefix);
    }

    last_prefix = 0;
    for (i, &d) in haystack.iter().enumerate() {
        while (last_prefix > 0) && (needle[last_prefix] != d) {
            last_prefix = prefix[last_prefix - 1];
        }
        if needle[last_prefix] == d {
            if last_prefix + 1 == needle.len() {
                result.push(i + 1 - needle.len());
                last_prefix = prefix[last_prefix];
            } else {
                last_prefix += 1;
            }
        }

    }
    result
}
