# Tree Recursion

A recursive function that calls itself **more than once** creates a
**tree of calls**. Each branch of the tree solves a smaller subproblem,
and the results are combined when the calls return.

```rust
fn count_branches(depth: u32) -> u32 {
    if depth == 0 {
        1
    } else {
        count_branches(depth - 1)   // first branch
            + count_branches(depth - 1) // second branch
    }
}
```

Every call to `count_branches(n)` produces two recursive calls, forming
a binary tree. The number of calls grows exponentially — `2ⁿ⁺¹ − 1`
total calls for depth `n`.

## When to use tree recursion

Tree recursion is useful when a problem can be broken into **multiple
independent subproblems** of the same kind, such as:

- Exploring branches in a tree or graph
- Divide-and-conquer algorithms (merge sort, quicksort)
- Combinatorial puzzles where each choice leads to new choices

The key difference from linear recursion (a single recursive call) is
that the call graph branches, so the total work grows faster than the
input size.
