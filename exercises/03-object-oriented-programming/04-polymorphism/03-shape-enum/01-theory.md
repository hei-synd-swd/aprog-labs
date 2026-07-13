# Enum-Based Polymorphism

Instead of trait objects, use an **enum** to group a fixed set of types and implement the trait on the enum:

```rust
trait Media { fn title(&self) -> &str; }

struct Book { title: String, pages: u32 }
struct Movie { title: String, runtime_minutes: u32 }

impl Media for Book  { fn title(&self) -> &str { &self.title } }
impl Media for Movie { fn title(&self) -> &str { &self.title } }

enum MediaEnum { Book(Book), Movie(Movie) }

impl Media for MediaEnum {
    fn title(&self) -> &str {
        match self {
            MediaEnum::Book(b) => b.title(),
            MediaEnum::Movie(m) => m.title(),
        }
    }
}

let items: Vec<MediaEnum> = vec![
    MediaEnum::Book(Book { title: "Dune".into(), pages: 412 }),
    MediaEnum::Movie(Movie { title: "Inception".into(), runtime_minutes: 148 }),
];
// items.iter().map(|m| m.title())  // no match needed — enum impl handles it
```

| | Enum | `Box<dyn Trait>` |
|---|---|---|
| Dispatch | Compile time (match) | Runtime (vtable) |
| Type set | Fixed (closed) | Open (any impl can be added) |
| Allocation | No heap allocation | Heap + vtable pointer |

**Use enums when the set of types is known and fixed.** Use trait objects when you need extensibility.
