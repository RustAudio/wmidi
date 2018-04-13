use std::io;
use std::io::Write;

/// Holds information based on the Midi 1.0 spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MidiMessage<'a> {
    /// This message is sent when a note is released (ended).
    NoteOff(Channel, Note, Velocity),

    /// This message is sent when a note is depressed (start).
    NoteOn(Channel, Note, Velocity),

    /// This message is most often sent by pressing down on the key after it
    /// "bottoms out".
    PolyphonicKeyPressure(Channel, Note, Velocity),

    /// This message is sent when a controller value changes. Controllers include devices such as
    /// pedals and levers.
    ///
    /// Controller numbers 120-127 are reserved as "Channel Mode Messages".
    ControlChange(Channel, ControlNumber, ControlValue),

    /// This message is sent when the patch number changes.
    ProgramChange(Channel, ProgramNumber),

    /// This message is most often sent by pressing down on the key after it "bottoms out". This
    /// message is different from polyphonic after-touch. Use this message to send the single
    /// greatest pressure value (of all the current depressed keys).
    ChannelPressure(Channel, Velocity),

    /// This message is sent to indicate a change in the pitch bender (wheel or level, typically).
    /// The pitch bender is measured by a fourteen bit value. Center is 8192.
    PitchBendChange(Channel, PitchBend),

    /// This message type allows manufacturers to create their own messages (such as bulk dumps,
    /// patch parameters, and other non-spec data) and provides a mechanism for creating
    /// additional MIDI Specification messages.
    ///
    /// In the data held by the SysEx message, the Manufacturer's ID
    /// code (assigned by MMA or AMEI) is either 1 byte or 3
    /// bytes. Two of the 1 Byte IDs are reserved for extensions
    /// called Universal Exclusive Messages, which are not
    /// manufacturer-specific. If a device recognizes the ID code as
    /// its own (or as a supported Universal message) it will listen
    /// to the rest of the message. Otherwise the message will be
    /// ignored.
    SysEx(&'a [U7]),

    /// MIDI Time Code Quarter Frame.
    ///
    /// The data is in the format 0nnndddd where nnn is the Message Type and dddd is the Value.
    ///
    /// TODO: Interpret data instead of providing the raw format.
    MidiTimeCode(U7),

    /// This is an internal 14 bit value that holds the number of MIDI beats (1 beat = six MIDI
    /// clocks) since the start of the song.
    SongPositionPointer(SongPosition),

    /// The Song Select specifies which sequence or song is to be played.
    SongSelect(Song),

    /// The u8 data holds the status byte.
    Reserved(u8),

    /// Upon receiving a Tune Request, all analog synthesizers should tune
    /// their oscillators.
    TuneRequest,

    /// Timing Clock. Sent 24 times per quarter note when synchronization is
    /// required.
    TimingClock,

    /// Start the current sequence playing. (This message will be followed with
    /// Timing Clocks).
    Start,

    /// Continue at the point the sequence was Stopped.
    Continue,

    /// Stop the current sequence.
    Stop,

    /// This message is intended to be sent repeatedly to tell the receiver that a connection is
    /// alive. Use of this message is optional. When initially received, the receiver will expect
    /// to receive another Active Sensing message each 300ms (max), and if it idoes not, then it
    /// will assume that the connection has been terminated. At termination, the receiver will
    /// turn off all voices and return to normal (non-active sensing) operation.
    ActiveSensing,

    /// Reset all receivers in the system to power-up status. This should be used sparingly,
    /// preferably under manual control. In particular, it should not be sent on power-up.
    Reset,
}

