use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/*
 *  thread::spawn signature:
 *
 *    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
 *        where
 *            F: FnOnce() -> T,
 *            F: Send + 'static,
 *            T: Send + 'static,
 *
 */

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // We’re calling unwrap on send for the case that sending fails
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // Vec::drain method accepts a range parameter to specify which items to remove
        // from the vector and returns an iterator of those items. Passing the .. range 
        // syntax will remove every value from the vector

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            // We need to move the thread out of the Worker instance that owns thread so
            // that join can consume the thread

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
