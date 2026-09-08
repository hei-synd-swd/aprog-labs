# Finding a Maximum

To find the largest value in a slice, keep a **running maximum**: start with the first element, then compare every other element against it and update whenever you find a bigger one. A single pass is enough - O(n).

An empty slice has no maximum, so the result is an `Option`: return `None` when there is nothing to compare, and `Some(largest)` otherwise. Returning an `Option` forces the caller to handle the empty case.
