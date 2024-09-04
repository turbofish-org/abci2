<h1 align="left">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./abci2-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="./abci2.svg">
  <img alt="ed" src="./abci2.svg">
</picture>
</h1>

*Low-level ABCI protocol server*

[![Crate](https://img.shields.io/crates/v/abci2.svg)](https://crates.io/crates/abci2)
[![API](https://docs.rs/abci2/badge.svg)](https://docs.rs/abci2)

This crate provides low-level access to the ABCI protocol, via a `Connection` type which exposes `read()` and `write()` methods which return or accept ABCI request or response structs.

Currently supports **CometBFT 0.34**.

## Usage

**Add this crate as a dependency:**
```
[dependencies]
abci2 = "0.1"
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

For a more complete example, see [examples/simple.rs](https://github.com/turbofish-org/abci2/blob/master/examples/simple.rs) (you can run it via `cargo run --example simple`).

abci2 is currently used by [Nomic](https://github.com/nomic-io/nomic), a blockchain powering decentralized custody of Bitcoin, built on [Orga](https://github.com/turbofish-org/orga).

### Rebuild Protobuf

If you are updating this crate to protobuf definitions for a newer version of Tendermint, you can regenerate the code by running: `cargo run --bin codegen --features codegen`.

## Contributing

abci2 is an open-source project spearheaded by Turbofish. Anyone is able to contribute to abci2 via GitHub.

[Contribute to abci2](https://github.com/turbofish-org/abci2/contribute)

## Security

abci2 is currently undergoing security audits.

Vulnerabilities should not be reported through public channels, including GitHub Issues. You can report a vulnerability via GitHub's Private Vulnerability Reporting or to Turbofish at `security@turbofish.org`.

[Report a Vulnerability](https://github.com/turbofish-org/abci2/security/advisories/new)

## License

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

---

Copyright © 2024 Turbofish, Inc.