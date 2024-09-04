use crate::connection::Connection;
use crate::error::Result;
use std::net::{SocketAddr, TcpListener, ToSocketAddrs};

pub struct Server(TcpListener);

impl Server {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server(listener))
    }

    pub fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.0.accept()?;
        Connection::new(stream)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.0.local_addr()?)
    }
}
