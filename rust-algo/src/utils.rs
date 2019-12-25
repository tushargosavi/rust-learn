
extern crate rand;
use rand::Rng;

#[inline]
pub fn swap(a : &mut Vec<i32>, i : usize, j : usize) {
    let t = a[i]; a[i] = a[j]; a[j] = t;
}

pub fn gen_num(n:u32) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  (0..n).map(|_| rng.gen()).collect()
}

pub fn gen_num_range(n:u32, low : i32, high : i32) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  (0..n).map(|_| rng.gen_range(low, high)).collect()
}
