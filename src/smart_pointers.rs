struct CustomSmartPointer {
  data: String,
}

// this `Drop` trait is somehow like destructor in cpp?...
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop pointer data: {}", self.data);
    }
}

pub fn box_value_in_heap() {
  let b = Box::new(5);
  println!("b = {b}");
}

pub fn drop_automatically_when_leave() {
  let a = CustomSmartPointer {
    data: String::from("hello"),
  };
  let b = CustomSmartPointer {
    data: String::from("world"),
  };
  println!("going to leave...");
}

pub fn drop_manually() {
  let a = CustomSmartPointer {
    data: String::from("hello"),
  };
  let b = CustomSmartPointer {
    data: String::from("world"),
  };
  // manually drop b, so a will be automatically dropped
  drop(b);
  println!("b dropped manually");
  println!("going to leave...");
}