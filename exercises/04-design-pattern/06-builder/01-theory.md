# Builder Pattern

The **builder** pattern constructs a complex object step by step, instead of passing every field to one big constructor. A separate builder holds a work-in-progress value and exposes one method per option.

```rust
#[derive(Default)]
struct Server {
    host: String,
    port: u16,
}

struct ServerBuilder {
    server: Server,
}

impl ServerBuilder {
    fn new() -> Self {
        Self { server: Server::default() }
    }
    fn host(mut self, h: &str) -> Self {
        self.server.host = h.into();
        self // return self to allow chaining
    }
    fn build(self) -> Server {
        self.server
    }
}

let s = ServerBuilder::new().host("localhost").build();
```

Each method takes `self` **by value** and returns `Self`, so calls can be **chained**; `build` consumes the builder and hands back the finished object.
