// TODO: use error-chain

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    MpscRecvError(std::sync::mpsc::RecvError),
    Protobuf(protobuf::error::ProtobufError),
    Other(Box<dyn std::error::Error + Send>)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Error::MpscRecvError(err)
    }
}

impl From<protobuf::error::ProtobufError> for Error {
    fn from(err: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
