use abci2::messages::abci;
use abci2::{Server, Connection};

fn main() {
    let server = Server::listen("localhost:26658").unwrap();

    handle_connection(1, server.accept().unwrap());
    handle_connection(2, server.accept().unwrap());
    handle_connection(3, server.accept().unwrap());
    std::thread::park();
}

fn handle_connection(i: usize, conn: Connection) {
    use abci::Request_oneof_value::*;

    std::thread::spawn(move || loop {
        let req = conn.read().unwrap();
        println!("got request on connection {}: {:?}", i, req);

        let mut res = abci::Response::new();

        match req.value {
            Some(info(_)) => res.set_info(abci::ResponseInfo::new()),
            Some(init_chain(_)) => res.set_init_chain(abci::ResponseInitChain::new()),
            Some(begin_block(_)) => res.set_begin_block(abci::ResponseBeginBlock::new()),
            Some(end_block(_)) => res.set_end_block(abci::ResponseEndBlock::new()),
            Some(commit(_)) => res.set_commit(abci::ResponseCommit::new()),
            Some(flush(_)) => res.set_flush(abci::ResponseFlush::new()),
            _ => {}
        }

        println!("sending response on connection {}: {:?}", i, res);
        conn.write(res).unwrap();
    });
}