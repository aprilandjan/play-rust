use std::cell::RefCell;
use std::rc::Rc;

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
    let _a = CustomSmartPointer {
        data: String::from("hello"),
    };
    let _b = CustomSmartPointer {
        data: String::from("world"),
    };
    println!("going to leave...");
}

pub fn drop_manually() {
    let _a = CustomSmartPointer {
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

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger
                .send("Error: all of your quota are consumed!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Warning: over 90% of your quota has been consumed!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Info: over 75% of your quota has been consumed...")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LimitTracker, Messenger};
    use std::cell::RefCell;

    struct MockMessenger {
        // RefCell will record how many active Ref<T> and RefMut<T>
        // and we can borrow_mut if we have only one RefMut<T> at the same time
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // this is the actual struct which implement the 'Messenger' trait
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // now we wll need to mutate self, so `send` in `Messenger` trait won't fit
            // self.sent_messages.push(msg.to_owned())
            self.sent_messages.borrow_mut().push(msg.to_owned());
        }
    }

    #[test]
    fn it_sends_on_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// use List::{Cons, Nil};

// enum List {
//   Cons(i32, Rc<List>),
//   Nil,
// }

// use List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//   let b = Cons(3, Rc::clone(&a));
//   let c = Cons(4, Rc::clone(&a));
// }

// single direction linked list
#[derive(Debug)]
enum List {
    // a linked list
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn multiple_references_over_one_value() {
    // the initial value
    let value = Rc::new(RefCell::new(1));
    // the first link to a nil
    let a_from = Rc::new(List::Nil);
    // this 'clone' makes the a_to and `value` have both ownership for 5
    let a_to = Rc::clone(&value);
    let a = Rc::new(List::Cons(a_to, a_from));

    let b = List::Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // 'dereference' the i32 from RefMut
    *value.borrow_mut() = 123;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
