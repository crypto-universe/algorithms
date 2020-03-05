#[allow(unused_imports)]
use levenshtein;

/*
// Core i5-7300U 2.60GHz:
// test levenshtein_test::levenshtein_distance_1_bench_1 ... bench: 12,238,653,260 ns/iter (+/- 1,903,572,519)
#[bench]
fn levenshtein_distance_1_bench_1(benchmark: &mut test::Bencher) {
    let input1: String = std::fs::read_to_string("test_data/2012usc09.htm").unwrap();
    let input2: String = std::fs::read_to_string("test_data/2018usc09.htm").unwrap();
    benchmark.iter(|| levenshtein::levenshtein_distance_1(&input1, &input2))
}
*/

#[test]
fn levenshtein_distance_1_test_1() {
    let result1 = levenshtein::levenshtein_distance_1("sitting", "kitten");
    assert!(result1 == 3);

    let result2 = levenshtein::levenshtein_distance_1("kitten", "sitting");
    assert!(result2 == 3);
}

#[test]
fn levenshtein_distance_1_test_2() {
    let result1 = levenshtein::levenshtein_distance_1("Saturday", "Sunday");
    assert!(result1 == 3);

    let result2 = levenshtein::levenshtein_distance_1("Sunday", "Saturday");
    assert!(result2 == 3);
}

#[test]
fn levenshtein_distance_1_test_3() {
    let result1 = levenshtein::levenshtein_distance_1("interest", "industry");
    dbg!(result1);
    assert!(result1 == 6);

    let result2 = levenshtein::levenshtein_distance_1("industry", "interest");
    assert!(result2 == 6);
}

#[test]
fn levenshtein_distance_1_test_4() {
    let result1 = levenshtein::levenshtein_distance_1("GUMBO", "GAMBOL");
    dbg!(result1);
    assert!(result1 == 2);

    let result2 = levenshtein::levenshtein_distance_1("GAMBOL", "GUMBO");
    assert!(result2 == 2);
}