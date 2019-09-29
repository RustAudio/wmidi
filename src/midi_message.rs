use crate::{Error, Note, U7};
use std::convert::TryFrom;
use std::io;
use std::io::Write;

/// Holds information based on the Midi 1.0 spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MidiMessage<'a> {
    /// This message is sent when a note is released (ended).
    NoteOff(Channel, Note, Velocity),

    /// This message is sent when a note is depressed (start).
    NoteOn(Channel, Note, Velocity),

    /// This message is most often sent by pressing down on the key after it "bottoms out".
    PolyphonicKeyPressure(Channel, Note, Velocity),

    /// This message is sent when a controller value changes. Controllers include devices such as pedals and levers.
    ///
    /// Controller numbers 120-127 are reserved as "Channel Mode Messages".
    ControlChange(Channel, ControlNumber, ControlValue),

    /// This message is sent when the patch number changes.
    ProgramChange(Channel, ProgramNumber),

    /// This message is most often sent by pressing down on the key after it "bottoms out". This message is different
    /// from polyphonic after-touch. Use this message to send the single greatest pressure value (of all the current
    /// depressed keys).
    ChannelPressure(Channel, Velocity),

    /// This message is sent to indicate a change in the pitch bender (wheel or level, typically). The pitch bender is
    /// measured by a fourteen bit value. Center is 8192.
    PitchBendChange(Channel, PitchBend),

    /// This message type allows manufacturers to create their own messages (such as bulk dumps, patch parameters, and
    /// other non-spec data) and provides a mechanism for creating additional MIDI Specification messages.
    ///
    /// In the data held by the SysEx message, the Manufacturer's ID code (assigned by MMA or AMEI) is either 1 byte or
    /// 3 bytes. Two of the 1 Byte IDs are reserved for extensions called Universal Exclusive Messages, which are not
    /// manufacturer-specific. If a device recognizes the ID code as its own (or as a supported Universal message) it
    /// will listen to the rest of the message. Otherwise the message will be ignored.
    SysEx(&'a [U7]),

    /// This message type allows manufacturers to create their own messages (such as bulk dumps, patch parameters, and
    /// other non-spec data) and provides a mechanism for creating additional MIDI Specification messages.
    ///
    /// In the data held by the SysEx message, the Manufacturer's ID code (assigned by MMA or AMEI) is either 1 byte or
    /// 3 bytes. Two of the 1 Byte IDs are reserved for extensions called Universal Exclusive Messages, which are not
    /// manufacturer-specific. If a device recognizes the ID code as its own (or as a supported Universal message) it
    /// will listen to the rest of the message. Otherwise the message will be ignored.
    OwnedSysEx(Vec<U7>),

    /// MIDI Time Code Quarter Frame.
    ///
    /// The data is in the format 0nnndddd where nnn is the Message Type and dddd is the Value.
    ///
    /// TODO: Interpret data instead of providing the raw format.
    MidiTimeCode(U7),

    /// This is an internal 14 bit value that holds the number of MIDI beats (1 beat = six MIDI clocks) since the start
    /// of the song.
    SongPositionPointer(SongPosition),

    /// The Song Select specifies which sequence or song is to be played.
    SongSelect(Song),

    /// The u8 data holds the status byte.
    Reserved(u8),

    /// Upon receiving a Tune Request, all analog synthesizers should tune their oscillators.
    TuneRequest,

    /// Timing Clock. Sent 24 times per quarter note when synchronization is required.
    TimingClock,

    /// Start the current sequence playing. (This message will be followed with Timing Clocks).
    Start,

    /// Continue at the point the sequence was Stopped.
    Continue,

    /// Stop the current sequence.
    Stop,

    /// This message is intended to be sent repeatedly to tell the receiver that a connection is alive. Use of this
    /// message is optional. When initially received, the receiver will expect to receive another Active Sensing message
    /// each 300ms (max), and if it idoes not, then it will assume that the connection has been terminated. At
    /// termination, the receiver will turn off all voices and return to normal (non-active sensing) operation.
    ActiveSensing,

    /// Reset all receivers in the system to power-up status. This should be used sparingly, preferably under manual
    /// control. In particular, it should not be sent on power-up.
    Reset,
}

