---
id          = "impp_lifetimes"
name        = "Lifetimes"
language    = "rust"
difficulty  = 4
description = "Add lifetime annotations to functions that return a reference."
topics      = ["lifetimes", "references", "borrowing", "annotation", "E0106"]
---

# Lifetimes

Three helpers take a log line such as `"WARN: disk almost full"` apart. All of them return a **reference** into their input, no text is copied.

The file does **not compile** at first: two functions return a reference but have two reference parameters, so the compiler asks for lifetime annotations (`E0106`).

## Your Task

Complete the three functions. Do not change the parameter names, the parameter order, `main` or the tests.

**`level(line: &str) -> &str`** returns the part before the first `:`. Without a `:` it returns the whole line.

1. The signature stays as it is. With a single reference parameter the compiler knows where the result comes from.
2. Replace the `todo!()` with a `match line.split_once(':')`. The arm `Some((level, _))` returns `level`, the arm `None` returns `line`.

**`message_or`** returns the part after the first `": "`. Without a `": "` it returns `fallback`.

1. The result is either a part of `line` or `fallback`, so **both** parameters are tied to the result. Change the signature to `fn message_or<'a>(line: &'a str, fallback: &'a str) -> &'a str`.
2. Replace the `todo!()` with a `match line.split_once(": ")`. The arm `Some((_, message))` returns `message`, the arm `None` returns `fallback`.

**`strip_tag`** removes `tag` from the start of the line. If the line does not start with `tag`, it returns the line unchanged.

1. The result always points into `line`, never into `tag`. Declare `<'a>` and put `'a` on `line` and on the return type only. `tag` keeps the plain type `&str`.
2. Replace the `todo!()` with `line.strip_prefix(tag).unwrap_or(line)`.

```
level("WARN: disk almost full")             → "WARN"
level("plain text")                         → "plain text"
message_or("WARN: disk almost full", "n/a") → "disk almost full"
message_or("broken line", "n/a")            → "n/a"
strip_tag("[db] connected", "[db] ")        → "connected"
strip_tag("connected", "[db] ")             → "connected"
```

## Expected Result

- The file compiles: `message_or` and `strip_tag` carry lifetime annotations, `level` has none.
- All three functions return the parts shown above.
- The result of `strip_tag` stays usable after the `tag` value has been dropped. A test checks this, and it only compiles if `tag` has **no** `'a`.
- `main` prints `WARN`, `disk almost full`, `n/a` and `connected`.