impl<'a> MidiMessage<'a> {
    /// Construct a midi message from bytes.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, Error> {
        if bytes.is_empty() {
            return Err(Error::NoBytes);
        }
        let chan = Channel::from_index(bytes[0] & 0x0F)?;
        let data_a = bytes
            .get(1)
            .ok_or(Error::NotEnoughBytes)
            .and_then(valid_data_byte);
        let data_b = bytes
            .get(2)
            .ok_or(Error::NotEnoughBytes)
            .and_then(valid_data_byte);
        match bytes[0] & 0xF0 {
            0x80 => Ok(MidiMessage::NoteOff(chan, data_a?, data_b?)),
            0x90 => Ok(MidiMessage::NoteOn(chan, data_a?, data_b?)),
            0xA0 => Ok(MidiMessage::PolyphonicKeyPressure(chan, data_a?, data_b?)),
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
                    data_a?,
                    data_b?,
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

    #[inline(always)]
    fn new_sysex(bytes: &'a [u8]) -> Result<Self, Error> {
        debug_assert!(bytes[0] == 0xF0);
        let end_i = 1
            + bytes[1..]
                .iter()
                .cloned()
                .position(is_status_byte)
                .ok_or(Error::NoSysExEndByte)?;
        if bytes[end_i] != 0xF7 {
            return Err(Error::UnexpectedNonSysExEndByte(bytes[end_i]));
        }
        Ok(MidiMessage::SysEx(&bytes[1..end_i]))
    }

    /// Return `Some(midi_message)` if `self` is not a SysEx message, or `None`
    /// if it is. This expands the lifetime of the `MidiMessage` from `'a` to
    /// `'static`.
    pub fn drop_sysex(self) -> Option<MidiMessage<'static>> {
        match self {
            MidiMessage::NoteOff(a, b, c) => Some(MidiMessage::NoteOff(a, b, c)),
            MidiMessage::NoteOn(a, b, c) => Some(MidiMessage::NoteOff(a, b, c)),
            MidiMessage::PolyphonicKeyPressure(a, b, c) => {
                Some(MidiMessage::PolyphonicKeyPressure(a, b, c))
            },
            MidiMessage::ControlChange(a, b, c) => Some(MidiMessage::ControlChange(a, b, c)),
            MidiMessage::ProgramChange(a, b) => Some(MidiMessage::ProgramChange(a, b)),
            MidiMessage::ChannelPressure(a, b) => Some(MidiMessage::ChannelPressure(a, b)),
            MidiMessage::PitchBendChange(a, b) => Some(MidiMessage::PitchBendChange(a, b)),
            MidiMessage::SysEx(_) => None,
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

    /// The number of bytes the MIDI message takes.
    pub fn wire_size(&self) -> usize {
        match self {
            &MidiMessage::NoteOff(..) => 3,
            &MidiMessage::NoteOn(..) => 3,
            &MidiMessage::PolyphonicKeyPressure(..) => 3,
            &MidiMessage::ControlChange(..) => 3,
            &MidiMessage::ProgramChange(..) => 2,
            &MidiMessage::ChannelPressure(..) => 2,
            &MidiMessage::PitchBendChange(..) => 2,
            &MidiMessage::SysEx(ref b) => 2 + b.len(),
            &MidiMessage::MidiTimeCode(_) => 2,
            &MidiMessage::SongPositionPointer(_) => 3,
            &MidiMessage::SongSelect(_) => 2,
            &MidiMessage::Reserved(_) => 1,
            &MidiMessage::TuneRequest => 1,
            &MidiMessage::TimingClock => 1,
            &MidiMessage::Start => 1,
            &MidiMessage::Continue => 1,
            &MidiMessage::Stop => 1,
            &MidiMessage::ActiveSensing => 1,
            &MidiMessage::Reset => 1,
        }
    }

    /// The channel associated with the MIDI message, if applicable for the
    /// message type.
    pub fn channel(&self) -> Option<Channel> {
        match self {
            &MidiMessage::NoteOff(c, ..) => Some(c),
            &MidiMessage::NoteOn(c, ..) => Some(c),
            &MidiMessage::PolyphonicKeyPressure(c, ..) => Some(c),
            &MidiMessage::ControlChange(c, ..) => Some(c),
            &MidiMessage::ProgramChange(c, ..) => Some(c),
            &MidiMessage::ChannelPressure(c, ..) => Some(c),
            &MidiMessage::PitchBendChange(c, ..) => Some(c),
            _ => None,
        }
    }

    /// Write the contents of the MIDI message as raw MIDI bytes.
    pub fn write(&self, w: &mut Write) -> Result<usize, io::Error> {
        match self {
            &MidiMessage::NoteOff(a, b, c) => w.write(&[0x80 | a.index(), b, c]),
            &MidiMessage::NoteOn(a, b, c) => w.write(&[0x90 | a.index(), b, c]),
            &MidiMessage::PolyphonicKeyPressure(a, b, c) => w.write(&[0xA0 | a.index(), b, c]),
            &MidiMessage::ControlChange(a, b, c) => w.write(&[0xB0 | a.index(), b, c]),
            &MidiMessage::ProgramChange(a, b) => w.write(&[0xC0 | a.index(), b]),
            &MidiMessage::ChannelPressure(a, b) => w.write(&[0xD0 | a.index(), b]),
            &MidiMessage::PitchBendChange(a, b) => {
                w.write(&[0xE0 | a.index()])?;
                w.write(&split_data(b))
            },
            &MidiMessage::SysEx(b) => {
                w.write(&[0xF0])?;
                w.write(b)?;
                w.write(&[0xF7])
            },
            &MidiMessage::MidiTimeCode(a) => w.write(&[0xF1, a]),
            &MidiMessage::SongPositionPointer(a) => {
                w.write(&[0xF2])?;
                w.write(&split_data(a))
            },
            &MidiMessage::SongSelect(a) => w.write(&[0xF3, a]),
            &MidiMessage::Reserved(a) => w.write(&[a]),
            &MidiMessage::TuneRequest => w.write(&[0xF6]),
            &MidiMessage::TimingClock => w.write(&[0xF8]),
            &MidiMessage::Start => w.write(&[0xFA]),
            &MidiMessage::Continue => w.write(&[0xFB]),
            &MidiMessage::Stop => w.write(&[0xFC]),
            &MidiMessage::ActiveSensing => w.write(&[0xFE]),
            &MidiMessage::Reset => w.write(&[0xFF]),
        }
    }
}

/// Midi encoding and decoding errors.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The MIDI channel is not between 1 and 16 inclusive.
    ChannelOutOfRange,

    /// No MIDI bytes were provided.
    NoBytes,

    /// A SysEx start byte was provided, but there was no corresponding SysEx
    /// end byte.
    NoSysExEndByte,

    /// Not enough data bytes for the specified MIDI message.
    NotEnoughBytes,

    /// Found a SysEx end byte, but there was no start byte.
    UnexpectedEndSysExByte,

    /// Found a status byte interleaved with SysEx data. SysEx messages should
    /// be a start byte, followed by data bytes, and ending in a end byte.
    UnexpectedNonSysExEndByte(u8),

    /// Found a status byte, but expected a `U7` data byte.
    UnexpectedStatusByte,
}

/// A data byte that holds 7 bits of information.
pub type U7 = u8;

/// A combination of 2 data bytes that holds 14 bits of information.
pub type U14 = u16;

/// Specifies a MIDI note.
pub type Note = U7;

/// Specifies the velocity of an action (often key press, release, or
/// aftertouch).
pub type Velocity = U7;

/// Specifies a MIDI control number.
pub type ControlNumber = U7;

/// Specifies the value of a MIDI control.
pub type ControlValue = U7;

/// Specifies a program. Sometimes known as patch.
pub type ProgramNumber = U7;

/// A 14bit value specifying the pitch bend. Neutral is 8192.
pub type PitchBend = U14;

/// 14 bit value that holds the number of MIDI beats (1 beat = six MIDI clocks)
/// since the start of the song.
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
    pub fn index(&self) -> u8 {
        match self {
            &Channel::Ch1 => 0,
            &Channel::Ch2 => 1,
            &Channel::Ch3 => 2,
            &Channel::Ch4 => 3,
            &Channel::Ch5 => 4,
            &Channel::Ch6 => 5,
            &Channel::Ch7 => 6,
            &Channel::Ch8 => 7,
            &Channel::Ch9 => 8,
            &Channel::Ch10 => 9,
            &Channel::Ch11 => 10,
            &Channel::Ch12 => 11,
            &Channel::Ch13 => 12,
            &Channel::Ch14 => 13,
            &Channel::Ch15 => 14,
            &Channel::Ch16 => 15,
        }
    }

