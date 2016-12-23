use std::cmp::Ordering::*;

//Dutch national flag problem - best way to arrange items into 3 groups.
//Complexity O(N)
#[inline]
fn dnf_partition<T>(mut array: &mut [&[T]], mid_val: &T, mut lo: usize, mut hi: usize, index: usize) -> (usize, usize)
        where T: Ord {
    let mut j = lo;
    while j <= hi {
        if let Some(element) = array[j].get(index) {
            match element.cmp(mid_val) {
                Less => {
                    array.swap(lo, j);
                    lo += 1;
                    j += 1;
                },
                Equal => {
                    j += 1;
                },
                Greater => {
                    array.swap(j, hi);
                    hi -= 1;
                },
            }
        }
        else {
            array.swap(lo, j);
            lo += 1;
            j += 1;
        }
    }
    (lo, hi)
}

// Three-way radix qsort with Dutch national flag splitting algorithm
// Return number of iterations (recursive calls in naive implementation)
pub fn sort<T>(mut a: &mut [&[T]]) -> (usize, usize) where T: Ord {
    let mut pivot: &T;
    let mut stack: Vec<(usize, usize, usize)> = Vec::new();

    let mut stack_usage = 0;
    let mut max_stack_size = 1;

    //Init: lower bound, upper bound, index
    stack.push((0, a.len()-1, 0));

    while let Some((lo, hi, index)) = stack.pop() {
        //TODO: Some fancy strategy
        let mut pivot_index = lo;
        while a[pivot_index].len() <= index {
            pivot_index += 1;
        }
        if pivot_index != lo {
            a.swap(lo, pivot_index);
        }
        pivot = a[lo].get(index).unwrap();

        let (left, right) = dnf_partition(a, pivot, lo, hi, index);

        //[left, right]
        match right - left {
            0 => {/*skip, only one element*/},
            1 => {             //two elements
                if a[left][index+1..].cmp(&a[right][index+1..]) == Greater {
                    a.swap(left, right);
                }
            },
            _ => {
                stack.push((left, right, index+1));
            },
        }

        //[lo, left)
        match left - lo {
            0 | 1 => {/*skip, 0 or 1 element*/},
            2 => {                 //two elements
                if a[lo][index..].cmp(&a[left-1][index..]) == Greater {
                    a.swap(lo, left-1);
                }
            },
            _ => {
                stack.push((lo, left-1, index));
            },
        }

        //(right, hi]
        match hi - right {
            0 | 1 => {/*skip, 0 or 1 element*/},
            2 => {                 //two elements
                if a[right+1][index..].cmp(&a[hi][index..]) == Greater {
                    a.swap(right+1, hi);
                }
            },
            _ => {
                stack.push((right+1, hi, index));
            },
        }
        stack_usage += 1;
        if stack.len() > max_stack_size {
            max_stack_size = stack.len();
        }
    }

    (stack_usage, max_stack_size)
}