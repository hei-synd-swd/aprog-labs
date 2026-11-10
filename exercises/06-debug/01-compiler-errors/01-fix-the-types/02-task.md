---
id          = "debug_type_errors"
name        = "Fix the Type Errors"
language    = "rust"
difficulty  = 2
description = "Fix four compile-time type errors in a satellite telemetry program by reading rustc's messages."
topics      = ["debugging", "compiler errors", "types", "E0308"]
---

# Fix the Type Errors

A colleague left you a weather-satellite telemetry helper that does **not compile**. There are four type errors in `main.rs`. Your job is to fix them with the help of *the compilers error messages*.

LangQuest compiles the file on every save and shows the compiler output on the **Debug** page. Read the errors from top to bottom: fix the first error, save, move to the next error if the previous one was fixed.

There are generally no `TODO:`s in `06-debug`. You're supposed to figure out the issues yourself based on compiler errors. Make sure to not change any tests.

## Expected Result

The file compiles with no errors or warnings, the program prints a small telemetry report, and all unit tests pass.

