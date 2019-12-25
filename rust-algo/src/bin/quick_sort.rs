extern crate rand;

use rand::Rng;

fn partition(a : &mut Vec<i32>, start : usize, end : usize) {
    let p = a[start];
    let mut i = start + 1;
    let mut j = i;
    for k in start + 1 .. end {
        if a[k] < p {
            let temp = a[k];
            a[k] = a[j];
            a[j] = temp;
            i = i + 1;
        } else {
            j = k;
        }
        println!("{:?} {} {} {}", a, i, j, k);
    }
    let temp = a[i];
    a[i] = p;
    a[0] = temp;
}

fn create_array(n : i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let a : Vec<i32> = (0 .. n)
        .map(|_| { rng.gen_range(0, 100) as i32 })
        .collect();
    return a;
}

fn main() {
    let mut a = create_array(10);
    println!("{:?}", a);
    let s = a.len();
    partition(&mut a, 0, s);
    println!("{:?}", a);
}
