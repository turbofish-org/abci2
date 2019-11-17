use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread::spawn;
use protobuf::Message;
use crate::error::Result;
use crate::messages::abci::{Request, Response};
use crate::varint;

pub const MAX_MESSAGE_LENGTH: usize = 256 * 1024; // TODO: make configurable?

pub struct Connection {
    read_channel: mpsc::Receiver<Result<Request>>,
    write_channel: mpsc::SyncSender<Response>,
    socket: TcpStream
    // TODO: make generic for io::Read/Write
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Self> {
        Self::buffered(socket, 0)
    }

    pub fn buffered(socket: TcpStream, capacity: usize) -> Result<Self> {
        let read_socket = socket.try_clone()?;
        let read_channel = Self::create_reader(read_socket, capacity);

        let write_socket = socket.try_clone()?;
        let write_channel = Self::create_writer(write_socket, capacity);

        Ok(Connection {
            read_channel,
            write_channel,
            socket
        })
    }

    pub fn read(&self) -> Result<Request> {
        Ok(self.read_channel.recv()??)
        // TODO: close connection if there was an error
    }

    pub fn write(&self, res: Response) -> Result<()> {
        self.write_channel.send(res)?;
        // TODO: get last write error?
        // TODO: close connection if there was an error
        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
        self.end()
    }

    fn create_reader(socket: TcpStream, capacity: usize) -> mpsc::Receiver<Result<Request>> {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        spawn(move || read(socket, sender));
        receiver
    }

    fn create_writer(socket: TcpStream, capacity: usize) -> mpsc::SyncSender<Response> {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        spawn(move || write(socket, receiver));
        sender
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
            // swallow NotConnected errors since we want to disconnect anyway
            Err(crate::error::Error::IO(err))
                if err.kind() == ErrorKind::NotConnected => {},

            Err(err) => panic!(err),
            _ => {}
        };
    }
}

fn read(mut socket: TcpStream, sender: mpsc::SyncSender<Result<Request>>) {
    let mut buf = [0 as u8; MAX_MESSAGE_LENGTH];

    let mut read_request = || -> Result<Request> {
        let length = varint::read(&mut socket)? as usize;
        if length > MAX_MESSAGE_LENGTH {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Incoming ABCI request exceeds maximum length ({})", length).to_string()
            ).into());
        }

        socket.read_exact(&mut buf[..length])?;

        let req: Request = protobuf::parse_from_bytes(&buf[..length])?;
        Ok(req)
    };

    loop {
        sender.send(read_request()).unwrap(); // TODO: silently exit on error?
    }
}

fn write(mut socket: TcpStream, receiver: mpsc::Receiver<Response>) {
    let mut write_response = || -> Result<()> {
        let res: Response = receiver.recv().unwrap(); // TODO: silently exit on error?

        let mut buf = [0 as u8; 8];
        let length = res.compute_size() as i64;
        let varint_length = varint::encode(&mut buf, length);
        socket.write(&buf[..varint_length])?;

        res.write_to_writer(&mut socket)?;

        Ok(())
    };
    
    loop {
        if let Err(err) = write_response() {
            panic!(err) // TODO: send in error channel
        }
    }
}
