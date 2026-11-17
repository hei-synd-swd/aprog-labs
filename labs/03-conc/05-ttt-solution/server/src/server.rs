//! Wire everything up and run the game.
//!
//! The shape is the same in both programs: a few threads do the blocking
//! I/O and put what they get on a bus, and ONE loop — the dispatcher —
//! reads that bus and is the only thing allowed to change the game. That
//! is why there is no lock anywhere in this crate: only one thread ever
//! touches the state.
//!
//! ```text
//!   socket (X) --> reader --\                             /--> writer --> socket (X)
//!                            >-- inbox --> SessionController --
//!   socket (O) --> reader --/  (seat, request)            \--> writer --> socket (O)
//!                                                  (to, update) — each writer
//!                                                  keeps what is for its seat
//! ```

use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use minibus::Channel;
use ttt_core::{Player, Request, Update};

use crate::controller::{SessionController, To};
use crate::net;

/// Play one game on a listener that `main` has already bound. Returns when
/// the game ends or someone leaves.
pub(crate) fn run(listener: TcpListener) {
    // The two buses: requests coming in, updates going out.
    let inbox: Channel<(Player, Request)> = Channel::new("inbox");
    let updates: Channel<(To, Update)> = Channel::new("updates");

    // Subscribe everyone BEFORE anything can be published, so no message is
    // missed: the dispatcher below, plus one queue per seat for the writer
    // threads that `accept_loop` will start.
    let requests = inbox.subscribe();
    let seat_x = updates.subscribe();
    let seat_o = updates.subscribe();

    // Seating happens in its own thread, so a single player can already
    // talk to the server while the second seat is still empty.
    // `writer_rx` collects the writer threads, to wait for them at the end.
    let (writer_tx, writer_rx) = mpsc::channel();
    thread::spawn(move || net::accept_loop(listener, inbox, seat_x, seat_o, writer_tx));

    // THE DISPATCHER: hand each request to the referee — the only owner of
    // the game — and send out the answers it gives back.
    let mut controller = SessionController::new();
    for (seat, request) in requests {
        for reply in controller.handle(seat, request) {
            updates.publish(reply);
        }
        if controller.finished() {
            break;
        }
    }

    // Shutting down, in two steps:
    //  1. drop our handle on `updates`. It is the last one, so every writer
    //     queue ends as soon as it has sent what is already in it — the
    //     final board and the Bye included;
    //  2. wait for those writer threads. When they are done, the bytes are
    //     really on the wire, and this process can exit.
    drop(updates);
    while let Ok(writer) = writer_rx.recv() {
        let _ = writer.join();
    }
}
