---
id          = "conc_deadlock"
name        = "Fix the Deadlock"
language    = "rust"
difficulty  = 4
description = "Find the deadlock in a bank transfer and remove it with lock ordering."
topics      = ["deadlock", "mutex", "lock ordering", "arc", "threads"]
---

# Fix the Deadlock

A bank moves money between accounts. Every `Account` has a unique `id` and a `balance` behind a `Mutex`. The function `transfer(from, to, amount)` locks both accounts, then moves the money.

The program contains a bug: when two transfers run in **opposite directions** at the same time (Alice pays Bob while Bob pays Alice), it freezes. Run it to see that `main` never finishes.

## What goes wrong

`transfer` always locks `from` first and `to` second:

- Thread 1 runs `transfer(alice, bob)`: it locks Alice and then wants Bob.
- Thread 2 runs `transfer(bob, alice)`: it locks Bob and then wants Alice.

Each thread holds the lock the other one needs. That is a deadlock.

## Your Task

Rewrite the body of `transfer(from: &Account, to: &Account, amount: i64)` so that it always locks the account with the **lower `id` first**, whatever the direction of the transfer. Do not change `Account`, `simulate_work`, the signature, `main` or the tests.

1. Declare both guards without a value: `let mut from_balance;` and `let mut to_balance;`.
2. Write an `if from.id < to.id` with an `else`.
3. In the `if` branch, `from` has the lower id. Lock it first: `from_balance = from.lock();`, then call `simulate_work();`, then `to_balance = to.lock();`.
4. In the `else` branch, `to` has the lower id. Lock it first: `to_balance = to.lock();`, then call `simulate_work();`, then `from_balance = from.lock();`.
5. After the `if`, move the money as before: `*from_balance -= amount;` and `*to_balance += amount;`.

Keep the call to `simulate_work()` **between** the two locks in both branches. It stands for the work a real bank does there, and it makes the bug show up reliably. The two accounts of a transfer are always different.

```
alice (id 1): 100, bob (id 2): 100

transfer(&alice, &bob, 30)   → alice 70,  bob 130     locks alice, then bob
transfer(&bob, &alice, 50)   → alice 120, bob 80      locks alice, then bob
```

## Expected Result

- A transfer moves `amount` from `from` to `to`, in both directions.
- Transfers in opposite directions that run at the same time all finish. No deadlock.
- The total amount of money stays the same.
- `main` finishes and prints `alice: 120, bob: 80`.
