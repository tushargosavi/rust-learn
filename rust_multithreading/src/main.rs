use std::vec::Vec;
use std::{thread, time};
use std::rc::{Rc};

fn take(v : &Vec<i32>) {
  println!("Content of the vector is {:?}", v);
}

fn main() {
    let v = Rc::new(vec![1,2,3]);
    thread::spawn(move || {
        println!("print from thread");
    });
    println!("print from main");
    thread::sleep(time::Duration::from_millis(10));
}
