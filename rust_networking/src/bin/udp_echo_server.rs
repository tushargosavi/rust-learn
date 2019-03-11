use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("unable to bind");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("fail to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src).expect("fail to send a response");
                });
            },
            Err(e) => {
                eprintln!("could not recive a datagram packet {}", e);
            }
        }
    }
}

