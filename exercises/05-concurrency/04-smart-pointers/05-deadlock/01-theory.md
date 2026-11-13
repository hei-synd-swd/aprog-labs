# Deadlock and Lock Ordering

A **deadlock** happens when threads wait for each other's locks **forever**. Rust prevents data races at compile time, but not deadlocks: they are a design problem.

## How it happens

Two threads need the same two locks and take them in **opposite** order:

```rust
// thread A                         // thread B
let p = printer.lock();             let s = scanner.lock();
let s = scanner.lock(); // waits    let p = printer.lock(); // waits
```

Thread A holds the printer and waits for the scanner. Thread B holds the scanner and waits for the printer. Neither can continue, and neither ever releases its lock.

## The fix: lock ordering

Give all locks a fixed order and make **every** thread acquire them in that order. If both threads lock the printer first, one of them gets both locks, finishes and releases them, then the other one runs.

- Any order works, as long as it is the same everywhere. A unique **id** per resource is a common choice: lower id first.
- The order must depend on the resource itself, not on its role in an operation (source or destination, first or second argument).

## Declaring now, assigning later

A variable can be declared without a value and assigned in each branch of an `if`. This allows two variables to be assigned in a different order per branch:

```rust
let label;
if count == 1 {
    label = "item";
} else {
    label = "items";
}
```
