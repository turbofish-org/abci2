use abci2::{Connection, Server};
use tendermint_proto::abci::{request, response, Response};

// you can run this example by doing `cargo run --example simple`

fn main() {
    // start listening
    let server = Server::listen("localhost:26658").unwrap();
    println!(
        "listening for ABCI connections on {}",
        server.local_addr().unwrap()
    );

    // accept the 3 connections Tendermint is going to make, and handle incoming
    // requests in a separate thread for each
    handle_connection(1, server.accept().unwrap());
    handle_connection(2, server.accept().unwrap());
    handle_connection(3, server.accept().unwrap());
    handle_connection(4, server.accept().unwrap());

    // this just keeps the main thread from ending and closing the process
    std::thread::park();
}

fn handle_connection(i: usize, mut conn: Connection) {
    // create a thread which reads a request then writes the response in a loop,
    // forever
    std::thread::spawn(move || loop {
        // get next incoming request
        let req = conn.read().unwrap();
        println!("got request on connection {}: {:?}", i, req);
        // just send back some empty responses for the messages we'll get
        let value = match req.value {
            Some(request::Value::Info(_)) => {
                let inner = tendermint_proto::abci::ResponseInfo::default();
                response::Value::Info(inner)
            }
            Some(request::Value::InitChain(_)) => {
                let inner = tendermint_proto::abci::ResponseInitChain::default();
                response::Value::InitChain(inner)
            }
            Some(request::Value::BeginBlock(_)) => {
                let inner = tendermint_proto::abci::ResponseBeginBlock::default();
                response::Value::BeginBlock(inner)
            }
            Some(request::Value::EndBlock(_)) => {
                let inner = tendermint_proto::abci::ResponseEndBlock::default();
                response::Value::EndBlock(inner)
            }
            Some(request::Value::Commit(_)) => {
                let inner = tendermint_proto::abci::ResponseCommit::default();
                response::Value::Commit(inner)
            }
            Some(request::Value::Flush(_)) => {
                let inner = tendermint_proto::abci::ResponseFlush::default();
                response::Value::Flush(inner)
            }
            _ => panic!("Unhandled request type: {:?}", req),
        };

        let res = Response {
            value: value.into(),
        };

        println!("sending response on connection {}: {:?}", i, res);

        // send the response back to Tendermint
        conn.write(res).unwrap();
    });
}
