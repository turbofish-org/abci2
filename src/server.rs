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
        let (stream, _) = self.0.accept()?;
        Connection::new(stream)
    }

    pub fn accept_buffered(&self, capacity: usize) -> Result<Connection> {
        let (stream, _) = self.0.accept()?;
        Connection::buffered(stream, capacity)
    }
}
