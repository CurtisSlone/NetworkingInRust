use std::net::UdpSocket;
use std::{str,io};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind socket");
    socket.connect("127.0.0.1:8888").expect("Could not connect socket");
    loop {
        let mut input = String::new();
        let mut buf = [0u8; 1500];
    
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        socket.send(&input.as_bytes()).expect("Failed to send data");

        socket.recv_from(&mut buf).expect("Failed to read into buffer");
        println!("Received: {}", str::from_utf8(&buf)
            .expect("Failed to read from stdin"));
    }
}