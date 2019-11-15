use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread::{JoinHandle, spawn};
use protobuf::{CodedInputStream, CodedOutputStream};
use crate::error::Result;
use crate::messages::abci::{Request, Response};

pub const MAX_MESSAGE_LENGTH: usize = 256 * 1024; // TODO: make configurable?

pub struct Connection {
    read_channel: mpsc::Receiver<Result<Request>>,
    write_channel: mpsc::SyncSender<Response>,
    read_thread: JoinHandle<()>,
    write_thread: JoinHandle<()>,
    socket: TcpStream
    // TODO: make generic for io::Read/Write
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Self> {
        Self::buffered(socket, 0)
    }

    pub fn buffered(socket: TcpStream, capacity: usize) -> Result<Self> {
        let read_socket = socket.try_clone()?;
        let (read_channel, read_thread) = Self::create_reader(read_socket, capacity);

        let write_socket = socket.try_clone()?;
        let (write_channel, write_thread) = Self::create_writer(write_socket, capacity);

        Ok(Connection {
            read_channel,
            write_channel,
            read_thread,
            write_thread,
            socket
        })
    }

    pub fn read(&self) -> Request {
        self.read_channel.recv().unwrap().unwrap() // TODO: return result
    }

    pub fn write(&self, res: Response) -> Result<()> {
        self.write_channel.send(res);
        // TODO: get last write error?
        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
        self.end()
    }

    fn create_reader(socket: TcpStream, capacity: usize) -> (mpsc::Receiver<Result<Request>>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        let thread = spawn(move || read(socket, sender));
        (receiver, thread)
    }

    fn create_writer(socket: TcpStream, capacity: usize) -> (mpsc::SyncSender<Response>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        let thread = spawn(move || write(socket, receiver));
        (sender, thread)
    }

    fn end(&mut self) -> Result<()> {
        unimplemented!()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.end().expect("Failed to close ABCI connection");
    }
}

fn read(mut socket: TcpStream, sender: mpsc::SyncSender<Result<Request>>) {
    let mut buf = [0 as u8; MAX_MESSAGE_LENGTH];
    let mut length = 0;

    // TODO: turn panics into errors that get passed to error channel

    loop {
        let req_length = read_varint(&mut socket).unwrap() as usize;
        if req_length > MAX_MESSAGE_LENGTH {
            let message = format!("Incoming ABCI request exceeds maximum length ({})", req_length).to_string();
            sender.send(Err(
                Error::new(ErrorKind::InvalidData, message).into()
            ));
            return;
        }

        socket.read_exact(&mut buf[..req_length]).unwrap();

        let req: Request = protobuf::parse_from_bytes(&buf[..req_length]).unwrap();
        sender.send(Ok(req));
    }
}

fn write(mut socket: TcpStream, receiver: mpsc::Receiver<Response>) {
    let mut stream = CodedOutputStream::new(&mut socket);

    let mut write_response = || -> Result<()> {
        let res: Response = receiver.recv()?;
        println!("writing response: {:?}", res);
        stream.write_message_no_tag(&res)?;
        Ok(())
    };
    
    loop {
        if let Err(err) = write_response() {
            panic!(err) // TODO: send in error channel
        }
    }
}

fn read_varint<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buf = [0 as u8; 1];
    let mut value: u64 = 0;

    for i in 0..=8 {
        if i == 8 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "VarInt exceeded maximum length".to_string()
            ).into());
        }

        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF".to_string()
            ).into());
        }

        let part = 0b0111_1111 & buf[0];
        value |= (part as u64) << (i * 7);

        let done = (0b1000_0000 & buf[0]) == 0;
        if done { break }
    }

    // ZigZag encoding, from integer-encoding crate
    // (https://github.com/dermesser/integer-encoding-rs/blob/e9b21fa87ef309f3f4242caa79ea010e20c2f224/src/varint.rs#L57-L63)
    Ok(((value >> 1) ^ (-((value & 1) as i64)) as u64) as i64)
}
