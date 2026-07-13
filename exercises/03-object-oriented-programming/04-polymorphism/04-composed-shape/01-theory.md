# Composition + Trait Objects

Combine **composition** (has-a) with **trait objects** (dynamic dispatch) for flexible structures:

```rust
trait Widget { fn render(&self) -> String; }

struct Panel {
    children: Vec<Box<dyn Widget>>,
    title: String,
}

impl Panel {
    fn add(&mut self, w: Box<dyn Widget>) {
        self.children.push(w);
    }
    fn render_all(&self) -> String {
        self.children.iter().map(|w| w.render()).collect()
    }
}
```

- **Extensibility** — new widget types don't require changing `Panel`
- **Encapsulation** — `Panel` knows only the trait contract
- **Composition over inheritance** — `Panel` *contains* widgets; it doesn't *inherit* from them