impl<'a> TryFrom<&'a [u8]> for MidiMessage<'a> {
    type Error = Error;
    /// Construct a midi message from bytes.
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            return Err(Error::NoBytes);
        }
        let chan = Channel::from_index(bytes[0] & 0x0F)?;
        let data_a = bytes
            .get(1)
            .ok_or(Error::NotEnoughBytes)
            .and_then(|b| valid_data_byte(*b));
        let data_b = bytes
            .get(2)
            .ok_or(Error::NotEnoughBytes)
            .and_then(|b| valid_data_byte(*b));
        match bytes[0] & 0xF0 {
            0x80 => Ok(MidiMessage::NoteOff(chan, Note::from(data_a?), data_b?)),
            0x90 => Ok(MidiMessage::NoteOn(chan, Note::from(data_a?), data_b?)),
            0xA0 => Ok(MidiMessage::PolyphonicKeyPressure(
                chan,
                Note::from(data_a?),
                data_b?,
            )),
            0xB0 => Ok(MidiMessage::ControlChange(chan, data_a?, data_b?)),
            0xC0 => Ok(MidiMessage::ProgramChange(chan, data_a?)),
            0xD0 => Ok(MidiMessage::ChannelPressure(chan, data_a?)),
            0xE0 => Ok(MidiMessage::PitchBendChange(
                chan,
                combine_data(data_a?, data_b?),
            )),
            0xF0 => match bytes[0] {
                0xF0 => MidiMessage::new_sysex(bytes),
                0xF1 => Ok(MidiMessage::MidiTimeCode(data_a?)),
                0xF2 => Ok(MidiMessage::SongPositionPointer(combine_data(
                    data_a?, data_b?,
                ))),
                0xF3 => Ok(MidiMessage::SongSelect(data_a?)),
                0xF4 | 0xF5 => Ok(MidiMessage::Reserved(bytes[0])),
                0xF6 => Ok(MidiMessage::TuneRequest),
                0xF7 => Err(Error::UnexpectedEndSysExByte),
                0xF8 => Ok(MidiMessage::TimingClock),
                0xF9 => Ok(MidiMessage::Reserved(bytes[0])),
                0xFA => Ok(MidiMessage::Start),
                0xFB => Ok(MidiMessage::Continue),
                0xFC => Ok(MidiMessage::Stop),
                0xFD => Ok(MidiMessage::Reserved(bytes[0])),
                0xFE => Ok(MidiMessage::ActiveSensing),
                0xFF => Ok(MidiMessage::Reset),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> MidiMessage<'a> {
    /// Construct a midi message from bytes. Use `MidiMessage::try_from(bytes)` instead.
    #[deprecated(since = "2.0.0", note = "Use MidiMessage::try_from instead.")]
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, Error> {
        MidiMessage::try_from(bytes)
    }

    /// Return `Some(midi_message)` if `self` is not a SysEx message, or `None` if it is. This expands the lifetime of
    /// the `MidiMessage` from `'a` to `'static`.
    pub fn drop_unowned_sysex(self) -> Option<MidiMessage<'static>> {
        match self {
            MidiMessage::NoteOff(a, b, c) => Some(MidiMessage::NoteOff(a, b, c)),
            MidiMessage::NoteOn(a, b, c) => Some(MidiMessage::NoteOff(a, b, c)),
            MidiMessage::PolyphonicKeyPressure(a, b, c) => {
                Some(MidiMessage::PolyphonicKeyPressure(a, b, c))
            }
            MidiMessage::ControlChange(a, b, c) => Some(MidiMessage::ControlChange(a, b, c)),
            MidiMessage::ProgramChange(a, b) => Some(MidiMessage::ProgramChange(a, b)),
            MidiMessage::ChannelPressure(a, b) => Some(MidiMessage::ChannelPressure(a, b)),
            MidiMessage::PitchBendChange(a, b) => Some(MidiMessage::PitchBendChange(a, b)),
            MidiMessage::SysEx(_) => None,
            MidiMessage::OwnedSysEx(bytes) => Some(MidiMessage::OwnedSysEx(bytes)),
            MidiMessage::MidiTimeCode(a) => Some(MidiMessage::MidiTimeCode(a)),
            MidiMessage::SongPositionPointer(a) => Some(MidiMessage::SongPositionPointer(a)),
            MidiMessage::SongSelect(a) => Some(MidiMessage::SongSelect(a)),
            MidiMessage::Reserved(a) => Some(MidiMessage::Reserved(a)),
            MidiMessage::TuneRequest => Some(MidiMessage::TuneRequest),
            MidiMessage::TimingClock => Some(MidiMessage::TimingClock),
            MidiMessage::Start => Some(MidiMessage::Start),
            MidiMessage::Continue => Some(MidiMessage::Continue),
            MidiMessage::Stop => Some(MidiMessage::Stop),
            MidiMessage::ActiveSensing => Some(MidiMessage::ActiveSensing),
            MidiMessage::Reset => Some(MidiMessage::Reset),
        }
    }

    /// Take ownership of the SysEx data. This expands the lifetime of the message to `'static`. If `'static` lifetime
    /// is needed but SysEx messages can be dropped, consider using `self.drop_unowned_sysex()`.
    #[inline(always)]
    pub fn to_owned(&self) -> MidiMessage<'static> {
        match self.clone() {
            MidiMessage::NoteOff(a, b, c) => MidiMessage::NoteOff(a, b, c),
            MidiMessage::NoteOn(a, b, c) => MidiMessage::NoteOff(a, b, c),
            MidiMessage::PolyphonicKeyPressure(a, b, c) => {
                MidiMessage::PolyphonicKeyPressure(a, b, c)
            }
            MidiMessage::ControlChange(a, b, c) => MidiMessage::ControlChange(a, b, c),
            MidiMessage::ProgramChange(a, b) => MidiMessage::ProgramChange(a, b),
            MidiMessage::ChannelPressure(a, b) => MidiMessage::ChannelPressure(a, b),
            MidiMessage::PitchBendChange(a, b) => MidiMessage::PitchBendChange(a, b),
            MidiMessage::SysEx(bytes) => MidiMessage::OwnedSysEx(bytes.to_vec()),
            MidiMessage::OwnedSysEx(bytes) => MidiMessage::OwnedSysEx(bytes),
            MidiMessage::MidiTimeCode(a) => MidiMessage::MidiTimeCode(a),
            MidiMessage::SongPositionPointer(a) => MidiMessage::SongPositionPointer(a),
            MidiMessage::SongSelect(a) => MidiMessage::SongSelect(a),
            MidiMessage::Reserved(a) => MidiMessage::Reserved(a),
            MidiMessage::TuneRequest => MidiMessage::TuneRequest,
            MidiMessage::TimingClock => MidiMessage::TimingClock,
            MidiMessage::Start => MidiMessage::Start,
            MidiMessage::Continue => MidiMessage::Continue,
            MidiMessage::Stop => MidiMessage::Stop,
            MidiMessage::ActiveSensing => MidiMessage::ActiveSensing,
            MidiMessage::Reset => MidiMessage::Reset,
        }
    }

    /// The number of bytes the MIDI message takes when encoded with the `std::io::Read` trait.
    pub fn wire_size(&self) -> usize {
        match self {
            MidiMessage::NoteOff(..) => 3,
            MidiMessage::NoteOn(..) => 3,
            MidiMessage::PolyphonicKeyPressure(..) => 3,
            MidiMessage::ControlChange(..) => 3,
            MidiMessage::ProgramChange(..) => 2,
            MidiMessage::ChannelPressure(..) => 2,
            MidiMessage::PitchBendChange(..) => 2,
            MidiMessage::SysEx(b) => 2 + b.len(),
            MidiMessage::OwnedSysEx(b) => 2 + b.len(),
            MidiMessage::MidiTimeCode(_) => 2,
            MidiMessage::SongPositionPointer(_) => 3,
            MidiMessage::SongSelect(_) => 2,
            MidiMessage::Reserved(_) => 1,
            MidiMessage::TuneRequest => 1,
            MidiMessage::TimingClock => 1,
            MidiMessage::Start => 1,
            MidiMessage::Continue => 1,
            MidiMessage::Stop => 1,
            MidiMessage::ActiveSensing => 1,
            MidiMessage::Reset => 1,
        }
    }

    /// The channel associated with the MIDI message, if applicable for the message type.
    pub fn channel(&self) -> Option<Channel> {
        match self {
            MidiMessage::NoteOff(c, ..) => Some(*c),
            MidiMessage::NoteOn(c, ..) => Some(*c),
            MidiMessage::PolyphonicKeyPressure(c, ..) => Some(*c),
            MidiMessage::ControlChange(c, ..) => Some(*c),
            MidiMessage::ProgramChange(c, ..) => Some(*c),
            MidiMessage::ChannelPressure(c, ..) => Some(*c),
            MidiMessage::PitchBendChange(c, ..) => Some(*c),
            _ => None,
        }
    }

    #[inline(always)]
    fn new_sysex(bytes: &'a [u8]) -> Result<Self, Error> {
        debug_assert!(bytes[0] == 0xF0);
        let end_i = 1 + bytes[1..]
            .iter()
            .copied()
            .position(is_status_byte)
            .ok_or(Error::NoSysExEndByte)?;
        if bytes[end_i] != 0xF7 {
            return Err(Error::UnexpectedNonSysExEndByte(bytes[end_i]));
        }
        // We've already gone through the bytes to find the first non data byte so we are assured
        // that values from 1..end_i are valid data bytes.
        let data_bytes = unsafe { U7::from_bytes_unchecked(&bytes[1..end_i]) };
        Ok(MidiMessage::SysEx(data_bytes))
    }
}

