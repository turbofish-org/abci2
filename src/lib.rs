//! A low-level ABCI protocol server, for building applications that work with
//! Tendermint or CometBFT consensus.

#![feature(trait_alias)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod connection;
mod error;
mod server;
mod varint;

pub use connection::Connection;
pub use error::Error;
pub use server::Server;
