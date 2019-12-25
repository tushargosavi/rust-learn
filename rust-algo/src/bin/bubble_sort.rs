extern crate rand;

use tglib::utils;


fn bubble_sort(mut a : &mut Vec<i32>) {
    for i in 0 .. a.len() {
        for j in i .. a.len() {
            if a[i] < a[j] {
                tglib::utils::swap(&mut a, i, j);
            }
        }
    }
}

fn insertion_sort(a : &mut Vec<i32>) {
    for i in 1 .. a.len() {
        let mut j = i;
        let t = a[i];
        while j > 0 && a[j-1] > t {
            a[j] = a[j-1];
            j -= 1;
        }
        a[j] = t;
    }
}


fn main() {
    let mut a = tglib::utils::gen_num_range(10, 0, 100);
    println!("Original {:?}", a);
    bubble_sort(&mut a);
    println!("Sorted {:?}", a);

    let mut a = tglib::utils::gen_num_range(10, 0, 100);
    println!("Original {:?}", a);
    insertion_sort(&mut a);
    println!("Sorted {:?}", a);
}
