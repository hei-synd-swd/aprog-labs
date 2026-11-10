# Reading Compiler Errors

Rust pushes as many bugs as possible to **compile time**. A typical error looks like this:

```text
error[E0308]: mismatched types
  --> main.rs:10:25
   |
10 | const DUTY_CYCLE: f64 = 1 / 2;
   |                   ---   ^^^^^ expected `f64`, found integer
   |                   |
   |                   expected because of the type of the constant
```


You can make fixing easier by using `rustc --explain E0308` which prints a longer explanation of a specific error code.

The compilers suggestions are guesses. This means that its suggestion may be incorrect, change behavioour, not fix the compilation error, etc.
