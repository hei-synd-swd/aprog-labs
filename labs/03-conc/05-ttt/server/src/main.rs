//! server — the referee. One game per run.
//!
//! This file is the edge of the program: it reads the command line, binds
//! the port, and owns the exit codes. Then it hands the ready listener to
//! `server::run`, which never exits the process — it returns.
//!
//! The three other files: `server.rs` wires things up and runs the loop,
//! `controller.rs` decides what to answer, `net.rs` talks to the sockets.
//!
//! Usage: `server <port>`

mod controller;
mod net;
mod server;

use std::net::TcpListener;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: server <port>");
        process::exit(2);
    }
    let port: u16 = match args[1].parse() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("usage: server <port> (a number)");
            process::exit(2);
        }
    };

    let listener = match TcpListener::bind(format!("0.0.0.0:{port}")) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("cannot listen on port {port}: {e}");
            process::exit(1);
        }
    };
    println!("listening on 0.0.0.0:{port}");

    server::run(listener);
}
