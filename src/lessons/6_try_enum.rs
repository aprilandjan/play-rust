use rand::Rng;

#[derive(Debug)]
struct A {
  x: u32,
}

// three possible enum syntaxes
// https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
#[derive(Debug)]
pub enum Data {
  A(A),
  B(u32, u32),
  C {
    width: u32,
    height: u32,
  },
  D {
    m: u32,
    n: u32,
  }
}

fn generate_random_structured_enum_value() -> Data {
  let r = rand::thread_rng().gen_range(0..10);
  if r < 3 {
    Data::A(A {
      x: 100,
    })
  } else if r < 6 {
    Data::B(100, 200)
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
  match r {
    Data::A(_) => {println!("Data A matched")},
    Data::B(w, h) => {println!("Data B matched, w={:?}, height={:?}", w, h)},
    Data::C{width, height} => {println!("Data C matched, width={:?}, height={:?}", width, height)},
    Data::D {..} => {println!("Data D matched, and dont care about it")},
  };
  return r;
}