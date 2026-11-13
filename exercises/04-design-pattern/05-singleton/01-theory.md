# Singleton Pattern

A singleton ensures that a type has **only one instance** and provides a **single access point**.

Singletons are typically implemented with `OnceLock`.

```rust
use std::sync::OnceLock;

struct Config {
    db_url: String,
}

impl Config {
    fn global() -> &'static Config {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| Config {
            db_url: "postgres://localhost/mydb".to_string(),
        })
    }
}

fn main() {
    let cfg = Config::global();
    println!("DB URL: {}", cfg.db_url);
}
```

`OnceLock` guarantees the initialization closure runs **exactly once**.
