//! Rust Book: Ch 20 - Final Project: Building a Multithreaded Web Server
//!
//! HTTP - HyperText Transfer Protocol, specific server requests
//! TCP - Transmission Control Protocol, routing of data from one server to another
//! ^, ^^ both are request-response protocols
//! TCP is a lower-level protocol which HTTP is (usually) sent atop of
//!
//! Run program then use a browser (for example) aimed at "127.0.0.1:7878"

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    //                  ^ wraps the stream data, while doing "infrequent" chunk reads
    //                    so that we can make small 'frequent' calls to it
    //                    without the innefficiency of the same number of system calls
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //   ^ we take all the elements up until the nominal end of the request as denoted
    //     by an empty line (consecutive endline chars in the raw stream)
    //     , but it doesn't seem like we're closing the connection at any point
    //     (perhaps that's done by dint of the function taking ownership of the
    //     connection and then destroying it's when it de-scopes (?) )
    //                       ^ the stream isn't precisely a memory element (I assume)
    //                         so that's not strictly needed as far as I can tell

    // ^ I assume the above is only good practice because we make an assumption that
    //   the request is finite and of a reasonable length
    //   (/ perhaps that's something that a more robust system would need to account for
    //    or perhaps it's something that the HTTP protocol itself guarantees (?) )

    println!("Request: {:#?}", http_request);
}
