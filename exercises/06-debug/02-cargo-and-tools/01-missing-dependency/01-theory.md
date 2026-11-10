# Cargo and Dependencies

`Cargo` is Rust's build tool **and** package manager. As you've seen before, every package is described by a manifest, `Cargo.toml`:

```toml
[package]
name    = "dice-roller"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10"
```

The `[dependencies]` section declares every **crate** the code uses from the public registry [crates.io](https://crates.io) or (with a little more config) other arbitrary git repositories.

When you run `cargo build`, Cargo:

1. reads the manifest,
2. resolves each dependency to a concrete version, creates a hash and writes this information plus some other metadata into `Cargo.lock`,
3. downloads missing crates into `~/.cargo/registry` or an equivalent (OS-dependent) directory,
4. compiles the dependencies, then your code,
5. links them into a single binary.

A version like `"0.10"` is **semver-compatible shorthand** for `^0.10` - "any 0.10.x, but not 0.11".

If the major version is `0`, even the minor version counts as breaking, so `0.10` will not pull in `0.11`.

If source code refers to a crate that no dependency declares, the build fails with:

```text
error[E0432]: unresolved import `rand`
 --> main.rs:6:5
  |
6 | use rand::random_range;
  |     ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: if you wanted to use a crate named `rand`, use `cargo add rand` to add it to your `Cargo.toml`
```

To fix the error add the dependency line via `cargo add` or manually editing, then build again.
