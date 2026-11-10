---
id          = "debug_borrow_errors"
name        = "Fix the Borrow Errors"
language    = "rust"
difficulty  = 3
description = "Fix four ownership and borrow-checker errors in a library book-log program by following rustc's move analysis."
topics      = ["debugging", "ownership", "borrowing", "E0382", "E0499", "E0106"]
---

# Fix the Borrow Errors

A book-tracking program fails to compile with **three ownership / borrow errors**. All typing is correct this time. The bugs are all about *ownership*.

Check the debug tab after every save and work through the remaining errors from top to bottom.

## Expected Result

The file compiles with no errors or warnings and all unit tests pass.

