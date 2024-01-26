use rand::Rng;

#[derive(Debug)]
struct T {
  x: u32,
}

#[derive(Debug, Clone)]
enum Z {
  ZA = 1,
  ZB = 2,
}

// three possible enum syntaxes
// https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
#[derive(Debug)]
pub enum Data {
  A(T),
  B(u32, u32),
  C {
    width: u32,
    height: u32,
  },
  D {
    m: u32,
    n: u32,
  },
  E(Z),
}

fn generate_random_structured_enum_value() -> Data {
  let r = rand::thread_rng().gen_range(0..10);
  if r < 2 {
    Data::A(T {
      x: 100,
    })
  } else if r < 4 {
    Data::B(100, 200)
  } else if r < 6 {
    Data::C {
      width: 100,
      height: 200,
    }
  } else if r < 8 {
    Data::D {
      m: 3,
      n: 2,
    }
  } else if r < 9 {
    Data::E(Z::ZA)
  } else {
    Data::E(Z::ZB)
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
    Data::E(Z::ZA) => {println!("Data E with ZA matched")},
    Data::E(Z::ZB) => {println!("Data E with ZB matched")},
  };
  return r;
}