use std::marker::Send;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, Thread}; // message channel

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    // use `Option` to move ownership manually
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // so, the worker run loop endlessly to receive possible jobs
            // will that takes up cpu?
            loop {
                // so many unwrap...
                // the lock might error if the thread owning the lock crashed
                // the recv might error if the channel closed
                // and the recv will block and pending, if there's no incoming jobs
                // so probably the 'loop' wont takes up much cpu
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("worker {} got a job, executing it now...", id);
                        job();
                        println!("worker {} job done!", id);
                    }
                    Err(_) => {
                        // if the channel closed, then 'recv' will through error
                        // just break the loop
                        println!("worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
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
            sender: Some(sender),
        }
    }

    // mimic behavior of `thread`, which accepts a 'job closure'
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO:
        let job = Box::new(f);
        // send the job closure via message channel
        self.sender
            .as_ref()
            .unwrap() // so why as_ref?...hard to understand
            .send(job)
            .unwrap(); // incase that error happened
    }
}

// make sure when the pool is dropped, jobs on each thread are finished
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // move sender out of thread pool
        // then close it, which makes no more messages sent
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                // the `join` takes ownership
                thread.join().unwrap();
            }
        }
    }
}
