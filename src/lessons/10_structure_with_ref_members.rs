#[derive(Debug)]
struct Cat {
  name: String,
}

#[derive(Debug)]
struct PersonOwnTheCat {
  name: String,
  cat: Cat,
}

// TODO:
// #[derive(Debug)]
// struct PersonBorrowTheCat {
//   name: String,
//   cat: &Cat,
// }

pub fn a_person_with_a_cat() {
  let cat = Cat {
    name: String::from("kitty"),
  };

  let kim = PersonOwnTheCat {
    name: String::from("kim"),
    cat,
  };
  // the ownership of the cat is transferred to 'kim', so we cannot use it any more
  // println!("cat: {:?}", cat);
  println!("person own the cat: {:?}", kim);
}