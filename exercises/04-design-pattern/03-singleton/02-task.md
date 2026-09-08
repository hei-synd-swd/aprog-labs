---
id          = "singleton"
name        = "Singleton Pattern"
language    = "rust"
difficulty  = 3
description = "Ensure a type has exactly one global instance."
topics      = ["struct", "once_lock", "singleton"]
---

# Singleton Pattern

Provide a single, globally accessible `Logger`. The `level()` getter is already written for you.

## Your Task

1. Give `Logger` a `String` field named `level`.
2. Implement `Logger::global() -> &'static Logger`. Inside it, declare a `static INSTANCE: OnceLock<Logger>` and use `get_or_init` to build the logger once with level `"INFO"`.
3. In `main`, call `Logger::global()` twice (the second time inside an inner block) and print the level each time.

## Expected Result

Both calls return the **same** instance, so the level prints as `INFO` both times. The closure passed to `get_or_init` runs only once, no matter how often `global()` is called.
