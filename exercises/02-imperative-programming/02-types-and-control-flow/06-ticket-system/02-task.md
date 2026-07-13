---
id          = "impp_ticket_system"
name        = "Ticket System"
language    = "rust"
difficulty  = 3
description = "Move ownership between functions with a ticket struct."
topics      = ["ownership", "move", "consume", "struct"]
---

# Ticket System

Explore how ownership moves when passing values to functions.

1. Define a struct `Ticket` with a `description: String` field.
2. Write `buy(description: &str) -> Ticket` — creates an owned `Ticket`
   from a string slice.
3. Write `check(ticket: &Ticket) -> &str` — returns a reference to the
   ticket's description (borrows, does not take ownership).
4. Write `redeem(ticket: Ticket)` — takes **ownership** of the ticket.
   It should print `"Redeemed: {description}"`.
5. Write `upgrade(ticket: &mut Ticket, new_desc: &str)` — changes the
   description of an existing ticket.

The key insight: after `redeem(ticket)`, the ticket is **moved** and
can no longer be used.

## Expected Result

```
let mut t = buy("Concert");
check(&t)           → "Concert"
upgrade(&mut t, "VIP Concert");
check(&t)           → "VIP Concert"
redeem(t);          // t is MOVED here — can't use t after this
// check(&t)        // would NOT compile!
```
