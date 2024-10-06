use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8888")
        .expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("FDailed tp clone socket");

        match socket.recv_from(&mut buf) {
            Ok((_,src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src)
                        .expect("Failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("Error receiving UDP data: {}", e);
            }
        }
    }
}