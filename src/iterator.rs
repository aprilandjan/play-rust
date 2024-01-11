pub fn filter_numeric_value_in_vec() {
  let list: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

  // here, `into_iter` gets the reference of the numeric element
  // so we need to use `deref` operator to get the value
  let a: Vec<u32> = list.into_iter().filter(|v| *v != 0).collect();
  println!("a: {:?}", a);

  // TODO: difference between `into_iter` and `iter`?
}