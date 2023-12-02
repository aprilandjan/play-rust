use std::thread;
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