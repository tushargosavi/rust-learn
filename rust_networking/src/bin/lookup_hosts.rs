//#![feature(lookup_host)]
use std::env;
use std::net;

fn main() {
    /*
    This code does not compile because lookup_host is nightly only

    let args : Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("please provide any host name");
    } else {
        let addresses = net::lookup_host(&args[1]).unwrap();
        for address in addresses {
            println!("{}", address.ip());
        }
    }
    */
}
