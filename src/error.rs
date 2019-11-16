// TODO: use error-chain

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Mpsc(MpscError),
    Protobuf(protobuf::error::ProtobufError),
    Other(Box<dyn std::error::Error + Send>)
}

#[derive(Debug)]
pub enum MpscError {
    Recv(std::sync::mpsc::RecvError),
    Send
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Error::Mpsc(MpscError::Recv(err))
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(_err: std::sync::mpsc::SendError<T>) -> Self {
        Error::Mpsc(MpscError::Send)
    }
}

impl From<protobuf::error::ProtobufError> for Error {
    fn from(err: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
