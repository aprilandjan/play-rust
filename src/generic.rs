use std::{ops::*, fmt::Debug};

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
        // // Add the below if you don't want floats.
        // + Eq
        // + Ord
{
}

pub fn call_generic_function_for_num() {
  let a: u16 = 16;
  let b: u64 = 64;
  let c: i32 = -2820;
  takes_a_num(0.32);
  takes_a_num(a);
  takes_a_num(b);
  takes_a_num(c);
}

fn takes_a_num<T: NumericValue + Debug>(num: T) {
  println!("taked num: {:?}", num);
}
