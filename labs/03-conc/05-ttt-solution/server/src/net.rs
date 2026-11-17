//! Everything that touches a socket. No game state lives here: these
//! threads only carry messages between the sockets and the buses.
//!
//! Seats go by arrival: the first connection is X, the second is O, and
//! then the listener is dropped, so a third connection is refused.
//!
//! Each seat gets two threads. The reader turns the lines it receives into
//! requests on the `inbox` bus. The writer takes updates off the `updates`
//! bus and sends the ones meant for its seat — that filter IS the
//! addressing: no address is ever written on the wire, because the socket
//! you are writing to already says who you are talking to.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use minibus::{Channel, Subscriber};
use ttt_core::{Player, Request, Update, wire};

use crate::controller::To;

/// Seat X, then O, then stop accepting (dropping the listener refuses any
/// further connection).
///
/// Each seat's update queue is created by `server.rs` before this thread
/// starts, so no answer can be missed. The handle of each writer thread is
/// sent back through `writers`, so the shutdown at the end of `run` can
/// wait for the last bytes to go out.
pub(crate) fn accept_loop(
    listener: TcpListener,
    inbox: Channel<(Player, Request)>,
    seat_x: Subscriber<(To, Update)>,
    seat_o: Subscriber<(To, Update)>,
    writers: mpsc::Sender<thread::JoinHandle<()>>,
) {
    for (seat, updates) in [(Player::X, seat_x), (Player::O, seat_o)] {
        // Keep trying until THIS seat is taken. Moving on after a failed
        // accept would leave seat X empty for good, and the game could
        // never start.
        let (stream, peer) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(e) => eprintln!("accept failed: {e}"),
            }
        };
        println!("seat {seat} connected from {peer}.");

        // The writer first, and its handle registered straight away, so the
        // shutdown never misses a writer that has just started.
        let write_half = stream.try_clone().expect("clone stream");
        let writer = thread::spawn(move || writer_loop(seat, write_half, updates));
        let _ = writers.send(writer);

        let bus = inbox.clone();
        thread::spawn(move || reader_loop(seat, stream, bus));
    }
}

/// Send this seat its share of the updates. Ends when the `updates`
/// channel is dropped and this queue has been drained — that is how the
/// server shuts down without guessing how long a flush takes.
fn writer_loop(seat: Player, mut stream: TcpStream, updates: Subscriber<(To, Update)>) {
    for (to, update) in updates {
        let for_me = match to {
            To::Everyone => true,
            To::Seat(player) => player == seat,
        };
        if !for_me {
            continue; // a private answer for the other player
        }

        let frame = wire::encode_update(&update);
        if stream.write_all(frame.as_bytes()).is_err() || stream.flush().is_err() {
            break; // this client is gone; its reader will say so
        }
    }
}

/// Put this seat's messages on the `inbox` bus, tagged with the seat.
///
/// A line we cannot read is ignored and reading continues — the protocol
/// has no error frame. The end of the stream, clean or not, becomes a
/// `Bye` sent on this seat's behalf.
fn reader_loop(seat: Player, stream: TcpStream, inbox: Channel<(Player, Request)>) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let Ok(line) = line else { break }; // the connection is gone
        if let Some(request) = wire::decode_request(&line) {
            inbox.publish((seat, request));
        }
    }
    println!("seat {seat} disconnected.");
    inbox.publish((seat, Request::Bye));
}
