use tglib::utils;

fn main() {
    let v = tglib::utils::gen_num_range(30, 0, 100);
    println!("Generated array {:?}", v);
}
