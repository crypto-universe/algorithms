use std::cmp::min;
use std::mem::swap;

// iterative implementation with two matrix rows
pub fn levenshtein_distance_1(str1: &str, str2: &str) -> usize {
    let len1 = str1.len();

    // initial distances for row1 are known
    let mut row1: Vec<usize> = (0..len1 + 1).collect();
    let mut row2: Vec<usize> = Vec::with_capacity(len1 + 1);

    for (i, ch2) in str2.chars().enumerate() {
        row2.push(i+1);

        for (j, ch1) in str1.chars().enumerate() {
            let del_cost = row1[j+1] + 1;
            let ins_cost = row2[j] + 1;
            let subst_cost = row1[j] + if ch1 != ch2 { 1 } else { 0 };

            row2.push(min(subst_cost, min(del_cost, ins_cost)));
        }

        // keep row2 results for new iteration, reuse row1
        swap(&mut row1, &mut row2);
        row2.clear();

    }
    row1[len1]
}