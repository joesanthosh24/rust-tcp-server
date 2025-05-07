use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Handle communication with any connected client
fn client_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read stream");
    
    let req = String::from_utf8_lossy(&buffer[..]);
}