//! TCP port listener and connection handler.

use crate::connection::Connection;
use crate::error::Result;
use std::net::{SocketAddr, TcpListener, ToSocketAddrs};

/// A TCP server that listens for incoming ABCI connections.
pub struct Server(TcpListener);

impl Server {
    /// Create a new server that listens on the given address.
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server(listener))
    }

    /// Accept a new connection, blocking until one is received.
    pub fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.0.accept()?;
        Connection::new(stream)
    }

    /// Get the local address that the server is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.0.local_addr()?)
    }
}
