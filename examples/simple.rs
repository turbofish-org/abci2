use abci2::messages::abci;
use abci2::{Server, Connection};

// you can run this example by doing `cargo run simple-example`

fn main() {
    // start listening
    let server = Server::listen("localhost:26658").unwrap();

    // accept the 3 connections Tendermint is going to make, and handle incoming
    // requests in a separate thread for each
    handle_connection(1, server.accept().unwrap());
    handle_connection(2, server.accept().unwrap());
    handle_connection(3, server.accept().unwrap());

    // this just keeps the main thread from ending and closing the process
    std::thread::park();
}

fn handle_connection(i: usize, conn: Connection) {
    use abci::Request_oneof_value::*;

    // create a thread which reads a request then writes the response in a loop,
    // forever
    std::thread::spawn(move || loop {
        // get next incoming request
        let req = conn.read().unwrap();
        println!("got request on connection {}: {:?}", i, req);

        // just send back some empty responses for the messages we'll get
        let mut res = abci::Response::new();
        match req.value {
            Some(info(_)) => res.set_info(abci::ResponseInfo::new()),
            Some(init_chain(_)) => res.set_init_chain(abci::ResponseInitChain::new()),
            Some(begin_block(_)) => res.set_begin_block(abci::ResponseBeginBlock::new()),
            Some(end_block(_)) => res.set_end_block(abci::ResponseEndBlock::new()),
            Some(commit(_)) => res.set_commit(abci::ResponseCommit::new()),
            Some(flush(_)) => res.set_flush(abci::ResponseFlush::new()),
            _ => panic!("Unhandled request type: {:?}", req)
        }
        println!("sending response on connection {}: {:?}", i, res);

        // send the response back to Tendermint
        conn.write(res).unwrap();
    });
}