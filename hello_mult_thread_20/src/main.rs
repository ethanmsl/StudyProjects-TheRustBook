//! Rust Book: Ch 20 - Final Project: Building a Multithreaded Web Server
//!
//! HTTP - HyperText Transfer Protocol, specific server requests
//! TCP - Transmission Control Protocol, routing of data from one server to another
//! ^, ^^ both are request-response protocols
//! TCP is a lower-level protocol which HTTP is (usually) sent atop of
//!
//! Run program then use a browser (for example) aimed at "127.0.0.1:7878"

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_mult_thread_20::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //                                _________
    //                                   |      ____
    //                                   |       |
    //                                   |       port
    //                        IPv4 "loopback" or "localhost" address
    //                            (generic across systems)
    //                  Note: IPv6 uses `::1` as localhost

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        //                 ^ returns an iterator over connection attempts received on this
        //                   "listener"
        //                 WHERE "connection" represents the whole request&respond
        //                 process -- and a "connection" is "closed" when we respond
        //                 so basically we're iterating over un-responded to requests
        //
        let stream = stream.unwrap();

        pool.execute(|| {
            println!("Connection established!");
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        //                              ^ NOTE: `match` does not auto ref & deref
        "GET / HTTP/1.1" => {
            println!("'Get /' request received");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("'Get /sleep' request received");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => {
            println!("unrecognized request received");
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
