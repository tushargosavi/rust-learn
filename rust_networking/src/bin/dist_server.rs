#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{BufRead, stdin, BufReader, Error, Write};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64
}

impl Point3D {
    fn distance(&self) -> f64 {
        let distq = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        distq.sqrt()
    }
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    let mut buf = Vec::new();
    let mut reader = BufReader::new(stream);
    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf).unwrap();
        if bytes_read == 0 {
            return Ok(());
        }
        let input: Point3D = serde_json::from_slice(&buf).unwrap();
        let val = input.distance();

        write!(reader.get_mut(), "{}", val)?;
        write!(reader.get_mut(), "\n")?;
    }
}

fn start_server() {
    let addr : SocketAddr = "0.0.0.0:8081".parse().unwrap();
    println!("working in server mode, addr {}", addr);
    let ss = TcpListener::bind(&addr).unwrap();
    for stream in ss.incoming() {
        match stream {
            Err(e) => {
                eprintln!("Error {}", e);
                return;
            },
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| {
                        eprintln!("{:?}", error);
                    });
                });
            }
        }
    }
}

fn start_client() {
    println!("working in client mode");
    let addr : SocketAddr = "127.0.0.1:8081".parse().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("provide 3 integers separated by comma");
    loop {

        // read input from cli, and convert to point
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        let parts : Vec<&str> = input
                .trim_matches('\n')
                .split(',')
                .collect();
        let point = Point3D {
            x : parts[0].parse().unwrap(),
            y : parts[1].parse().unwrap(),
            z : parts[2].parse().unwrap()
        };
        println!("read point {:?}", point);

        // serialize and write data to server
        stream.write_all(serde_json::to_string(&point).unwrap().as_bytes())
            .expect("Unable to write to server");
        stream.write(b"\n").expect("Unable to write to server");

        // read data from server
        let mut reader = BufReader::new(&stream);
        let mut buffer : Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("Unable to read from server");
        let result = str::from_utf8(&buffer).expect("Unabe to convert to str");
        if result == "" {
            println!("received empty response from the server");
        }
        println!("response from the server {}", result);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide --client or --server as argument");
        std::process::exit(1);
    }

    if args[1] == "--server" {
        start_server();
    } else if args[1] == "--client" {
        start_client();
    } else {
        eprintln!("Invalid argument {}", args[1]);
    }
}
