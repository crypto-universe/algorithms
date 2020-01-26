#[allow(unused_imports)]
use bellman_ford::{Edge, min_distances};

#[test]
fn bellman_ford_test_1() {
    let input = [Edge{from:0, to:1, weight:1},
                 Edge{from:1, to:2, weight:-1},
                 Edge{from:2, to:3, weight:-1},
                 Edge{from:3, to:0, weight:-1}];
    let result = min_distances(4, 0, &input);
    assert_eq!(result, Result::Err("Negative weight cycle detected!"));
}

#[test]
fn bellman_ford_test_2() {
    let input = [Edge{from:0, to:1, weight:-1},
                 Edge{from:0, to:2, weight:4},
                 Edge{from:1, to:2, weight:3},
                 Edge{from:1, to:3, weight:2},
                 Edge{from:1, to:4, weight:2},
                 Edge{from:3, to:2, weight:5},
                 Edge{from:3, to:1, weight:1},
                 Edge{from:4, to:3, weight:-3},
                 ];
    let result = min_distances(5, 0, &input);
    assert_eq!(result, Result::Ok([0, -1, 2, -2, 1].to_vec()));
}

#[test]
fn bellman_ford_test_3() {
    let input = [Edge{from:0, to:2, weight:4},
                 Edge{from:2, to:3, weight:5},
                 Edge{from:3, to:0, weight:6}];
    let result = min_distances(4, 2, &input);
    // Vertex 1 is disconnected
    assert_eq!(result, Result::Err("Disconnected vertex found."));
}

#[test]
fn bellman_ford_test_4() {
    let input = [Edge{from:1, to:2, weight:4},
                 Edge{from:2, to:1, weight:3}];
    let result = min_distances(3, 0, &input);
    // Vertex 0 is disconnected
    assert_eq!(result, Result::Err("The source vertex is unreachable. Is it disconnected?"));
}

#[test]
fn bellman_ford_test_5() {
    let input = [Edge{from:0, to:1, weight:4},
                 Edge{from:5, to:1, weight:3},
                 Edge{from:0, to:2, weight:4},
                 Edge{from:2, to:5, weight:-2},
                 Edge{from:5, to:4, weight:-3},
                 Edge{from:6, to:5, weight:2},
                 Edge{from:2, to:0, weight:1},
                 Edge{from:3, to:0, weight:3},
                 Edge{from:3, to:2, weight:2},
                 Edge{from:4, to:3, weight:1},
                 // negative weight cycle here
                 Edge{from:4, to:6, weight:-2},
                 Edge{from:6, to:7, weight:2},
                 Edge{from:7, to:4, weight:-2},
                 ];
    let result = min_distances(8, 0, &input);
    assert_eq!(result, Result::Err("Negative weight cycle detected!"));
}

#[test]
fn bellman_ford_test_6() {
    let input = [Edge{from:6, to:1, weight:6},
                 Edge{from:6, to:2, weight:5},
                 Edge{from:6, to:3, weight:5},
                 Edge{from:1, to:4, weight:-1},
                 Edge{from:2, to:4, weight:1},
                 Edge{from:2, to:1, weight:-2},
                 Edge{from:3, to:2, weight:-2},
                 Edge{from:3, to:5, weight:-1},
                 Edge{from:4, to:0, weight:3},
                 Edge{from:5, to:0, weight:3}
                 ];
    let result = min_distances(7, 6, &input);
    assert_eq!(result, Ok([3, 1, 3, 5, 0, 4, 0].to_vec()));
}