use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // buffer to read data from client
    stream.read(&mut buffer).expect(
        "
        Failed to read from client 😭
    ",
    ); // read data from stream - stores in buffer

    let request = String::from_utf8_lossy(&buffer[..]); // UTF-8 encoded string
    println!("Received request: {}", request);

    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response 😭");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream)); // anti-closure with no input params
            } // only executes if successful stream
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e) // macro to print error msg
                                                                   // stderr - standard error stream
            }
        }
    }
}
