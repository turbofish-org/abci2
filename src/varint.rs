//! Protobuf varint encoding and decoding.

use crate::error::Result;
use std::io::{Error, ErrorKind, Read};

/// Reads a varint from a [Read] implementation.
pub fn read<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buf = [0 as u8; 1];
    let mut value: u64 = 0;

    for i in 0..8 {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF".to_string()).into());
        }

        let part = 0b0111_1111 & buf[0];
        value |= (part as u64) << (i * 7);

        let done = (0b1000_0000 & buf[0]) == 0;
        if done {
            // ZigZag encoding, from integer-encoding crate
            // (https://github.com/dermesser/integer-encoding-rs/blob/e9b21fa87ef309f3f4242caa79ea010e20c2f224/src/varint.rs#L57-L63)
            return Ok(((value >> 1) ^ (-((value & 1) as i64)) as u64) as i64);
        }
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "VarInt exceeded maximum length".to_string(),
    )
    .into())
}

/// Encodes a varint into a buffer, returning the number of bytes written.
pub fn encode(buf: &mut [u8; 8], value: i64) -> usize {
    // ZigZag encoding
    let mut value = ((value << 1) ^ (value >> 63)) as u64;

    for i in 0..8 {
        buf[i] = 0b0111_1111 & (value as u8);

        let done = value <= 0b0111_1111;
        if done {
            return i + 1;
        }

        buf[i] |= 0b1000_0000;
        value >>= 7;
    }

    unreachable!("VarInt should not be longer than 8 bytes");
}
