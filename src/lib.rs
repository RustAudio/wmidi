use std::io::Write;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MidiMessage<'a> {
    NoteOff(Channel, Note, Velocity),
    NoteOn(Channel, Note, Velocity),
    PolyphonicKeyPressure(Channel, Note, Velocity),
    ControlChange(Channel, ControlNumber, ControlValue),
    ProgramChange(Channel, ProgramNumber),
    ChannelPressure(Channel, Velocity),
    PitchBendChange(Channel, PitchBend),
    SysEx(&'a [u8]),
    MidiTimeCode(i8),
    SongPositionPointer(SongPosition),
    SongSelect(Song),
    Reserved(u8),
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

impl<'a> MidiMessage<'a> {
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
        let end_i = bytes
            .iter()
            .cloned()
            .position(is_status_byte)
            .ok_or(Error::FoundNoSysExEndByte)?;
        if bytes[end_i] != 0xF7 {
            return Err(Error::UnexpectedNonSysExEndByte(bytes[end_i]));
        }
        Ok(MidiMessage::SysEx(&bytes[1..end_i]))
    }

    pub fn drop_sysex(self) -> Option<MidiMessage<'static>> {
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

    pub fn write(&self, w: &mut Write) -> Result<usize, io::Error> {
        match self {
            &MidiMessage::NoteOff(a, b, c) => w.write(&[0x80 | a.index(), b as u8, c as u8]),
            &MidiMessage::NoteOn(a, b, c) => w.write(&[0x90 | a.index(), b as u8, c as u8]),
            &MidiMessage::PolyphonicKeyPressure(a, b, c) => {
                w.write(&[0xA0 | a.index(), b as u8, c as u8])
            }
            &MidiMessage::ControlChange(a, b, c) => w.write(&[0xB0 | a.index(), b as u8, c as u8]),
            &MidiMessage::ProgramChange(a, b) => w.write(&[0xC0 | a.index(), b as u8]),
            &MidiMessage::ChannelPressure(a, b) => w.write(&[0xD0 | a.index(), b as u8]),
            &MidiMessage::PitchBendChange(a, b) => {
                w.write(&[0xE0 | a.index()])?;
                w.write(&split_data(b))
            }
            &MidiMessage::SysEx(b) => {
                w.write(&[0xF0])?;
                w.write(b)?;
                w.write(&[0xF7])
            }
            &MidiMessage::MidiTimeCode(a) => w.write(&[0xF1, a as u8]),
            &MidiMessage::SongPositionPointer(a) => {
                w.write(&[0xF2])?;
                w.write(&split_data(a))
            }
            &MidiMessage::SongSelect(a) => w.write(&[0xF3, a as u8]),
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

#[derive(Debug, Copy, Clone)]
pub enum Error {
    ChannelOutOfRange,
    ExpectedSysExStartByte,
    FoundNoSysExEndByte,
    NoBytes,
    NotEnoughBytes,
    UnexpectedEndSysExByte,
    UnexpectedNonSysExEndByte(u8),
    UnexpectedStatusByte,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    pub fn number(&self) -> u8 {
        self.index() + 1
    }
}

pub type Note = i8;
pub type ControlNumber = i8;
pub type Velocity = i8;
pub type ControlValue = i8;
pub type ProgramNumber = i8;
pub type PitchBend = i16;
pub type SongPosition = i16;
pub type Song = i8;

#[inline(always)]
fn combine_data(lower: i8, higher: i8) -> i16 {
    lower as i16 + 127 * higher as i16
}

#[inline(always)]
fn split_data(data: i16) -> [u8; 2] {
    [(data % 127) as u8, (data / 127) as u8]
}

#[inline(always)]
fn is_status_byte(b: u8) -> bool {
    b & 0x80 == 0x80
}

#[inline(always)]
fn valid_data_byte(b: &u8) -> Result<i8, Error> {
    let x = b.clone();
    if is_status_byte(x) {
        Err(Error::UnexpectedStatusByte)
    } else {
        Ok(x as i8)
    }
}
