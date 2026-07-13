# Default Implementations

Traits can provide **default implementations** for methods. Types that
implement the trait can use the default or override it.

```rust
trait Summary {
    fn summary(&self) -> String;

    // Default method — types can use it as-is or override
    fn detailed_summary(&self) -> String {
        format!("Detailed: {}", self.summary())
    }
}
```

Types that implement `Summary` only need to provide `summary` — the default
`detailed_summary` is available automatically. They can also override it:

```rust
struct Tweet { handle: String, content: String }

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.handle, self.content)
    }

    // Override the default
    fn detailed_summary(&self) -> String {
        format!("Tweet from @{}: \"{}\"", self.handle, self.content)
    }
}
```

Defaults are useful for convenience (less boilerplate), evolution (adding a
method doesn't break existing code), and template methods (defaults can call
other trait methods).
