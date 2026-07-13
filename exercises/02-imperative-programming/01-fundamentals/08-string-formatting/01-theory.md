# String Formatting

`format!` builds a `String` using the same syntax as `println!`. A format
specifier goes inside `{}` after a colon:

```
{[argument]:[fill][align][sign][#][0][width][.precision][type]}
```

Useful pieces:

```rust
format!("{:5}", 42);     // "   42"   width 5, right-aligned
format!("{:<5}", 42);    // "42   "   left-aligned
format!("{:05}", 42);    // "00042"   zero-padded to width 5
format!("{:.2}", 3.14159); // "3.14"  two decimal places
```

Number bases and the `#` "alternate" flag (adds the `0x` / `0b` prefix):

```rust
format!("{:b}", 5);      // "101"
format!("{:08b}", 5);    // "00000101"
format!("{:x}", 255);    // "ff"
format!("{:#06x}", 255); // "0x00ff"  (# adds 0x; width counts the prefix)
```

You can also use inline arguments: `format!("{n:08}")`.
