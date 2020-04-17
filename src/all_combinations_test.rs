#[allow(unused_imports)]
use all_combinations;

#[test]
fn all_combinations_copy_1() {
    let result = all_combinations::combinations_copy(&[1, 2, 3], &Vec::new());
    let expected = vec![
        vec![1, 2, 3], vec![1, 2], vec![1, 3], vec![1], vec![2, 3], vec![2], vec![3]
    ];
    assert_eq!(result, expected);
}

#[test]
fn all_combinations_copy_2() {
    let result = all_combinations::combinations_copy(&[22, 88, 44, 66], &Vec::new());
    let expected = vec![
        vec![22, 88, 44, 66], vec![22, 88, 44], vec![22, 88, 66], vec![22, 88], 
        vec![22, 44, 66], vec![22, 44], vec![22, 66], vec![22], vec![88, 44, 66],
        vec![88, 44], vec![88, 66], vec![88], vec![44, 66], vec![44], vec![66]
    ];
    assert_eq!(result, expected);
}

#[test]
fn all_combinations_borrow_1() {
    let result = all_combinations::combinations_borrow(&[1, 2, 3], Vec::new());
    let expected = vec![
        vec![&1, &2, &3], vec![&1, &2], vec![&1, &3], vec![&1], vec![&2, &3], vec![&2], vec![&3]
    ];
    assert_eq!(result, expected);
}

#[test]
fn all_combinations_borrow_2() {
    let result = all_combinations::combinations_borrow(&[22, 88, 44, 66], Vec::new());
    let expected = vec![
        vec![&22, &88, &44, &66], vec![&22, &88, &44], vec![&22, &88, &66], vec![&22, &88], 
        vec![&22, &44, &66], vec![&22, &44], vec![&22, &66], vec![&22], vec![&88, &44, &66],
        vec![&88, &44], vec![&88, &66], vec![&88], vec![&44, &66], vec![&44], vec![&66]
    ];
    assert_eq!(result, expected);
}

#[test]
fn all_combinations_iter_1() {
    let result = all_combinations::combinations_iter(&[1, 2, 3]);
    let expected = vec![
        vec![&1], vec![&2], vec![&3], vec![&1, &2], vec![&1, &3], vec![&2, &3], vec![&1, &2, &3]
    ];

    for (r, e) in result.zip(expected.iter()) {
        assert_eq!(r, *e);
    }
}

#[test]
fn all_combinations_iter_2() {
    let result = all_combinations::combinations_iter(&[22, 88, 44, 66]);
    let expected = vec![
        vec![&22], vec![&88], vec![&44], vec![&66], 
        vec![&22, &88], vec![&22, &44], vec![&22, &66], vec![&88, &44], vec![&88, &66], vec![&44, &66],
        vec![&22, &88, &44], vec![&22, &88, &66], vec![&22, &44, &66], vec![&88, &44, &66],
        vec![&22, &88, &44, &66]
    ];

    for (r, e) in result.zip(expected.iter()) {
        assert_eq!(r, *e);
    }
}

#[test]
fn all_combinations_combined_test() {
    use std::collections::HashSet;
    let input = [5, 7, 34, 1, 90, 6, 44, 32, 8765, 7, 5, 123, 99, 666, 999, 234];
    let result1: HashSet<Vec<&usize>> = all_combinations::combinations_iter(&input).collect();
    let mut result2: HashSet<Vec<&usize>> = HashSet::new();
    for v in all_combinations::combinations_borrow(&input, Vec::new()) {
        result2.insert(v);
    }

    assert_eq!(result1.len(), result2.len());  //65375
    assert_eq!(result1, result2);
}

#[allow(dead_code)]
const BENCH_INPUT: [usize; 14] = [34, 1, 90, 6, 44, 32, 8765, 7, 5, 123, 99, 666, 999, 234];

#[bench]
fn combinations_iter_bench_1(benchmark: &mut test::Bencher) {
    benchmark.iter(|| all_combinations::combinations_iter(&BENCH_INPUT).collect::<Vec<Vec<&usize>>>())
}

#[bench]
fn combinations_borrow_bench_1(benchmark: &mut test::Bencher) {
    benchmark.iter(|| all_combinations::combinations_borrow(&BENCH_INPUT, Vec::new()))
}

#[bench]
fn combinations_copy_bench_1(benchmark: &mut test::Bencher) {
    benchmark.iter(|| all_combinations::combinations_copy(&BENCH_INPUT, &Vec::new()))
}