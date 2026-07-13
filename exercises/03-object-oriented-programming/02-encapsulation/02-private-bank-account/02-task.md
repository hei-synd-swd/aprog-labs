---
id          = "encapsulation_bank_account"
name        = "Private Bank Account"
language    = "rust"
difficulty  = 3
description = "Add methods to an encapsulated Account struct inside a module."
topics      = ["encapsulation", "module", "private", "pub", "Result"]
---

# Private Bank Account

An `Account` struct is defined inside a `mod bank` block. Its fields are
**private** — outside code can only interact through the public methods.

The struct already has `new`, `deposit`, and `balance` methods.

1. **Implement `withdraw(&mut self, amount: u64) -> Result<(), String>`**
   — subtract `amount` from balance if sufficient funds exist, otherwise
   return an error message.

2. **Implement `is_active(&self) -> bool`** — returns `true` if the balance
   is greater than zero.

## Expected Result

- `withdraw` returns `Ok(())` and reduces the balance when funds suffice.
- `withdraw` returns `Err(...)` and leaves the balance unchanged when
  funds are insufficient.
- `is_active` reflects whether the account holds any money.
