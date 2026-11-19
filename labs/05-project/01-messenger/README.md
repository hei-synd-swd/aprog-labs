# messenger — write the server

A multi-room chat over TCP. You are given a complete, working system: the
**client** (a window), a **reference server** (a program, without its
source) and the **core** (the words the two sides trade, and how they are
written on the wire). The **server** is yours: you write all of it, and you
write down the protocol it speaks.

- **core** — given. `Request` and `Update`, `is_valid_name`, and `wire`:
  the text format, with its tests. Read it; do not change it.
- **bin/** — given, as programs (no source):
  - **client**: `just client` opens a window. Its **Wire** button shows
    every line it sends and receives.
  - **reference-server**: the specification. Your server must behave the
    same, word for word.
  - **tester**: checks a running server, rule by rule.
- **server/** — yours. A Cargo crate with an empty `main.rs`; how it is
  organised is up to you. To use the minibus of the tic-tac-toe lab (its
  solution), uncomment its line in `server/Cargo.toml`.
- **doc/** — yours: the protocol you discover, in the form you like.
- **uml/** — yours: the diagrams of your design.

## What to do

1. **Observe the reference.** `just reference` in one terminal, then
   `just client` in two or three others. Chat, join rooms, send DMs, make
   mistakes, disconnect. Watch the server's trace and each window's Wire
   panel.
2. **Document the protocol**: the messages, the rules, and how they are
   written on the wire.
3. **Draw your design** in `uml/`: the kind of diagram and the tool are up
   to you.
4. **Build the server** component by component, testing each one before
   the next.
5. **Test the whole** with the client and with `just tester`.

The course document has the details.

## Commands

```
just reference           # the reference server, on port 5252
just server              # your server, on port 5252
just client              # a window; fill in the nick and Connect
just tester              # check the server running on port 5252
just test                # your unit tests
just lint                # rustfmt + clippy, zero warnings
just check               # build, tests, lint
```

On Windows, `bin/client.exe`, `bin/reference-server.exe` and
`bin/tester.exe` run directly from PowerShell; `just` runs the Linux
programs through WSL.
