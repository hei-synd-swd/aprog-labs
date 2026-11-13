# Questions: Choose the Smart Pointer

For each situation, name the smart pointer type and give the reason in one sentence (how many owners, how many threads).

1. A binary tree: every node contains up to two child nodes of the same type. There is one owner per node.
2. A read-only configuration that several parts of a **single-threaded** program own together.
3. The same read-only configuration, now owned by **several threads**.
4. A hit counter that **several threads** own together and all **increment**.
5. A child node that must refer to its parent, but must **not** keep the parent alive.

<!-- Write your answer below -->
