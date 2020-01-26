pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: isize
}

pub fn min_distances(num_of_vertices: usize, starting_vertex:usize, edges: &[Edge]) -> Result<Vec<isize>, &str> {
    // init vector of distances as infinite 
    let mut distances: Vec<Option<isize>> = [Option::None].repeat(num_of_vertices);
    // distance to self is 0
    distances[starting_vertex] = Option::Some(0);

    // O(edges * vertices)
    for _i in 0..num_of_vertices-1 {
        for Edge{from, to, weight} in edges {
            // proceed only if source vertex has a distance
            if let Some(y) = distances[*from] {
                let new_dist = y + *weight;
                distances[*to] = match distances[*to] {
                    Some(x) => Option::Some(std::cmp::min(x, new_dist)), // assign minimal of 2 distances
                    None    => Option::Some(new_dist), // current distance is infinity, any other is applicable
                };
            }
        }
    }

    // detect a negative weight cycle, complexity O(E)
    for Edge{from, to, weight} in edges {
        if let Some(y) = distances[*from] {
            let new_dist = y + *weight;
            match distances[*to] {
                Some(x) => {
                    if x > new_dist {
                        return Err("Negative weight cycle detected!");
                    }
                },
                None => { return Err("The destination vertex is unreachable. Is it disconnected?"); }
            };
        } else {
            return Err("The source vertex is unreachable. Is it disconnected?");
        }
    }

    // Is there any None in distances vector? If so, that vertex is unreachable from given source.
    if distances.iter().any(|&x| x.is_none()) {
        return Err("Disconnected vertex found.");
    }

    // unwrap options
    Ok(distances.iter().map(|distance| distance.unwrap()).collect())
}