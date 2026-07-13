use std::collections::{HashMap, HashSet};

// TODO: `dfs(graph: &HashMap<u32, Vec<u32>>, node: u32, visited: &mut HashSet<u32>)`
// Insert node into visited, then recursively visit unvisited neighbors.
// fn dfs(...) { ... }

// TODO: `reachable_nodes(graph: &HashMap<u32, Vec<u32>>, start: u32) -> Vec<u32>`
// Create a HashSet, call dfs, convert to sorted Vec.
// fn reachable_nodes(...) -> Vec<u32> { ... }

fn main() {
    let graph = HashMap::from([
        (0, vec![1, 2]),
        (1, vec![0, 3]),
        (2, vec![0, 4]),
        (3, vec![1]),
        (4, vec![2]),
        (5, vec![]),
    ]);
    println!("{:?}", reachable_nodes(&graph, 0));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_graph() {
        let graph = HashMap::from([(0, vec![1, 2]), (1, vec![0, 3]), (2, vec![0]), (3, vec![1])]);
        let mut reachable = reachable_nodes(&graph, 0);
        reachable.sort();
        assert_eq!(reachable, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_disconnected() {
        let graph = HashMap::from([(0, vec![1]), (1, vec![0]), (2, vec![]), (3, vec![])]);
        let mut reachable = reachable_nodes(&graph, 0);
        reachable.sort();
        assert_eq!(reachable, vec![0, 1]);
    }

    #[test]
    fn test_single_node() {
        let graph = HashMap::from([(0, vec![])]);
        assert_eq!(reachable_nodes(&graph, 0), vec![0]);
    }
}
