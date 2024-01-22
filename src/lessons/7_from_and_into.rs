// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html

#[derive(Debug)]
enum City {
  Beijing,
  Shanghai,
  Guangzhou,
  Shenzhen,
}

#[derive(Debug)]
struct Address {
  city: City,
  street: String,
}

#[derive(Debug)]
struct Location {
  details: String,
}

impl From<Address> for Location {
  fn from(address: Address) -> Self { // 'self' refers to the `Location` structure
    let city = match address.city {
      City::Beijing => "Beijing",
      City::Shanghai => "Shanghai",
      City::Guangzhou => "Guangzhou",
      City::Shenzhen => "Shenzhen"
    };
    Location {
      details: (city.to_owned() + " city, " + &address.street + " street"),
    }
  }
}

// impl Into<Location> for Address {
//   fn into(self) -> Location {
//     let city = match self.city {
//       City::Beijing => "Beijing",
//       City::Shanghai => "Shanghai",
//       City::Guangzhou => "Guangzhou",
//       City::Shenzhen => "Shenzhen"
//     };
//     Location {
//       details: (city.to_owned() + " city, " + &self.street + " street"),
//     }
//   }
// }

pub fn convert_address_to_location() {
  let address = Address {
    city: City::Shenzhen,
    street: "Nanhai avenue".to_owned(),
  };
  println!("address before convert: {:?}", address);
  // let location = Location::from(address);
  // println!("location from the address: {:?}", location);

  // the 'into' is automatically available if we had 'from' trait defined
  // but we need to explicitly declare typing
  let location: Location = address.into();
  println!("address into location: {:?}", location);
}
