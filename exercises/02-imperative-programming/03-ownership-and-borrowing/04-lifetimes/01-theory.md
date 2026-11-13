# Lifetimes

A reference must never outlive the value it borrows. The compiler checks this with **lifetimes**: the span for which a reference is valid. Most of the time it works them out by itself.

It needs help when a function **returns a reference** and has several reference parameters. Then it cannot tell which parameter the result borrows from, and reports `error[E0106]: missing lifetime specifier`.

```rust
// One reference parameter: the result can only borrow from `text`.
// No annotation needed.
fn trimmed(text: &str) -> &str {
    text.trim()
}

// The result is `a` or `b`: both are tied to the result with 'a.
fn earlier<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a <= b { a } else { b }
}

// The result always points into `text`, never into `marker`:
// only `text` is tied to the result.
fn before<'a>(text: &'a str, marker: &str) -> &'a str {
    match text.split_once(marker) {
        Some((head, _)) => head,
        None => text,
    }
}
```

## Reading the annotation

- `<'a>` after the function name **declares** a lifetime called `a`.
- `&'a str` is a reference that is valid for `'a`.
- The same `'a` on a parameter and on the return type means: the result borrows from this parameter, so the result must not be used after that value is gone.

## What it does and does not do

- An annotation **describes** a relation. It does not make any value live longer.
- Give `'a` only to the parameters the result can come from. A parameter without it is free: the caller may drop that value while still using the result.
- Lifetimes are checked at compile time only and cost nothing at run time.
