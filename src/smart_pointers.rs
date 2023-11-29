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

//==================

// this is just a interface for someone to implement
pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T:Messenger {
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker { messenger, value: 0, max }
  }
  pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: all of your quota are consumed!");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("Warning: over 90% of your quota has been consumed!");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("Info: over 75% of your quota has been consumed...")
    }
  }
}

#[cfg(test)]
mod tests {
    use super::Messenger;

  struct MockMessenger {
    sent_messages: Vec<String>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger { sent_messages: vec![] }
    }
  }

  // this is the actual struct which implement the 'Messenger' trait
  impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
      // now we wll need to mutate self, so `send` in `Messenger` trait won't fit
      // self.sent_messages.push(msg.to_owned())
    }
  }
}