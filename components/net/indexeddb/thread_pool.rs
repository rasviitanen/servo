use crossbeam_channel::{self, Receiver, Sender};
use std::thread;

#[derive(Clone)]
pub struct TransactionPool {
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl TransactionPool {
    pub fn new(threads: u32) -> Result<Self, ()> {
        let (tx, rx) = crossbeam_channel::unbounded::<Box<dyn FnOnce() + Send + 'static>>();
        for _ in 0..threads {
            let rx = TaskReceiver(rx.clone());
            thread::Builder::new()
                .spawn(move || run_tasks(rx))
                .expect("Could not spawn thread");
        }
        Ok(TransactionPool { tx })
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(job))
            .expect("The thread pool has no thread.");
    }
}

#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl Drop for TaskReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let rx = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(rx)) {
                error!("Failed to spawn a thread: {}", e);
            }
        }
    }
}

fn run_tasks(rx: TaskReceiver) {
    while let Ok(task) = rx.0.recv() {
        task();
    }
}