impl<'a> io::Read for MidiMessage<'a> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        match self {
            MidiMessage::NoteOff(a, b, c) => {
                buf.write(&[0x80 | a.index(), u8::from(*b), u8::from(*c)])
            }
            MidiMessage::NoteOn(a, b, c) => {
                buf.write(&[0x90 | a.index(), u8::from(*b), u8::from(*c)])
            }
            MidiMessage::PolyphonicKeyPressure(a, b, c) => {
                buf.write(&[0xA0 | a.index(), *b as u8, u8::from(*c)])
            }
            MidiMessage::ControlChange(a, b, c) => {
                buf.write(&[0xB0 | a.index(), u8::from(*b), u8::from(*c)])
            }
            MidiMessage::ProgramChange(a, b) => buf.write(&[0xC0 | a.index(), u8::from(*b)]),
            MidiMessage::ChannelPressure(a, b) => buf.write(&[0xD0 | a.index(), u8::from(*b)]),
            MidiMessage::PitchBendChange(a, b) => {
                let bytes_written = buf.write(&[0xE0 | a.index()])? + buf.write(&split_data(*b))?;
                Ok(bytes_written)
            }
            MidiMessage::SysEx(b) => {
                let bytes_written =
                    buf.write(&[0xF0])? + buf.write(U7::data_to_bytes(b))? + buf.write(&[0xF7])?;
                Ok(bytes_written)
            }
            MidiMessage::OwnedSysEx(ref b) => {
                let bytes_written =
                    buf.write(&[0xF0])? + buf.write(U7::data_to_bytes(&b))? + buf.write(&[0xF7])?;
                Ok(bytes_written)
            }
            MidiMessage::MidiTimeCode(a) => buf.write(&[0xF1, u8::from(*a)]),
            MidiMessage::SongPositionPointer(a) => {
                let bytes_written = buf.write(&[0xF2])? + buf.write(&split_data(*a))?;
                Ok(bytes_written)
            }
            MidiMessage::SongSelect(a) => buf.write(&[0xF3, u8::from(*a)]),
            MidiMessage::Reserved(a) => buf.write(&[*a]),
            MidiMessage::TuneRequest => buf.write(&[0xF6]),
            MidiMessage::TimingClock => buf.write(&[0xF8]),
            MidiMessage::Start => buf.write(&[0xFA]),
            MidiMessage::Continue => buf.write(&[0xFB]),
            MidiMessage::Stop => buf.write(&[0xFC]),
            MidiMessage::ActiveSensing => buf.write(&[0xFE]),
            MidiMessage::Reset => buf.write(&[0xFF]),
        }
    }
}

