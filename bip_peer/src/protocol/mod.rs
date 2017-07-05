//! Generic `PeerProtocol` implementations.

use std::io::{self, Write};

use nom::IResult;

pub mod null;
pub mod wire;

/// Trait for implementing a bittorrent protocol message.
pub trait PeerProtocol {
    /// Type of message the protocol operates with.
    type ProtocolMessage;

    /// Parse a `ProtocolMessage` from the given bytes.
    fn parse_bytes<'a>(&mut self, bytes: &'a [u8]) -> IResult<&'a [u8], Self::ProtocolMessage>;

    /// Write a `ProtocolMessage` to the given writer.
    fn write_bytes<W>(&mut self, message: &Self::ProtocolMessage, writer: W) -> io::Result<()>
        where W: Write;

    /// Retrieve how many bytes the message will occupy on the wire.
    fn message_size(&mut self, message: &Self::ProtocolMessage) -> usize;
}