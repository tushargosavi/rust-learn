
mod vector;

fn main() {
    let mut p = vector::Point2d::new(3.0, 4.0);
    p.add_inplace(vector::Point2d::new(2.0,3.0));
    p.add_inplace_tuple((4.0, 8.0));
    p.add_inplace_args(1.2, 3.4);
    println!("Hello, world! {:?} ", p);
}
