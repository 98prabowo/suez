use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::error::Result;

use super::Error;

pub type Task = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<Result<()>>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
        let thread = thread::spawn(move || -> Result<()> {
            loop {
                let message = receiver
                    .lock()
                    .map_err(|_| Error::MutexPoisoned)? 
                    .recv();

                match message {
                    Ok(job) => {
                        println!("{} {:<2} - executing", "WORKER", id);
                        job()?;
                    }
                    Err(_) => {
                        println!("{} {:<2} - disconnected", "WORKER", id);
                        break;
                    }
                }
            }

            Ok(())
        });

        let thread = Some(thread);

        Worker { id, thread }
    }
}

#[cfg(test)]
mod worker_tests {
    use std::{sync::mpsc::channel, time::Duration};

    use super::*;

    #[test]
    fn worker_process_job_test() {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let _sut = Worker::new(0, receiver);

        let (rsend, rrecv) = channel();
        let job = move || -> Result<()> { 
            rsend.send(()).unwrap();
            Ok(())
        };
        let job = Box::new(job);

        sender.send(job).unwrap();

        assert_eq!(rrecv.recv_timeout(Duration::from_secs(1)), Ok(()));
    }
}
