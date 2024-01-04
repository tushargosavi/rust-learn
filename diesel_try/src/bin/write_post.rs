extern crate diesel_try;
extern crate diesel;

use self::diesel_try::*;
use std::io::{stdin, Read};

fn main() {
    let mut connection = establish_connectoin();

    println!("What would you like your title ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() -1)];
    println!("\nOk! Lets write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&mut connection, title, &body);
    println!("\nSave draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF : &'static str = "CTLR+D";

#[cfg(windows)]
const EOF : &'static str = "CTLR+Z";
