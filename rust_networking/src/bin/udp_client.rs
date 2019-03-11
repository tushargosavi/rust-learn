use std::net::UdpSocket;
use std::{str, io};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8000").expect("Unable to bind to 8888");
    socket.connect("127.0.0.1:8888").expect("Could not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input).expect("Unable to read from stdin");
        socket.send(input.as_bytes()).expect("failed to write to server");
        socket.recv_from(&mut buffer).expect("could not read in buffer");
        println!("{}", str::from_utf8(&buffer).expect("Not a utf8 string"));
    }
}

