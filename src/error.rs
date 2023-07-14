use std::sync::mpsc::RecvError;
use std::sync::mpsc::SendError;
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    TendermintSend(#[from] SendError<tendermint_proto::v0_34::abci::Response>),
    #[error(transparent)]
    ProstEncode(#[from] prost::EncodeError),
    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
    #[error(transparent)]
    RecV(#[from] RecvError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Request Error: {0}")]
    Request(String),
    #[error("Unknown Error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
