# Ownership and Borrow Errors

The borrow checker enforces Rust's two memory rules at compile time:

1. Every value has exactly **one owner**. Moving a value transfers ownership and the old binding becomes unusable.
2. At any time you may have *either* **one mutable reference** *or* **multiple shared references**. References must never outlive the data they point to.

The two most common error codes are:

| Code     | Meaning                                                            |
|----------|--------------------------------------------------------------------|
| `E0382`  | use of moved value: the owner changed                              |
| `E0499`  | more than one mutable borrow of the same value at the same time    |

The error messages show the exact "flow" of the value: where it moved, where it is still used.

