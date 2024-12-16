use std::sync::{
    mpsc::{channel, Sender}, 
    Arc, 
    Mutex
};

use crate::error;

use super::{
    worker::{Task, Worker}, 
    Error 
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Task>>,
}

impl ThreadPool {
    pub fn build(size: usize) -> super::Result<Self> {
        if size <= 0 {
            return Err(Error::InvalidPoolSize)
        }

        let (sender, receiver) = channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        Ok(Self { workers, sender })
    }

    pub fn execute<F>(&self, task: F) -> super::Result<()>
    where 
        F: FnOnce() -> error::Result<()> + Send + 'static
    {
        let job = Box::new(task);

        self.sender
            .as_ref()
            .ok_or(Error::SenderIsNil)?
            .send(job)
            .map_err(|_| Error::ReceiverIsDropped)?;

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("{} {:<2} - shutting down", "WORKER", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .unwrap()
                    .expect(format!("{} {:<2} - thread join fail", "WORKER", worker.id).as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn threadpool_build_test() {
        let sut = ThreadPool::build(10);
        let workers = &sut.as_ref().unwrap().workers; 
        let sender = &sut.as_ref().unwrap().sender; 
        assert_eq!(workers.len(), 10);
        assert_eq!(sender.is_some(), true);
    }

    #[test]
    fn threadpool_build_invalid_size_test() {
        let sut = ThreadPool::build(0);
        assert_eq!(sut.err(), Some(Error::InvalidPoolSize));
    }
    
    #[test]
    fn execute_threadpool_test() {
        let sut = ThreadPool::build(10);

        let (tx, rx) = channel();
        let task = move || -> error::Result<()> {
            tx.send(()).unwrap();
            Ok(())
        };
        
        sut.unwrap().execute(task).unwrap();

        assert_eq!(rx.recv_timeout(Duration::from_secs(1)), Ok(()));
    }
}
