# Depth-First Search

DFS is a graph traversal algorithm that explores as far as possible
along each branch before backtracking.

```rust
fn dfs(graph: &HashMap<u32, Vec<u32>>, node: u32, visited: &mut HashSet<u32>) {
    visited.insert(node);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs(graph, neighbor, visited);
            }
        }
    }
}
```

The order of visiting neighbors determines the traversal order but
all reachable nodes are the same regardless.

## Hash graph

A graph is stored as `HashMap<u32, Vec<u32>>` where each key is a node
and the value is its list of neighbors. Using `HashMap` and `HashSet`
avoids manual index casting and works with any node IDs.
