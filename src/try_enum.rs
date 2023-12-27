use rand::Rng;

#[derive(Debug)]
struct A {
  x: u32,
}

#[derive(Debug)]
struct B {
  name: String,
}

#[derive(Debug)]
pub enum Data {
  A(A),
  B(B),
  C {
    width: u32,
    height: u32,
  },
}

fn generate_random_structured_enum_value() -> Data {
  let r = rand::thread_rng().gen_range(0..10);
  if r < 3 {
    Data::A(A {
      x: 100,
    })
  } else if r < 6 {
    Data::B(B {
      name: "b".to_owned(),
    })
  } else {
    Data::C {
      width: 100,
      height: 200,
    }
  }
}

pub fn output_random_enum_structured_enum_value() -> Data {
  let r = generate_random_structured_enum_value();
  println!("{:?}", r);
  return r;
}