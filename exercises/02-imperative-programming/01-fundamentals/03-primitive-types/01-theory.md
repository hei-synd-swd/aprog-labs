# Primitive Types

Rust's scalar types include integers, floats, booleans, and characters.

**Integers** expose their bounds as *associated constants*:

```rust
u8::MIN   // 0
u8::MAX   // 255
i32::MAX  // 2_147_483_647
u8::BITS  // 8
```

**`char`** is a single Unicode scalar value and has many query methods:

```rust
'a'.is_alphabetic()   // true
'5'.is_ascii_digit()  // true
'A'.is_uppercase()    // true
' '.is_whitespace()   // true
```

**`bool`** holds `true` or `false` and combines with logical operators:

```rust
let ok = a && b;  // AND
let any = a || b; // OR
let no = !a;      // NOT
```

Use the associated constants and built-in methods rather than hard-coding
values like `255` — they are clearer and adapt to the type.
