extern crate mio;

use mio::tcp::TcpListener;
use mio::*;

const SERVER : Token = Token(0);

fn main() {
    let addr = "0.0.0.0:8081".parse().unwrap();
    let server = TcpListener::bind(&addr).unwrap();
    let poll = Poll::new().unwrap();
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let (_stream, remote) = server.accept().unwrap();
                    println!("Connection from {}", remote);
                }
                _ => { unreachable!(); }
            }
        }

    }
}


