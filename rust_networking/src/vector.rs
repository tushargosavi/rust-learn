use std::ops::{Add, Sub, Mul, Div, AddAssign};
use std::fmt::Debug;

pub trait Num : Add<Output=Self> + AddAssign + Sub<Output=Self> + Mul<Output=Self> + Copy + Debug {
}

impl Num for f32 {}
impl Num for i64 {}
impl Num for f64 {}

#[derive(Debug)]
pub struct Point2d<T : Num> {
  x : T,
  y : T
}

impl <T: Num> Point2d<T> {
  pub fn new(x : T, y : T) -> Point2d<T> {
    Point2d{x : x, y: y}
  }

  pub fn add(self, p : Point2d<T>) -> Point2d<T> {
    Point2d{x : self.x + p.x, y : self.y + p.y }
  }

  pub fn add_inplace(&mut self, p : Point2d<T>) {
    self.x = self.x + p.x;
    self.y = self.y + p.y;
  }

  pub fn add_inplace_args(&mut self, x :T, y :T) {
    self.x += x;
    self.y += y;
  }

  pub fn add_inplace_tuple(&mut self, (x, y) : (T, T)) {
    self.x += x;
    self.y += y;
  }
}

#[derive(Debug)]
pub struct Point {
    x : f32,
    y : f32,
    z : f32
}

impl Point {
  pub fn new(x : f32, y : f32, z : f32) -> Point {
    Point{x : x, y : x , z: z}
  }

  pub fn add(& self, p : Point) -> Point {
    Point{x : self.x + p.x, y : self.y + p.y, z : self.z + p.z}
  }

  pub fn add_inplace(&mut self, p : Point) {
    self.x += p.x;
    self.y += p.y;
    self.z += p.z;
  }
}
