extern crate mio;

use mio::*;
use mio::net::{TcpStream};

const CLIENT: Token = Token(1);

fn main() {
    let addr = "127.0.0.1:8081".parse().unwrap();
    let sock = TcpStream::connect(&addr).unwrap();
    let poll = Poll::new().unwrap();
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();
    let mut events = Events::with_capacity(1024);
    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            match event.token() {
                CLIENT => {
                    println!("some data avilable to read");
                    return;
                }
                _ => { unreachable!("Can't reach hear, error in logic"); }
            }
        }
    }
}