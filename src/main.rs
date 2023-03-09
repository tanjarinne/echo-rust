use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};


fn parse_request_line(line: &str) -> (&str, &str, &str) {
    let mut parts = line.trim().split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let version = parts.next().unwrap();

    (method, path, version)
}

fn handle_request(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let (method, path, _version) = parse_request_line(&line);
    println!("{} {}", &method, &path);
    line.clear();

    let mut headers = vec![];
    while reader.read_line(&mut line).unwrap() > 0 {
        if line == "\r\n" {
            break;
        }
        headers.push(line.clone());
        line.clear();
    }

    let mut body = String::new();
    reader.read_to_string(&mut body).unwrap();
    let response_body = format!("{}\r\n{}", &headers.join("\n"), &body);
    let response = format!("HTTP/1.1 200 OK\r\n\
        Server: Echo\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\r\n\
        {response_body}", response_body.len());

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
        }
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
