//! Minimal two-peer TCP chat, built on MiniBus. (Reference solution.)
//!
//! One `Channel<String>` per direction of the socket:
//!
//!   stdin  --publish-->  outgoing  --subscribe-->  socket writer
//!   socket --publish-->  incoming  --subscribe-->  main (prints)
//!
//! Run it in two terminals:
//!   cargo run --example chat -- listen 7878
//!   cargo run --example chat -- connect 127.0.0.1:7878

use minibus::Channel;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, process, thread};

fn main() {
    let stream = connect_from_args();

    let incoming: Channel<String> = Channel::new("incoming"); // lines from the peer
    let outgoing: Channel<String> = Channel::new("outgoing"); // lines we type
    let in_sub = incoming.subscribe();
    let out_sub = outgoing.subscribe();

    // Reader thread: socket -> `incoming` bus.
    let reader_bus = incoming.clone();
    let read_socket = stream.try_clone().expect("clone socket");
    thread::spawn(move || {
        for line in BufReader::new(read_socket).lines() {
            let line = line.expect("socket read");
            reader_bus.publish(line);
        }
    });

    // Writer thread: `outgoing` bus -> socket.
    let mut write_socket = stream;
    thread::spawn(move || {
        for msg in out_sub {
            if write_socket.write_all(msg.as_bytes()).is_err()
                || write_socket.write_all(b"\n").is_err()
                || write_socket.flush().is_err()
            {
                break; // peer gone
            }
        }
    });

    // Stdin thread: what you type -> `outgoing` bus.
    let stdin_bus = outgoing.clone();
    thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            stdin_bus.publish(line.expect("stdin read"));
        }
    });

    // Main thread: print what the peer sent. Ctrl-C to quit.
    for line in in_sub {
        println!("peer> {line}");
    }
}

/// Parse the CLI and return a connected stream (the `listen` side accepts
/// exactly one peer).
fn connect_from_args() -> TcpStream {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "listen" {
        let addr = format!("0.0.0.0:{}", args[2]);
        let listener = TcpListener::bind(&addr).expect("bind");
        println!("listening on {addr}, waiting for a peer...");
        return listener.accept().expect("accept").0;
    }
    if args.len() == 3 && args[1] == "connect" {
        let stream = TcpStream::connect(&args[2]).expect("connect");
        println!("connected to {}", args[2]);
        return stream;
    }
    eprintln!("usage: chat listen <port> | chat connect <host:port>");
    process::exit(1);
}
