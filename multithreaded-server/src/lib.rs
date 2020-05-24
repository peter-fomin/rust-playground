use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;
type JobReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The new function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        Self{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: JobReceiver) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job, executing...", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}