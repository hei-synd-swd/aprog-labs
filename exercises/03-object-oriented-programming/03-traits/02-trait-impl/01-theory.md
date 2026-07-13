# Implementing a Trait on Multiple Types

The real power of traits is that **different types can implement the same
trait**, each with their own behaviour.

```rust
struct Article { title: String, word_count: u32 }
struct Video   { title: String, duration_secs: u32 }

trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("Article: {} ({} words)", self.title, self.word_count)
    }
}

impl Summary for Video {
    fn summary(&self) -> String {
        format!("Video: {} ({} sec)", self.title, self.duration_secs)
    }
}
```

Now both `Article` and `Video` have a `.summary()` method, each producing
output appropriate for its type.

> **Orphan rule:** You can implement a trait on a type only if at least one
> of the trait or the type is local to your crate.
