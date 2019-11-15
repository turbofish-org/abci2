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
    use crate::messages::abci;
    use super::*;

    #[test]
    fn server() {
        fn handle_connection(i: usize, conn: Connection) {
            std::thread::spawn(move || loop {
                let req = conn.read().unwrap();
                println!("got request on connection {}: {:?}", i, req);

                match req.value {
                    Some(abci::Request_oneof_value::info(_)) => {
                        let mut res = abci::Response::new();
                        let mut info = abci::ResponseInfo::new();
                        info.set_last_block_app_hash(vec![0; 20]);
                        res.set_info(info);
                        conn.write(res).unwrap();
                    },

                    Some(abci::Request_oneof_value::flush(_)) => {
                        let mut res = abci::Response::new();
                        res.set_flush(abci::ResponseFlush::new());
                        conn.write(res).unwrap();
                    },

                    _ => {}
                }
            });
        }

        let server = Server::listen("localhost:26658").unwrap();

        handle_connection(1, server.accept().unwrap());
        handle_connection(2, server.accept().unwrap());
        handle_connection(3, server.accept().unwrap());
        std::thread::park();
    }
}
