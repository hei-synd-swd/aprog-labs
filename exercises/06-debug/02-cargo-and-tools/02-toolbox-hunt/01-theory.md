# The Cargo Toolbox

Cargo has many subcommands for different functionality. THese are the three that you'll use the most:

**`cargo test`** compiles your crate with its unit tests and runs them:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running unittests src/lib.rs (target/debug/deps/md5_rs-a7f44c57368a2f65)

running 3 tests
test tests::tests::bytes ... ok
test tests::tests::nullbyte ... FAILED
test tests::tests::txt ... ok

failures:

---- tests::tests::nullbyte stdout ----

thread 'tests::tests::nullbyte' (1121787) panicked at src/tests.rs:43:9:
assertion `left == right` failed
  left: "d41d8cd98f00b204e9800998ecf8427f"
 right: "d41d8cd98f00b204e9800998ecf8427e"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::tests::nullbyte

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

The panic message shows *expected vs actual* values and the location of the failing assertion. You can then trace the code back to where it produce an incorrect value.

**`cargo clippy`** is a linter. It checks for best practices and suspicious code.

```text
warning: usage of `Box<&[u8; 64]>`
  --> src/lib.rs:86:25
   |
86 | fn process_block(block: Box<&[u8; 64]>, state: &mut [u32; 4], constants: [[u32; 64]; 2]) {
   |                         ^^^^^^^^^^^^^^ help: try: `&[u8; 64]`
   |
   = note: `&[u8; 64]` is already a pointer, `Box<&[u8; 64]>` allocates a pointer on the heap
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#redundant_allocation
   = note: `#[warn(clippy::redundant_allocation)]` on by default

warning: local variable doesn't need to be boxed here
  --> src/lib.rs:86:18
   |
86 | fn process_block(block: Box<&[u8; 64]>, state: &mut [u32; 4], constants: [[u32; 64]; 2]) {
   |                  ^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#boxed_local
   = note: `#[warn(clippy::boxed_local)]` on by default

warning: `md5-rs` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```
Lints do not stop the build as they are syntactically correct, so a passing `cargo build` says nothing about clippy's result. It is therefore important, to run `cargo clippy` before each commit, ideally also in the repos CI.

**`cargo fmt`** reformats code to the expected Rust style. **`cargo fmt --check`** only *reports* differences (exit code 1) without changing files:

```text
Diff in /home/user/Documents/GitRepos/md5-rs/src/lib.rs:70:
 
     block.push(0x80_u8);
 
-    if block.len() > 56   {
+    if block.len() > 56 {
         block.resize(64, 0u8);
         res.push(block.try_into().unwrap());
         block = vec![0u8; 56];
```

