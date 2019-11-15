#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Other(Box<dyn std::error::Error + Send>)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
