use std::{thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}};


type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, recv) = mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(recv.clone()));
        }

        Self {
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(handle) = worker.0.take() {
                handle.join().unwrap();
            }
        }
    }
}


struct Worker(Option<thread::JoinHandle<()>>);

impl Worker {
    pub fn new(recv: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let lock = recv.lock().unwrap();
            let msg = lock.recv();
            drop(lock);

            match msg {
                Ok(job) => job(),
                Err(_) => break
            }
        });

        Self(Some(handle))
    }
}
