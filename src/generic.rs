use std::{ops::*, fmt::Debug};

// define 'zero' for different numeric types...
// https://users.rust-lang.org/t/generic-function-compare-integer-and-float-with-0/58246/2
trait Zero {
  const ZERO: Self;
}

impl Zero for i32 {
  const ZERO: Self = 0;
}

impl Zero for i64 {
  const ZERO: Self = 0;
}

impl Zero for u16 {
  const ZERO: Self = 0;
}

impl Zero for u32 {
  const ZERO: Self = 0;
}

impl Zero for u64 {
  const ZERO: Self = 0;
}

impl Zero for f32 {
  const ZERO: Self = 0.0;
}

impl Zero for f64 {
  const ZERO: Self = 0.0;
}

fn is_zero<T: PartialEq + Zero>(value: &T) -> bool
{
  *value == T::ZERO
}

// https://www.reddit.com/r/rust/comments/qlyn12/how_to_write_a_generic_function_for_only_numeric/

// this is a custom trait that act like a numeric value
trait NumericValue {

} // Optionally add some type bounds here also

// Implement this trait for everything that also implements the following
impl<T> NumericValue for T where
    T: Add<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Rem<Output = Self>
        + Copy
        + PartialEq
        + PartialOrd
        + Zero
        // // Add the below if you don't want floats.
        // + Eq
        // + Ord
{
}

pub fn call_generic_function_for_num() {
  let a: u16 = 16;
  let b: u64 = 64;
  let c: i32 = -2820;
  let d: f64 = 0.0;
  takes_a_num(0.32);
  takes_a_num(a);
  takes_a_num(b);
  takes_a_num(c);
  takes_a_num(d);
  takes_a_num(0);
}

fn takes_a_num<T: NumericValue + Debug + PartialEq + Zero>(num: T) {
  println!("taked num: {:?}", num);
  if (is_zero(&num)) {
    println!("the num taked is zero...");
  }
}
