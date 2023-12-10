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
    let thread = thread::spawn(move ||{
      // so, the worker run loop endlessly to receive possible jobs
      // will that takes up cpu?
      loop {
        // so many unwrap...
        // the lock might error if the thread owning the lock crashed
        // the recv might error if the channel closed
        // and the recv will block and pending, if there's no incoming jobs
        // so probably the 'loop' wont takes up much cpu
        let job = receiver.lock().unwrap().recv().unwrap();
        println!("worker {} got a job, executing it now...", id);
        job();
        println!("worker {} job done!", id);
      }
    });
    Worker { id, thread }
  }
}

// struct Job {}
/// alias to a trait that holds some 'job closure';
type Job = Box<dyn FnOnce() + Send + 'static>;

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

  // mimic behavior of `thread`, which accepts a 'job closure'
  pub fn execute<F>(&self, f: F) where
  F: FnOnce() + Send + 'static
  {
    // TODO:
    let job = Box::new(f);
    // send the job closure via message channel
    self.sender.send(job).unwrap(); // incase that error happened
  }
}

