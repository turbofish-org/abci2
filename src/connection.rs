use crate::error::{Error, Result};
use crate::varint;
use log::trace;
use prost::Message;
use std::io::{Read, Write};
use std::net::TcpStream;
use tendermint_proto::v0_34::abci::request::Value;
use tendermint_proto::v0_34::abci::*;

pub const MAX_MESSAGE_LENGTH: usize = 4 * 1024 * 1024; // TODO: make configurable?

pub struct Connection {
    socket: TcpStream, // TODO: make generic for io::Read/Write
    saw_info: bool,
    buf: Vec<u8>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Self> {
        Ok(Connection {
            socket,
            saw_info: false,
            buf: vec![],
        })
    }

    pub fn read(&mut self) -> Result<Request> {
        let length = varint::read(&mut self.socket)? as usize;
        if length > MAX_MESSAGE_LENGTH {
            return Err(Error::Request(format!(
                "Incoming ABCI request exceeds maximum length ({})",
                length
            )));
        }

        self.buf.resize(length, 0);
        self.socket.read_exact(&mut self.buf[..length])?;

        let mut req = Request::decode(&self.buf[..length]);

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
        let length = res.encoded_len();
        let varint_length = varint::encode(&mut buf, length as i64);
        self.socket.write_all(&buf[..varint_length])?;

        if length > self.buf.capacity() {
            self.buf.reserve(length - self.buf.capacity());
        }
        self.buf.clear();
        res.encode(&mut self.buf)?;
        self.socket.write_all(&self.buf[..length])?;

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
            Err(e) => panic!("Error closing connection: {:?}", e),
        }
    }
}
