use std::vec::Vec;
use std::{io::Write, string::String};

/// An error that can occur when dealing with SMF.
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// `Format::SingleTrack` was selected but there was not exactly 1 track.
    SingleTrackFormatContainsNoTracks,
    /// `Format::SingleTrack` was selected but there were more than 1 track.
    SingleTrackFormatContainsMoreThanOneTrack,
    /// `Division::TicksPerBeat(..)` was used but the value was not positive.
    DivisionTicksMustBePositive,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl std::error::Error for Error {}

/// A sequence of events.
pub type Track<'a> = Vec<TrackEvent<'a>>;

#[derive(Clone, Debug)]
/// http://www.ccarh.org/courses/253/handout/smf/
pub struct SmfWriter<'a> {
    /// The format of the SMF file.
    pub format: Format,
    /// Unit for delta timing.
    pub division: Division,
    /// The tracks. Note: if `format` is `Format::SingleTrack`, then this must contain only a track.
    pub tracks: Vec<Track<'a>>,
}

impl<'a> SmfWriter<'a> {
    /// Write the contents of self to `w` in SMF format.
    pub fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.validate()?;
        // SMF := <header_chunk> + <track_chunk> [ + <track_chunk> ... ]
        Ok(self.encode_header(w)? + self.encode_tracks(w)?)
    }

    /// Perform validations to guarantee the integrity of the data.
    fn validate(&self) -> std::io::Result<()> {
        if self.format == Format::SingleTrack {
            match self.tracks.len() {
                0 => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    Error::SingleTrackFormatContainsNoTracks,
                )),
                1 => Ok(()),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    Error::SingleTrackFormatContainsMoreThanOneTrack,
                )),
            }?;
        }
        match self.division {
            Division::TicksPerBeat(ticks) if ticks <= 0 => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                Error::DivisionTicksMustBePositive,
            )),
            _ => Ok(()),
        }?;
        Ok(())
    }

    /// Write the encoded header chunk to `w`.
    fn encode_header<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let header_length: u32 = 6;
        let format: u16 = self.format as u16;
        let num_tracks = self.tracks.len() as u16;
        let division = self.division.encode();
        // header chunk := "MThd" + <header_length:4> + <format:2> + <num_tracks:2> + <time_division:2>
        let mut bytes_written = w.write(b"MThd")?;
        bytes_written += w.write(&header_length.to_be_bytes())?;
        bytes_written += w.write(&format.to_be_bytes())?;
        bytes_written += w.write(&num_tracks.to_be_bytes())?;
        bytes_written += w.write(&division.to_be_bytes())?;
        Ok(bytes_written)
    }

    /// Encode and write all tracks to `w`.
    fn encode_tracks<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut bytes_written = 0;
        for track in self.tracks.iter() {
            bytes_written += self.encode_track(w, track)?;
        }
        Ok(bytes_written)
    }

    /// Encode and write a single track to `w`.
    fn encode_track<W: Write>(&self, w: &mut W, track: &[TrackEvent]) -> std::io::Result<usize> {
        let track_bytes: u32 = track.iter().map(|e| e.bytes_len()).sum();
        // track_chunk := "MTrk" + <length:4> + <track_event> [ + <track_event> ... ]
        let mut bytes_written = w.write(b"MTrk")?;
        bytes_written += w.write(&track_bytes.to_be_bytes())?;
        for e in track.iter() {
            bytes_written += e.encode(w)?;
        }
        Ok(bytes_written)
    }
}

/// Meta events are non-MIDI data of various sorts consisting of a code and actual event data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetaEvent {
    SequenceNumber(String),
    TextEvent(String),
    CopyrightNotice(String),
    EndOfTrack,
}

impl MetaEvent {
    /// The number of bytes the event will take up once encoded.
    fn bytes_size(&self) -> usize {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap()
    }

    fn event_code(&self) -> u8 {
        match self {
            MetaEvent::SequenceNumber(_) => 0x00,
            MetaEvent::TextEvent(_) => 0x01,
            MetaEvent::CopyrightNotice(_) => 0x02,
            MetaEvent::EndOfTrack => 0x03,
        }
    }

    fn data(&self) -> &[u8] {
        match self {
            MetaEvent::SequenceNumber(s) => s.as_bytes(),
            MetaEvent::TextEvent(s) => s.as_bytes(),
            MetaEvent::CopyrightNotice(s) => s.as_bytes(),
            MetaEvent::EndOfTrack => &[],
        }
    }

    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let code = self.event_code();
        let data = self.data();
        Ok(
            w.write(&[0xFF, code])?
                + w.write(&encode_varint(data.len() as u64))?
                + w.write(data)?,
        )
    }
}

#[derive(Clone, Debug)]
pub enum TrackEvent<'a> {
    Midi {
        time: u64,
        event: crate::MidiMessage<'a>,
    },
    Meta {
        time: u64,
        event: MetaEvent,
    },
}

impl<'a> TrackEvent<'a> {
    fn time(&self) -> u64 {
        match self {
            TrackEvent::Midi { time, .. } => *time,
            TrackEvent::Meta { time, .. } => *time,
        }
    }

    fn bytes_len(&self) -> u32 {
        encode_varint(self.time()).len() as u32
            + match self {
                TrackEvent::Midi { event, .. } => event.bytes_size() as u32,
                TrackEvent::Meta { event, .. } => event.bytes_size() as u32,
            }
    }

    /// Write the encoded contents of the event to `w`.
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        match self {
            TrackEvent::Midi { time, event, .. } => {
                Ok(w.write(&encode_varint(*time))? + event.write(w)?)
            }
            TrackEvent::Meta { time, event, .. } => {
                Ok(w.write(&encode_varint(*time))? + event.encode(w)?)
            }
        }
    }
}

/// The format of the tracks structure.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Format {
    /// There is a single track.
    SingleTrack = 0,
    /// There are multiple tracks.
    MultipleTracks = 1,
    /// Each song represents a song. Similar to a series of SingleTrack.
    MultipleSong = 2,
}

/// Unit of time for delta timing.
#[derive(Copy, Clone, Debug)]
pub enum Division {
    /// The number of units per beat. For example, 96 would mean 96 ticks per beat.
    TicksPerBeat(i16),
    // TODO(wmedrano): Add SMPTE
}

impl Division {
    fn encode(&self) -> u16 {
        match self {
            Division::TicksPerBeat(ticks) => (*ticks & 0x7F) as u16,
        }
    }
}

fn encode_varint(val: u64) -> Vec<u8> {
    let mut storage = Vec::new();
    let mut cur = val;
    let mut continuation = false;
    let cont_mask = 0x80 as u8;
    let val_mask = 0x7F as u64;
    loop {
        let mut to_write = (cur & val_mask) as u8;
        cur = cur >> 7;
        if continuation {
            // we're writing a continuation byte, so set the bit
            to_write |= cont_mask;
        }
        storage.push(to_write);
        continuation = true;
        if cur == 0 {
            break;
        }
    }
    storage.reverse();
    storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint() {
        struct TestCase {
            int: u64,
            want: &'static [u8],
        }
        let test_cases = [
            TestCase {
                int: 127,
                want: &[0x7F],
            },
            TestCase {
                int: 255,
                want: &[0x81, 0x7F],
            },
            TestCase {
                int: 32768,
                want: &[0x82, 0x80, 0x00],
            },
        ];
        for tc in test_cases.iter() {
            let buffer = encode_varint(tc.int);
            assert_eq!(&buffer, &tc.want);
        }
    }
}
