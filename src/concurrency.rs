use std::thread::{self, JoinHandle, Thread};
use std::time::Duration;
use std::sync::{ Mutex, Arc };
use std::rc::Rc;

pub fn thread_do_not_guaranty_exec_order() {
  thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
      println!("hi number {} from the main thread!", i);
      thread::sleep(Duration::from_millis(1));
  }

  println!("leave function call stack now...");
}

pub fn wait_for_thread_done() {
  // let mut handles: Vec<JoinHandle<Thread>> = vec![];
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
    }
  });
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  handle.join().unwrap();

  println!("leave function call stack now...");
}

pub fn lock_threads_using_mutex_or_not() {
  println!("");
  println!("start lock_threads_using_mutex_or_not...");
  // let locked_num = Rc::new(Mutex::new(10));
  // Arc = Atomic rc
  let locked_num = Arc::new(Mutex::new(10));
  let mut handlers = vec![];
  for _ in 0..10 {
    let cloned_locked_num = Arc::clone(&locked_num);
    let handler = thread::spawn(move || {
      let mut num = cloned_locked_num.lock().unwrap();
      // deference
      *num += 1;
    });
    handlers.push(handler);
  }

  for handle in handlers {
    handle.join().unwrap();
  }

  // TODO: can we build a non-thread-safe add concurrently?...

  println!("the final num: {}", locked_num.lock().unwrap());
}