use std::net::{TcpListener, ToSocketAddrs};
use crate::error::Result;
use crate::connection::Connection;

pub struct Server (TcpListener);

impl Server {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server(listener))
    }

    pub fn accept(&self) -> Result<Connection> {
        let stream = self.0.incoming().next().unwrap()?;
        Connection::new(stream)
    }

    pub fn accept_buffered(&self, capacity: usize) -> Result<Connection> {
        let stream = self.0.incoming().next().unwrap()?;
        Connection::buffered(stream, capacity)
    }
}
