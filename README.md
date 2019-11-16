# abci2

*Low-level ABCI protocol server*

[![Crate](https://img.shields.io/crates/v/abci2.svg)](https://crates.io/crates/abci2)
[![API](https://docs.rs/abci2/badge.svg)](https://docs.rs/abci2)

This crate provides low-level access to the ABCI protocol, via a `Connection` type which exposes `read()` and `write()` methods which return or accept ABCI request or response structs.

Currently supports **Tendermint 0.32**.

## Usage

**Add this crate as a dependency:**
```
[dependencies]
abci2 = "0.1.0"
```

**Example:**
```rust
// listen for ABCI connections from Tendermint
let server = abci2::Server::listen("localhost:26658").unwrap();

// wait for Tendermint to connect (note that we will need to accept the 3
// separate connections that Tendermint makes). this function blocks until
// a connection comes in.
let connection = server.accept().unwrap();

loop {
    // get an incoming request
    let req = connection.read().unwrap();

    // handle the request somehow
    let res = process_request();

    // send back the response
    connection.write(res).unwrap();
}
```

For a more complete example, see [examples/simple.rs](https://github.com/nomic-io/abci2/blob/master/examples/simple.rs) (you can run it via `cargo run --example simple`).

## Rebuild Protobuf

If you are updating this crate to protobuf definitions for a newer version of Tendermint, you can regenerate the code by running: `cargo run --bin codegen --features codegen`.
