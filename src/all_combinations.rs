pub fn combinations_copy<T: Copy>(input: &[T], current_answer: &[T]) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    for (i, &e) in input.iter().enumerate() {
        let v: Vec<T> = current_answer.iter().chain(std::iter::once(&e)).copied().collect();
        if input.len() > 1 && i < input.len()-1 {
            let mut r = combinations_copy(&input[i+1..], &v);
            result.append(&mut r);
        }
        result.push(v);
    }
    result
}

pub fn combinations_borrow<'v, T>(input: &'v [T], mut current_answer: Vec<&'v T>) -> Vec<Vec<&'v T>> {
    let mut result: Vec<Vec<&'v T>> = Vec::new();
    for (i, e) in input.iter().enumerate() {
        current_answer.push(&e);
        if input.len() > 1 && i < input.len()-1 {
            let mut r = combinations_borrow(&input[i+1..], current_answer.to_owned());
            result.append(&mut r);
        }
        result.push(current_answer.to_owned());
        current_answer.pop();
    }
    result
}

pub fn combinations_iter<T>(source: &[T]) -> impl Iterator<Item=Vec<&T>> {
    CombinationsIter{
        elements: source,
        stack: Vec::with_capacity(source.len())
    }
}

pub struct CombinationsIter<'a, T> {
    elements: &'a [T],
    stack: Vec<(usize, &'a T)>,
}

impl <'a, T>Iterator for CombinationsIter<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        let slen = self.stack.len();
        let elen = self.elements.len();
        if elen == slen /*|| elen == 0*/ {
            // either no elements to iterate or all iterations are done
            return None;
        }
        if self.stack.is_empty() {
            // first iteration, nothing in the stack
            self.stack.push((0, &self.elements[0]));
            return Some(vec![&self.elements[0]]);
        }

        if let Some((index, _el)) = self.stack.first() {
            if *index == (elen - slen) {
                // iterated over all combinations of given length, lets increase length
                self.stack.clear();
                self.stack = self.elements[0..slen+1].iter().enumerate().collect();
                return Some(self.elements[0..slen+1].iter().map(|a| a).collect());
            }
        }
        
        let mut last_popped_index = 0;
        for num_of_pops in 1..(slen + 1) {
            match self.stack.pop() {
                Some((index, _element)) => {
                    last_popped_index = index;
                    if index < (elen - num_of_pops) {
                        for i in (index + 1)..(index + 1 + num_of_pops) {
                            self.stack.push((i, &self.elements[i]));
                        }
                        break;
                    }
                },
                None => {
                    self.stack = self.elements[(last_popped_index + 1)..(last_popped_index + 1 + slen)].iter().enumerate().collect();
                    break;
                }
            }
        }
        let mut result: Vec<&'a T> = Vec::with_capacity(slen);
        for (_index, element) in &self.stack {
            result.push(element);
        }
        Some(result)
    }
}