/// A combination of 2 data bytes that holds 14 bits of information.
pub type U14 = u16;

/// Specifies the velocity of an action (often key press, release, or aftertouch).
pub type Velocity = U7;

/// Specifies a MIDI control number.
pub type ControlNumber = U7;

/// Specifies the value of a MIDI control.
pub type ControlValue = U7;

/// Specifies a program. Sometimes known as patch.
pub type ProgramNumber = U7;

/// A 14bit value specifying the pitch bend. Neutral is 8192.
pub type PitchBend = U14;

/// 14 bit value that holds the number of MIDI beats (1 beat = six MIDI clocks) since the start of the song.
pub type SongPosition = U14;

/// A song or sequence.
pub type Song = U7;

/// The MIDI channel. There are 16 channels. They are numbered between 1 and 16
/// inclusive, or indexed between 0 and 15 inclusive.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
    Ch8,
    Ch9,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    Ch16,
}

impl Channel {
    /// Get a MIDI channel from an index that is between 0 and 15 inclusive.
    pub fn from_index(i: u8) -> Result<Channel, Error> {
        match i {
            0 => Ok(Channel::Ch1),
            1 => Ok(Channel::Ch2),
            2 => Ok(Channel::Ch3),
            3 => Ok(Channel::Ch4),
            4 => Ok(Channel::Ch5),
            5 => Ok(Channel::Ch6),
            6 => Ok(Channel::Ch7),
            7 => Ok(Channel::Ch8),
            8 => Ok(Channel::Ch9),
            9 => Ok(Channel::Ch10),
            10 => Ok(Channel::Ch11),
            11 => Ok(Channel::Ch12),
            12 => Ok(Channel::Ch13),
            13 => Ok(Channel::Ch14),
            14 => Ok(Channel::Ch15),
            15 => Ok(Channel::Ch16),
            _ => Err(Error::ChannelOutOfRange),
        }
    }

