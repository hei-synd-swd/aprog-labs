//! The socket side: connecting, and the two threads that carry messages
//! between the socket and the buses. This is the only file here that
//! touches the network.
//!
//! No seat, no filter: the server only sends this socket what concerns us,
//! because the connection itself says who we are. A line we cannot read is
//! ignored; the end of the stream becomes `Event::Closed`.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

use minibus::Channel;
use ttt_core::{Request, wire};

use crate::controller::Event;

/// Connect to a server. `None` on failure — `main` owns the exit code.
pub(crate) fn connect(addr: &str) -> Option<TcpStream> {
    match TcpStream::connect(addr) {
        Ok(socket) => {
            println!("connected to {addr}.");
            Some(socket)
        }
        Err(e) => {
            eprintln!("connection failed: {e}");
            None
        }
    }
}

/// socket -> events: read the server's updates and put them on the bus, as
/// `Event::FromServer`. This thread is the ONLY source of two of the three
/// `Event` variants: `FromServer` while the socket is open, then exactly
/// one `Closed` when it ends — clean or not, that is this thread's last word.
pub(crate) fn spawn_net_reader(events: &Channel<Event>, read_half: TcpStream) {
    let bus = events.clone();
    thread::spawn(move || {
        let reader = BufReader::new(read_half);
        for line in reader.lines() {
            let Ok(line) = line else { break }; // the connection is gone
            if let Some(update) = wire::decode_update(&line) {
                bus.publish(Event::FromServer(update));
            }
        }
        bus.publish(Event::Closed);
    });
}

/// outgoing -> socket: take each request off the bus and send it.
///
/// Subscribes before the thread starts, so a request published right after
/// this call (the first `Hello`) cannot be missed. Returns the thread's
/// handle: `main` waits for it so the last message really goes out.
pub(crate) fn spawn_net_writer(
    outgoing: &Channel<Request>,
    mut write_half: TcpStream,
) -> thread::JoinHandle<()> {
    let requests = outgoing.subscribe();
    thread::spawn(move || {
        for request in requests {
            let frame = wire::encode_request(&request);
            if write_half.write_all(frame.as_bytes()).is_err() || write_half.flush().is_err() {
                break; // the server is gone; the reader will say so
            }
        }
    })
}
