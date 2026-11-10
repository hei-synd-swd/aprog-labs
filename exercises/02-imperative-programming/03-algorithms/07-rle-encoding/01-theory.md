# Run-Length Encoding

Run-length encoding (RLE) is one of the simplest compression techniques: consecutive repeats of a character are replaced by the count and the character itself. `"aaabbc"` becomes `"3a2b1c"`.

## Building strings from parts

Instead of concatenating with `+` in a loop (which allocates a new `String` every time), push into a `String` or use `push_str`:

```rust
let mut result = String::new();
result.push_str(&count.to_string());
result.push(ch);
```

## Parsing multi-digit counts

When decoding, a count can span several characters (`"12a"`). Loop while the current character is a digit and accumulate the value:

```rust
let mut count = 0usize;
while let Some(c) = chars.next() {
    if c.is_ascii_digit() {
        count = count * 10 + (c as usize - '0' as usize);
    } else {
        // c is the character to repeat `count` times
    }
}
```

Note how `count * 10 + digit` builds the number left to right: `1` → `12`.

## Watch out for

- Empty input: `""` should encode to `""` and decode to `""`.
- The last run: a loop that only appends when the character *changes* must flush the final run after the loop ends.
- `"1"` counts: a run of length 1 still gets an explicit count, which keeps the format unambiguous.