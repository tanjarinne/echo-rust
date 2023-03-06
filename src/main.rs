use std::net::TcpListener;
use std::net::TcpStream;

fn handle_request(stream: &mut TcpStream) {

}

fn main() {
    let host = std::env::var("ECHO_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("ECHO_PORT").unwrap_or("8989".to_string());
    let bind_addr = format!("{host}:{port}");
    let listener = TcpListener::bind(bind_addr).unwrap();

    listener.incoming().for_each(|stream| {
        match stream {
            Ok(mut stream) => {
                handle_request(&mut stream);
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
 }
