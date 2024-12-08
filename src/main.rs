mod controller;
mod error;
mod model;
mod pool;
mod server;

use std::net::{IpAddr, Ipv4Addr};

use error::Result;
use server::Server;

fn main() -> Result<()> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let addr = IpAddr::V4(ip);
    let port: u16 = 31337;
    Server::init(addr, port).run()
}
