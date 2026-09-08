# RAII Pattern

**RAII** (*Resource Acquisition Is Initialisation*) ties a resource's lifetime to an object's lifetime: acquire the resource when the object is created, and release it automatically when the object is **dropped**.

```rust
struct FileHandle {
    name: String,
}

impl FileHandle {
    fn open(name: &str) -> Self {
        println!("Opening {name}");
        FileHandle { name: name.to_owned() }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing {}", self.name); // runs automatically
    }
}

fn main() {
    {
        let _f = FileHandle::open("data.txt");
    } // `_f` leaves scope here -> drop() runs
    println!("File already closed");
}
```

Rust calls `drop()` for you at the end of the scope - even on an early return or a panic - so cleanup is never forgotten.
