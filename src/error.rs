use core::fmt;
#[cfg(feature = "std")]
use std::error;

/// Midi decoding errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FromBytesError {
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

    /// The first byte of a midi message must be a status byte.
    UnexpectedDataByte,

    /// Found a status byte, but expected a `U7` data byte.
    UnexpectedStatusByte,

    /// Midi notes must be in the range [0, 127] inclusive.
    NoteOutOfRange,

    /// Data (U7) bytes must be between [0, 127] inclusive.
    DataByteOutOfRange,

    /// Data (U14) bytes must be between [0x0000, 0x03FF] or [0, 16383] inclusive.
    U14OutOfRange,
}

#[cfg(feature = "std")]
impl error::Error for FromBytesError {}

impl fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// An error that can occurr converting a midi message to a bytes slice.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ToSliceError {
    /// The destination buffer cannot fit all the bytes.
    BufferTooSmall,
}

#[cfg(feature = "std")]
impl error::Error for ToSliceError {}

impl fmt::Display for ToSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToSliceError::BufferTooSmall => write!(f, "buffer size too small"),
        }
    }
}
