// Given string s, find the shortest substring that contains all chars appeared in s.

use std::collections::HashSet;

pub fn shortest_substring(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }
    
    let mut unique_chars: HashSet<char> = HashSet::new();
    for ch in s.chars() {
        unique_chars.insert(ch);
    }
    
    let num_of_elements = unique_chars.len();
    if num_of_elements == 1 {
        return &s[0..1];
    }

    let mut solutions: Vec<&str> = Vec::new();
    
    let mut temp_chars: HashSet<char> = HashSet::with_capacity(num_of_elements);
    let mut start_index: usize = 0;
    let mut index_of_shortest_substr: usize = 0;

    loop {
        temp_chars.clear();
        for (i, ch) in s[start_index..].chars().enumerate() {
            temp_chars.insert(ch);
            if temp_chars.len() == num_of_elements {
                let end_index = start_index + i;
                // we call go_backwards with the slice shorter by 1 element because current "ch"
                // definitely belongs to the substring
                let offset_in_given_slice = go_backwards(&s[start_index..end_index], num_of_elements-1);
                let new_start_index = start_index + offset_in_given_slice;
                // end_index+1 to include last element
                solutions.push(&s[new_start_index..end_index+1]);
                if end_index - new_start_index < solutions[index_of_shortest_substr].len() {
                    index_of_shortest_substr = solutions.len() - 1;
                }
                start_index = new_start_index + 1;
                break;
            }
        }
        if temp_chars.len() != num_of_elements {break;}
    }
    solutions[index_of_shortest_substr]
}

// Return start index of the shortest substring in given slice.
fn go_backwards(s: &str, num_of_el: usize) -> usize {
    let mut temp_chars: HashSet<char> = HashSet::with_capacity(num_of_el);
    for (i, ch) in s.chars().rev().enumerate() {
        temp_chars.insert(ch);
        if temp_chars.len() == num_of_el {
            return s.len()-i-1;
        }
    }
    panic!("There is no substring, although it MUST be there!");
}