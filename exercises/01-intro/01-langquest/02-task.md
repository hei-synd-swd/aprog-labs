---
id          = "intro_langquest"
name        = "Getting Started with lq"
language    = "rust"
difficulty  = 1
description = "Learn how LangQuest works by fixing a small Hello World program."
topics      = ["langquest", "hello-world", "functions", "tests"]
---

# Getting Started with LangQuest

Time to try the workflow for real. The program on the **Output** page does not
pass its tests yet - two functions are broken. Fix them so **all four tests**
turn green.

Open the file in your editor with `e`, make your changes, and **save**.
LangQuest re-runs the tests automatically; check the **Output** page for your
score and the **Debug** page for compiler messages.

## Your Task

1. `hello()` must return exactly the string `"Hello, world!"`.
2. `greet(name)` must return a personal greeting of the form `"Hello, <name>!"`
   - for example `greet("Ada")` returns `"Hello, Ada!"`.

Do **not** change anything below the `DO NOT EDIT BELOW THIS LINE` banner: that
is where the unit tests live.

## Expected Result

All four tests pass:

- `hello()` equals `"Hello, world!"`
- `greet("Ada")` equals `"Hello, Ada!"`
- `greet("HEI-Vs")` equals `"Hello, HEI-Vs!"`
- `greet("")` equals `"Hello, !"`

Stuck? Press `h` for a hint - but give it a real try first.
