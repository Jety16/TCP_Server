use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Fixed syntax error here

    stream.read(&mut buffer).expect("Failed to read from client!");

    let request = String::from_utf8_lossy(&buffer[..]); // Added this line to convert bytes to string
    println!("Received request: {}", request);

    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

// Entry point
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_client(stream)); // Fixed path separator error and added move keyword
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
                // stderr - standard error stream
            }
        }
    }
}

