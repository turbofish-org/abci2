use std::net::{TcpListener, ToSocketAddrs};
use crate::error::Result;
use crate::connection::Connection;

pub struct Server (TcpListener);

impl Server {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        println!("listening on {}", listener.local_addr()?);
        Ok(Server(listener))
    }

    pub fn accept(&self) -> Result<Connection> {
        let stream = self.0.incoming().next().unwrap()?;
        println!("accepted connection from {}", stream.peer_addr()?);
        Connection::new(stream)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server() {
        fn handle_connection(i: usize, conn: Connection) -> ! {
            loop {
                let req = conn.read();
                println!("got request on connection {}: {:?}", i, req);
            }
        }

        let server = Server::listen("localhost:26658").unwrap();

        handle_connection(1, server.accept().unwrap());
        handle_connection(2, server.accept().unwrap());
        handle_connection(3, server.accept().unwrap());
    }
}
