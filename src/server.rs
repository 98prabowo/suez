use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::{Arc, Mutex},
};

use crate::{
    controller::Controller, 
    error::Result, 
    model::AtomicDB, 
    pool::ThreadPool,
};

pub struct Server {
    addr: SocketAddr,
    db: AtomicDB,
}

impl Server {
    pub fn init(addr: IpAddr, port: u16) -> Self {
        Self {
            addr: SocketAddr::new(addr, port),
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        let pool = ThreadPool::build(10)?;

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let db = Arc::clone(&self.db);

                pool.execute(move || -> Result<()> {
                    Controller::handle_input(&mut stream, db);
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
