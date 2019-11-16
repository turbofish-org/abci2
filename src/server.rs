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


#[cfg(test)]
mod tests {
    use crate::messages::abci;
    use super::*;

    #[test]
    fn server() {
        fn handle_connection(i: usize, conn: Connection) {
            use abci::Request_oneof_value::*;

            std::thread::spawn(move || loop {
                let req = conn.read().unwrap();
                println!("got request on connection {}: {:?}", i, req);

                let mut res = abci::Response::new();

                match req.value {
                    Some(info(_)) => res.set_info(abci::ResponseInfo::new());
                    Some(init_chain(_)) => res.set_init_chain(abci::ResponseInitChain::new());
                    Some(begin_block(_)) => res.set_begin_block(abci::ResponseBeginBlock::new());
                    Some(end_block(_)) => res.set_end_block(abci::ResponseEndBlock::new());
                    Some(commit(_)) => res.set_commit(abci::ResponseCommit::new());
                    Some(flush(_)) => res.set_flush(abci::ResponseFlush::new());
                    _ => {}
                }

                conn.write(res).unwrap();
            });
        }

        let server = Server::listen("localhost:26658").unwrap();

        handle_connection(1, server.accept().unwrap());
        handle_connection(2, server.accept().unwrap());
        handle_connection(3, server.accept().unwrap());
        std::thread::park();
    }
}
