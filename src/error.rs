use std::error;
use std::fmt;

/// Midi encoding and decoding errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The MIDI channel is not between 1 and 16 inclusive.
    ChannelOutOfRange,

    /// No MIDI bytes were provided.
    NoBytes,

    /// A SysEx start byte was provided, but there was no corresponding SysEx end byte.
    NoSysExEndByte,

    /// Not enough data bytes for the specified MIDI message.
    NotEnoughBytes,

    /// Found a SysEx end byte, but there was no start byte.
    UnexpectedEndSysExByte,

    /// Found a status byte interleaved with SysEx data. SysEx messages should be a start byte, followed by data bytes,
    /// and ending in a end byte.
    UnexpectedNonSysExEndByte(u8),

    /// Found a status byte, but expected a `U7` data byte.
    UnexpectedStatusByte,

    /// Midi notes must be in the range [0, 127] inclusive.
    NoteOutOfRange,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
