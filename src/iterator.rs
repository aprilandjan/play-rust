pub fn filter_numeric_value_in_vec() {
  let list: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

  // here, `into_iter` gets the reference of the numeric element
  // so we need to use `deref` operator to get the value
  let a: Vec<u32> = list.into_iter().filter(|v| *v != 0).collect();
  println!("a: {:?}", a);

  // TODO: difference between `into_iter` and `iter`?

  let m = vec![1, 2, 3, 4, 5];
  m.iter().for_each(|x| {
    println!("v: {:?}", x)
  });
  // the `iter()` self just has the reference of the original vec member
  // and besides, the closure passed to filter() takes a reference
  // and many iterators iterate over references
  // this leads to a possibly confusing situation
  // where the type of the closure is a double reference
  let n: Vec<&i32> = m.iter().filter(|v| {
    **v % 2 == 0
  }).collect();
  println!("n: {:?}", n);
  // we can use `destructuring argument` to make it simpler
  let n1: Vec<&i32> = m.iter().filter(|&&v| {
    v % 2 != 0
  }).collect();
  println!("n1: {:?}", n1);

  // use `iter()`, the ownership of the original vec is not taken away
  println!("m: {:?}", m);

  // the address of n/m is still the same
  println!("m[0] address: {:p}, n1[0] address: {:p}", &m[0], n1[0]);
}