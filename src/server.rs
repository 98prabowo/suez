use std::net::{IpAddr, SocketAddr, TcpListener};

use crate::{
    controller::Controller, 
    error::Result, 
    model::AtomicDb, 
    pool::ThreadPool,
};

pub struct Server {
    addr: SocketAddr,
    db: AtomicDb,
}

impl Server {
    pub fn init(addr: IpAddr, port: u16) -> Self {
        Self {
            addr: SocketAddr::new(addr, port),
            db: AtomicDb::new(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        let pool = ThreadPool::build(10)?;

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let db = self.db.clone();

                pool.execute(move || -> Result<()> {
                    Controller::handle_input(&mut stream, db);
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
