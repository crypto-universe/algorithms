use std::collections::HashMap;

// A variation of kahn's topological sort.
// Instead of removing edges this function decreases counter of incoming edges for each node.
// Advantages:
//   - graph is immutable and can be used later
// Disadvantages:
//   - space overhead for one HashMap
//   - sort initial sources vector because HashMap doesn't guarantee iteration order (may be disabled by stable_result argument)
pub fn kahns_sort<S, T>(dag_as_adjacency_list: &HashMap<T, Vec<T>, S>, stable_result: bool) -> Result<Vec<T>, &'static str>
where
    S: std::hash::BuildHasher,
    T: std::cmp::Ord + std::hash::Hash + Copy,
{
    let mut num_of_incoming_edges: HashMap<T, usize> = HashMap::new();
    for (node, list) in dag_as_adjacency_list {
        if num_of_incoming_edges.get_mut(&node).is_none() {
            // node was not in the HashMap
            num_of_incoming_edges.insert(*node, 0);
        }
        for dst in list {
            match num_of_incoming_edges.get_mut(&dst) {
                Some(v) => {
                    *v += 1;
                },
                None => {
                    num_of_incoming_edges.insert(*dst, 1);
                }
            }
        }
    }

    let mut sorted_nodes: Vec<T> = Vec::with_capacity(num_of_incoming_edges.len());

    // find all source nodes
    let mut source_nodes: Vec<T> =
        num_of_incoming_edges.iter().filter(|(_, b)| **b == 0).map(|(n, _)| *n).collect();

    // sort initial vector is a must because num_of_incoming_edges has no stable order 
    // topological sorting result will be different on each run (but still correct).
    if stable_result {
        source_nodes.sort();
    }

    while !source_nodes.is_empty() {
        let current_node = source_nodes.pop().unwrap();
        sorted_nodes.push(current_node);

        if let Some(list) = dag_as_adjacency_list.get(&current_node) {
            for node in list {
                if let Some(v) = num_of_incoming_edges.get_mut(node) {
                    if *v == 0 {
                        return Err("Internal error!");
                    }
                    *v -= 1;
                    if *v == 0 {
                        source_nodes.push(*node);
                    }
                }
            }
        }
    }

    let is_dag = num_of_incoming_edges.iter().any(|(_k, &v)| v != 0);
    if is_dag {
        Err("Provided graph has cycles!")
    } else {
        Ok(sorted_nodes)
    }
}