use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Could not connect to the server");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        stream.write_all(input.as_bytes()).expect("Failed to write to the server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        println!("{}", str::from_utf8(&buffer).expect("Could not write buffer to string"));
    }
}