    /// The index of this midi channel. The returned value is between 0 and 15
    /// inclusive.
    pub fn index(self) -> u8 {
        match self {
            Channel::Ch1 => 0,
            Channel::Ch2 => 1,
            Channel::Ch3 => 2,
            Channel::Ch4 => 3,
            Channel::Ch5 => 4,
            Channel::Ch6 => 5,
            Channel::Ch7 => 6,
            Channel::Ch8 => 7,
            Channel::Ch9 => 8,
            Channel::Ch10 => 9,
            Channel::Ch11 => 10,
            Channel::Ch12 => 11,
            Channel::Ch13 => 12,
            Channel::Ch14 => 13,
            Channel::Ch15 => 14,
            Channel::Ch16 => 15,
        }
    }

    /// The number of this midi channel. The returned value is between 1 and 16
    /// inclusive.
    pub fn number(self) -> u8 {
        self.index() + 1
    }
}

#[inline(always)]
fn combine_data(lower: U7, higher: U7) -> U14 {
    u16::from(u8::from(lower)) + 128 * u16::from(u8::from(higher))
}

#[inline(always)]
fn split_data(data: U14) -> [u8; 2] {
    [(data % 128) as u8, (data / 128) as u8]
}

#[inline(always)]
fn is_status_byte(b: u8) -> bool {
    b & 0x80 == 0x80
}

