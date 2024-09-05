//! Error handling.

use std::sync::mpsc::RecvError;
use std::sync::mpsc::SendError;
pub use thiserror::Error;

/// The error type, which is used throughout the library to represent the
/// various ways that operations can fail, including from foreign crates.
#[derive(Error, Debug)]
pub enum Error {
    /// An error sent from the Tendermint node.
    #[error(transparent)]
    TendermintSend(#[from] SendError<tendermint_proto::v0_34::abci::Response>),
    /// A protobuf encoding error.
    #[error(transparent)]
    ProstEncode(#[from] prost::EncodeError),
    /// A protobuf decoding error.
    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
    /// An error from [std::sync::mpsc];
    #[error(transparent)]
    RecV(#[from] RecvError),
    /// An error from [std::io].
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// An error encountered when processing an ABCI request.
    #[error("Request Error: {0}")]
    Request(String),
    /// An unknown error.
    #[error("Unknown Error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
