---
id          = "impp_dfs"
name        = "Depth-First Search"
language    = "rust"
difficulty  = 4
description = "Find all reachable nodes in a graph using DFS."
topics      = ["recursion", "graph", "dfs"]
---

# Depth-First Search

A graph can be represented as a **HashMap**: `HashMap<u32, Vec<u32>>`
where each key is a node and the value is the list of its neighbors.

Write two functions:

1. `dfs(graph: &HashMap<u32, Vec<u32>>, node: u32, visited: &mut HashSet<u32>)` —
   inserts `node` into `visited`, then recursively visits all unvisited neighbors.

2. `reachable_nodes(graph: &HashMap<u32, Vec<u32>>, start: u32) -> Vec<u32>` —
   creates a `HashSet`, calls `dfs`, then returns all visited nodes as a sorted `Vec`.

## Expected Result

```
let graph = HashMap::from([
    (0, vec![1, 2]),
    (1, vec![0, 3]),
    (2, vec![0]),
    (3, vec![1]),
]);
reachable_nodes(&graph, 0) → [0, 1, 2, 3]
```
