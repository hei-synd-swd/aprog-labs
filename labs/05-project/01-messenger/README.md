# <Project name>

> **Template.** This README is part of your deliverable: replace every `<...>` and every `TODO`, and delete this note. The course document (section "Evaluation") lists what it must contain.

<One or two sentences: what this project is.>

A multi-room chat over TCP: a server written in Rust, the clients that connect to it, and the protocol they speak.

**Authors:** <name> (<email>), <name> (<email>)

## Features

- TODO: what your server does (rooms, direct messages, refusals, ...)
- TODO: your bonus features, if any

## Usage

Requirements: the Rust toolchain and [`just`](https://github.com/casey/just).

```bash
just build               # build the server
just server              # run the server on port 5252 (`just server 6000` for another port)
just client              # open a client window; fill in the nick and press Connect
just tester              # check the server running on port 5252, rule by rule
just test                # the unit tests
just check               # build, tests and lint (zero warnings)
```

The given programs in `bin/` run on Linux (`bin/<name>`), macOS (`bin/<name>-macos`) and Windows (`bin/<name>.exe`). `just` picks the right one; on Windows its recipes run in PowerShell.

TODO: anything else a user must know to run your server (options, logs, ...).

## Protocol

The protocol between the clients and the server is described in [`doc/`](doc/). TODO: link the main file.

## Architecture

TODO: a short explanation of your design: the components and what each one is responsible for, the state and who owns it, the threads and how they communicate.

![Components](uml/<components>.svg)
![State](uml/<state>.svg)
![Threads](uml/<threads>.svg)
![One message](uml/<one-message>.svg)

## Tests

TODO: what your unit tests cover, and how to run them.

## License

TODO: the license of your code, for example MIT or Apache-2.0. Put its full text in a `LICENSE` file at the root of the workspace.

## Credits

Given by the course (HES-SO Valais-Wallis, Advanced Programming):

- `core/`: the messages (`Request`, `Update`) and the wire format.
- `bin/`: the client, the reference server and the tester.
- The minibus, from the tic-tac-toe lab (if you use it).

TODO: the libraries you added, and any source you used (documentation, articles, code, tools).
