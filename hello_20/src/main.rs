//! Rust Book: Ch 20 - Final Project: Building a Multithreaded Web Server
//!
//! HTTP - HyperText Transfer Protocol, specific server requests
//! TCP - Transmission Control Protocol, routing of data from one server to another
//! ^, ^^ both are request-response protocols
//! TCP is a lower-level protocol which HTTP is (usually) sent atop of
//!

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //                                _________
    //                                   |      ____
    //                                   |       |
    //                                   |       port
    //                        IPv4 "loopback" or "localhost" address
    //                            (generic across systems)
    //                  Note: IPv6 uses `::1` as localhost

    for stream in listener.incoming() {
        //                 ^ returns an iterator over connection attempts received on this 
        //                   "listener"
        //                 WHERE "connection" represents the whole request&respond
        //                 process -- and a "connection" is "closed" when we respond
        //                 so basically we're iterating over un-responded to requests
        //
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
