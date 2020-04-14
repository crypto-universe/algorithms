# algorithms

[![Build Status](https://travis-ci.org/crypto-universe/algorithms.svg?branch=master)](https://travis-ci.org/crypto-universe/algorithms)
![Rust CI](https://github.com/crypto-universe/algorithms/workflows/Rust/badge.svg?branch=master)
[![license](https://img.shields.io/github/license/mashape/apistatus.svg?maxAge=2592000)](https://mit-license.org/)

Just some algorithms, implemented for fun. See unit tests to figure out how to use these functions.

- [bellman_ford.rs](../master/src/bellman_ford.rs) Bellman-Ford algorithm searches minimal distances in graph that may contain negative weight edges
- [kmp.rs](../master/src/kmp.rs) Knuth-Morris-Pratt string-searching algorithm
- [levenshtein.rs](../master/src/levenshtein.rs) Levenshtein (edit) distance between 2 strings
- [sh_sub.rs](../master/src/sh_sub.rs) Find the shortest substring that contains all chars appeared in given string
- [topological_sorting.rs](../master/src/topological_sorting.rs) Kahn's algorithm for topological sorting.
- [twrqs.rs](../master/src/twrqs.rs) Three-way radix qsort with Dutch national flag splitting algorithm

## How to run

```bash
# you need nightly toolchain to run tests
cargo clippy
cargo test
cargo bench
```

## License

See the [LICENSE](LICENSE.txt) file for license rights and limitations (MIT).

## Test and benchmark data

[United States Code annual historical archives](https://uscode.house.gov/download/annualhistoricalarchives/downloadxhtml.shtml)
