use crate::error::{Error, Result};
use crate::varint;
use log::trace;
use prost::Message;
use std::io::{Read, Write};
use std::net::TcpStream;
use tendermint_proto::abci::request::Value;
use tendermint_proto::abci::*;

pub const MAX_MESSAGE_LENGTH: usize = 512 * 1024; // TODO: make configurable?

pub struct Connection {
    socket: TcpStream, // TODO: make generic for io::Read/Write
    saw_info: bool,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Self> {
        Ok(Connection { socket, saw_info: false })
    }

    pub fn read(&mut self) -> Result<Request> {
        let mut buf = [0; MAX_MESSAGE_LENGTH];

        let length = varint::read(&mut self.socket)? as usize;
        if length > MAX_MESSAGE_LENGTH {
            return Err(Error::Request(format!(
                "Incoming ABCI request exceeds maximum length ({})",
                length
            )));
        }

        self.socket.read_exact(&mut buf[..length])?;

        let mut req = Request::decode(&buf[..length]);

        // swallow message decode errors specifically on query connection
        match req {
            Ok(Request {
                value: Some(Value::Info(_)),
            }) => self.saw_info = true,
            Err(_) if self.saw_info => {
                req = Ok(Request {
                    value: Some(Value::Query(Default::default())),
                });
            }
            _ => {}
        }

        let req = req?;
        trace!("<< {:?}", req);

        // TODO: close connection if there was an error

        Ok(req)
    }

    pub fn write(&mut self, res: Response) -> Result<()> {
        trace!(">> {:?}", res);

        let mut buf = [0; 8];
        let length = res.encoded_len() as i64;
        let varint_length = varint::encode(&mut buf, length);
        self.socket.write_all(&buf[..varint_length])?;

        let mut buf = vec![];
        res.encode(&mut buf)?;
        self.socket.write_all(&buf)?;

        // TODO: close connection if there was an error

        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
        self.end()
    }

    fn end(&mut self) -> Result<()> {
        self.socket.shutdown(std::net::Shutdown::Both)?;
        // read and write threads will end as the connection will now error when
        // trying to use the socket or channels, whichever happens first
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        match self.end() {
            Ok(_) => (),
            Err(Error::IO(err)) if err.kind() == std::io::ErrorKind::NotConnected => (),
            Err(e) => Err(e).unwrap(),
        }
    }
}
