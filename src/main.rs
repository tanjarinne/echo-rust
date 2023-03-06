use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_request(stream: &mut TcpStream) {
    let mut request = String::new();
    let mut buf: [u8; 1024] = [0 as u8; 1024];

    match stream.read(&mut buf) {
        Ok(size) => request = String::from_utf8_lossy(&buf[0..size]).to_string(),
        Err(e) => print!("Error: {}", e),
    }

    let response = format!("HTTP/1.1 200 OK\r\n\
        Server: Echo\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\r\n\
        {}", request.len(), request);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("algo"),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let host = std::env::var("ECHO_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("ECHO_PORT").unwrap_or("8989".to_string());
    let listener = match TcpListener::bind(format!("{host}:{port}")) {
        Ok(listener) => listener,
        Err(e) => panic!("Error initialising the server: {}", e),
    };
    println!("Echo server listening on {host}:{port}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_request(&mut stream),
            Err(e) => println!("Error: {}", e),
        }
    }
 }