    /// The number of this midi channel. The returned value is between 1 and 16
    /// inclusive.
    pub fn number(&self) -> u8 { self.index() + 1 }
}

#[inline(always)]
fn combine_data(lower: U7, higher: U7) -> U14 { (lower as U14) + 128 * (higher as U14) }

#[inline(always)]
fn split_data(data: U14) -> [U7; 2] { [(data % 128) as U7, (data / 128) as U7] }

#[inline(always)]
fn is_status_byte(b: u8) -> bool { b & 0x80 == 0x80 }

#[inline(always)]
fn valid_data_byte(b: &u8) -> Result<U7, Error> {
    let x = b.clone();
    if is_status_byte(x) {
        Err(Error::UnexpectedStatusByte)
    } else {
        Ok(x as U7)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_raw() {
        assert_eq!(MidiMessage::from_bytes(&[0x84]), Err(Error::NotEnoughBytes));
        assert_eq!(
            MidiMessage::from_bytes(&[0x84, 64]),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::from_bytes(&[0x84, 64, 100]),
            Ok(MidiMessage::NoteOff(Channel::Ch5, 64, 100))
        );

        assert_eq!(MidiMessage::from_bytes(&[0x94]), Err(Error::NotEnoughBytes));
        assert_eq!(
            MidiMessage::from_bytes(&[0x94, 64]),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::from_bytes(&[0x94, 64, 100]),
            Ok(MidiMessage::NoteOn(Channel::Ch5, 64, 100))
        );

        assert_eq!(
            MidiMessage::from_bytes(&[0xF0, 4, 8, 12, 16, 0xF7]),
            Ok(MidiMessage::SysEx(&[4, 8, 12, 16]))
        );
        assert_eq!(
            MidiMessage::from_bytes(&[0xF0, 3, 6, 9, 12, 15, 0xF7, 125]),
            Ok(MidiMessage::SysEx(&[3, 6, 9, 12, 15]))
        );
        assert_eq!(
            MidiMessage::from_bytes(&[0xF0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Err(Error::NoSysExEndByte)
        );

        assert_eq!(MidiMessage::from_bytes(&[0xE4]), Err(Error::NotEnoughBytes));
        assert_eq!(
            MidiMessage::from_bytes(&[0xE4, 64]),
            Err(Error::NotEnoughBytes)
        );
        assert_eq!(
            MidiMessage::from_bytes(&[0xE4, 64, 100]),
            Ok(MidiMessage::PitchBendChange(Channel::Ch5, 12864))
        );
    }

    #[test]
    fn write() {
        let mut b = [0u8; 6];
        {
            let mut b: &mut [u8] = &mut b;
            MidiMessage::PolyphonicKeyPressure(Channel::Ch10, 93, 43)
                .write(&mut b)
                .unwrap();
        }
        let b: &[u8] = &b;
        assert_eq!(b, &[0xA9, 93, 43, 0, 0, 0]);
    }

    #[test]
    fn write_sysex() {
        let mut b = [0u8; 8];
        {
            let mut b: &mut [u8] = &mut b;
            MidiMessage::SysEx(&[10, 20, 30, 40, 50])
                .write(&mut b)
                .unwrap();
        }
        let b: &[u8] = &b;
        assert_eq!(b, &[0xF0, 10, 20, 30, 40, 50, 0xF7, 0]);
    }

    #[test]
    fn drop_sysex() {
        assert_eq!(MidiMessage::SysEx(&[1, 2, 3]).drop_sysex(), None);
        assert_eq!(
            MidiMessage::TuneRequest.drop_sysex(),
            Some(MidiMessage::TuneRequest)
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            MidiMessage::ControlChange(Channel::Ch8, 1, 55).channel(),
            Some(Channel::Ch8)
        );
        assert_eq!(MidiMessage::Start.channel(), None);
    }
}