#[inline(always)]
fn valid_data_byte(b: u8) -> Result<U7, Error> {
    U7::try_from(b).map_err(|_| Error::UnexpectedStatusByte)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Error, Note};
    use std::io::Read;

    #[test]
    fn try_from() {
        assert_eq!(
            MidiMessage::try_from([0x84].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0x84, 64].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0x84, 64, 100].as_ref()),
            Ok(MidiMessage::NoteOff(
                Channel::Ch5,
                Note::E3,
                U7::try_from(100).unwrap()
            ))
        );

        assert_eq!(
            MidiMessage::try_from([0x94].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0x94, 64].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0x94, 64, 100].as_ref()),
            Ok(MidiMessage::NoteOn(
                Channel::Ch5,
                Note::E3,
                U7::try_from(100).unwrap()
            ))
        );

        assert_eq!(
            MidiMessage::try_from([0xF0, 4, 8, 12, 16, 0xF7].as_ref()),
            Ok(MidiMessage::SysEx(
                U7::try_from_bytes(&[4, 8, 12, 16]).unwrap()
            ))
        );
        assert_eq!(
            MidiMessage::try_from([0xF0, 3, 6, 9, 12, 15, 0xF7, 125].as_ref()),
            Ok(MidiMessage::SysEx(
                U7::try_from_bytes(&[3, 6, 9, 12, 15]).unwrap()
            ))
        );
        assert_eq!(
            MidiMessage::try_from([0xF0, 1, 2, 3, 4, 5, 6, 7, 8, 9].as_ref()),
            Err(Error::NoSysExEndByte)
        );

        assert_eq!(
            MidiMessage::try_from([0xE4].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0xE4, 64].as_ref()),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::try_from([0xE4, 64, 100].as_ref()),
            Ok(MidiMessage::PitchBendChange(Channel::Ch5, 12864))
        );
    }

    #[test]
    fn read() {
        let b = {
            let mut b = [0u8; 6];
            let bytes_read = MidiMessage::PolyphonicKeyPressure(
                Channel::Ch10,
                Note::A5,
                U7::try_from(43).unwrap(),
            )
            .read(&mut b)
            .unwrap();
            assert_eq!(bytes_read, 3);
            b
        };
        assert_eq!(b, [0xA9, 93, 43, 0, 0, 0]);
    }

    #[test]
    fn read_sysex() {
        let b = {
            let mut b = [0u8; 8];
            let bytes_read = MidiMessage::SysEx(U7::try_from_bytes(&[10, 20, 30, 40, 50]).unwrap())
                .read(&mut b)
                .unwrap();
            assert_eq!(bytes_read, 7);
            b
        };
        assert_eq!(b, [0xF0, 10, 20, 30, 40, 50, 0xF7, 0]);
    }

    #[test]
    fn drop_unowned_sysex() {
        assert_eq!(
            MidiMessage::SysEx(U7::try_from_bytes(&[1, 2, 3]).unwrap()).drop_unowned_sysex(),
            None
        );
        assert_eq!(
            MidiMessage::OwnedSysEx(vec![
                U7::try_from(1).unwrap(),
                U7::try_from(2).unwrap(),
                U7::try_from(3).unwrap()
            ])
            .drop_unowned_sysex(),
            Some(MidiMessage::OwnedSysEx(vec![
                U7::try_from(1).unwrap(),
                U7::try_from(2).unwrap(),
                U7::try_from(3).unwrap()
            ]))
        );
        assert_eq!(
            MidiMessage::TuneRequest.drop_unowned_sysex(),
            Some(MidiMessage::TuneRequest)
        );
    }

    #[test]
    fn to_owned() {
        assert_eq!(
            MidiMessage::SysEx(U7::try_from_bytes(&[1, 2, 3]).unwrap()).to_owned(),
            MidiMessage::OwnedSysEx(vec![
                U7::try_from(1).unwrap(),
                U7::try_from(2).unwrap(),
                U7::try_from(3).unwrap()
            ])
        );
        assert_ne!(
            MidiMessage::SysEx(U7::try_from_bytes(&[1, 2, 3]).unwrap()).to_owned(),
            MidiMessage::SysEx(U7::try_from_bytes(&[1, 2, 3]).unwrap())
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            MidiMessage::ControlChange(
                Channel::Ch8,
                U7::try_from(7).unwrap(),
                U7::try_from(55).unwrap()
            )
            .channel(),
            Some(Channel::Ch8)
        );
        assert_eq!(MidiMessage::Start.channel(), None);
    }
}
