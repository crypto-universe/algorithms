#[allow(unused_imports)] // only for twrqs
use twrqs;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[bench]
fn bench_twrqs_1(benchmark: &mut test::Bencher) {
    let input: Vec<String> = my_read_file("input_16_10000.txt");
    let input_u8 = input.iter().map(|x| x.as_str().as_bytes()).collect::<Vec<&[u8]>>();
    benchmark.iter(|| twrqs::sort(input_u8.clone().as_mut_slice()))
}

#[bench]
fn bench_standard_1(benchmark: &mut test::Bencher) {
    let input: Vec<String> = my_read_file("input_16_10000.txt");
    let input_u8 = input.iter().map(|x| x.as_str().as_bytes()).collect::<Vec<&[u8]>>();
    benchmark.iter(|| input_u8.clone().as_mut_slice().sort())
}

#[test]
fn test_1() {
    let b_temp = &mut ["fat", "run", "big", "age", "bar", "rim", "goo", "foo", "bit", "ace"];
    let rez_tmp = &mut ["ace", "age", "bar", "big", "bit", "fat", "foo", "goo", "rim", "run"];
    let mut b = b_temp.iter().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();
    let rez = rez_tmp.iter().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();
    twrqs::sort(b.as_mut_slice());
    assert_eq!(rez, b);
}

#[test]
fn test_2() {
    let input: Vec<String> = my_read_file("input_16_10000.txt");
    let mut input_u8 = input.iter().map(|x| x.as_str().as_bytes()).collect::<Vec<&[u8]>>();
    let mut expected = input_u8.clone();
    expected.as_mut_slice().sort();
    twrqs::sort(input_u8.as_mut_slice());
    //Compare results from standard sort and current implementation of 3-way radix qsort
    assert!(input_u8.iter().zip(expected.iter()).all(|(a,b)| a == b));
}

// Internal function to read file into vector.
#[allow(dead_code)]
fn my_read_file(file: &str) -> Vec<String> {
    let file_in_dir = "test_data/".to_string() + file;
    let path = Path::new(&file_in_dir);

    let mut s = String::new();

    let mut reader = BufReader::new(File::open(&path).unwrap());
    let _r = reader.read_line(&mut s);
    let size: usize = s.trim().parse().ok().unwrap();

    let mut result: Vec<String> = Vec::with_capacity(size);

    for line in reader.lines() {
        match line{
            Err(why) => panic!("Bad line: {:?}", why),
            Ok(clean_line) => {result.push(clean_line);},
        }
    }

    result
}