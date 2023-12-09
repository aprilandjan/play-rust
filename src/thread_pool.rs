use std::marker::{ Send };
use std::thread;
use std::sync::{mpsc, Arc, Mutex}; // message channel

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(||{
      receiver;
    });
    Worker { id, thread }
  }
}

struct Job {}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size); // fixed size allocation
    for id in 0..size {
      // the receiver will be shared in all workers to dispatch jobs
      // 'mutex' ensures only one worker can use receiver at the same time
      // 'arc' allows these workers to own this receiver
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  // mimic behavior of `thread`
  pub fn execute<F>(&self, f: F) where
  F: FnOnce() + Send + 'static
  {
    // TODO:
  }
}

