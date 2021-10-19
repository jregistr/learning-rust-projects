use std::{io::Write, net::{TcpListener, TcpStream}};
use std::io::prelude::*;
use std::fs;

fn main() {
    println!("Listening for connections at 127.0.0.1 at port: {}", 7878);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write(b"hello world").unwrap();
                println!("Connection established!");
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let hello_contents = fs::read_to_string("hello.html").unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = format! {
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=UTF-8\r\n\
        Content-Length: {}\r\n\r\n{}",
        hello_contents.len(),
        hello_contents
    };
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
