use std::net::{TcpStream, SocketAddr};
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

fn main() {
    let remote : SocketAddr = "127.0.0.1:8081".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1))
        .expect("Unable to connect to the server ");

    stream.set_read_timeout(Some(Duration::from_secs(3)))
        .expect("Could not set read timeout ");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Unable to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");
        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("Count not read from stream");
        println!("{}", str::from_utf8(&buffer).expect("not utf8 data"));
    }
}