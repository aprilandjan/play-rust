#[derive(Debug)]
struct Cat {
  name: String,
}

impl Cat {
  // &self is sugar for 'self: &Self'
  fn meow(&self) {
    println!("the cat '{:?}' meowed...", self.name);
  }
}

#[derive(Debug)]
struct PersonOwnTheCat {
  name: String,
  cat: Cat,
}

impl PersonOwnTheCat {
  fn tick(&self) {
    println!("{:?} ticked the cat...", self.name);
    self.cat.meow();
  }
}

#[derive(Debug)]
struct PersonBorrowTheCat<'a> {
  name: String,
  cat: &'a Cat,
}

// structure methods with lifespan
// https://stackoverflow.com/questions/69641302/returning-references-from-struct-method-with-lifetimes
impl<'a> PersonBorrowTheCat<'a> {
  fn tick(&self) {
    println!("{:?} ticked the cat...", self.name);
    self.cat.meow();
  }
}

#[derive(Debug)]
struct ParentOfThePersonBorrowTheCat<'a> {
  child: &'a PersonBorrowTheCat<'a>,
}

pub fn a_person_with_a_cat() {
  let kitty = Cat {
    name: String::from("kitty"),
  };

  let kim = PersonOwnTheCat {
    name: String::from("kim"),
    cat: kitty,
  };
  // the ownership of the cat is transferred to 'kim', so we cannot use it any more
  // println!("cat: {:?}", cat);
  println!("person own the cat: {:?}", kim);

  kim.tick();

  let tommy = Cat {
    name: String::from("tommy"),
  };

  let tim = PersonBorrowTheCat {
    name: String::from("tim"),
    cat: &tommy,
  };

  println!("cat: {:?}", tommy);
  println!("person borrow the cat: {:?}", tim);

  let tommy = Cat {
    name: String::from("tommy2"),
  };

  // so the previous 'tommy' cat was not expired when variable name was taken
  println!("person borrow the cat: {:?}", tim);

  tim.tick();

  let father = ParentOfThePersonBorrowTheCat {
    child: &tim,
  };
}