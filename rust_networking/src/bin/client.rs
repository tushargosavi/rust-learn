use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write, Error, stdin};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8081")
        .expect("Count not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("failed to read from input");
        stream.write(input.as_bytes()).expect("failed to write to server");
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).expect("Coud not read into the buffer");
        println!("{}", str::from_utf8(&buffer).expect("could not write buffer as string"));
    }
}
