use std::thread::{self, JoinHandle, Thread};
use std::time::Duration;

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