use crate::Error;
use core::convert::TryFrom;
use core::fmt;

/// A midi note.
///
/// The format for the enum is `$NOTE` `$MODIFIER?` `$OCTAVE`. Note can be a note from `A` to `G`.
/// Modifier can be `b` for flat or `Sharp` for sharp. Octave is the number. The octaves `-2` and
/// `-1` are `Minus2` and `Minus1` respectively.
/// # Example
/// ```
/// use wmidi::Note;
/// let ab7_chord = [Note::AbMinus1, Note::C4, Note::Gb4]; // We omit the 5th for a jazzier sound
/// let dmaj_chord = [Note::D2, Note::FSharp3, Note::A3];
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Note {
    CMinus2 = 0,
    DbMinus2 = 1,
    DMinus2 = 2,
    EbMinus2 = 3,
    EMinus2 = 4,
    FMinus2 = 5,
    GbMinus2 = 6,
    GMinus2 = 7,
    AbMinus2 = 8,
    AMinus2 = 9,
    BbMinus2 = 10,
    BMinus2 = 11,
    CMinus1 = 12,
    DbMinus1 = 13,
    DMinus1 = 14,
    EbMinus1 = 15,
    EMinus1 = 16,
    FMinus1 = 17,
    GbMinus1 = 18,
    GMinus1 = 19,
    AbMinus1 = 20,
    AMinus1 = 21,
    BbMinus1 = 22,
    BMinus1 = 23,
    C0 = 24,
    Db0 = 25,
    D0 = 26,
    Eb0 = 27,
    E0 = 28,
    F0 = 29,
    Gb0 = 30,
    G0 = 31,
    Ab0 = 32,
    A0 = 33,
    Bb0 = 34,
    B0 = 35,
    C1 = 36,
    Db1 = 37,
    D1 = 38,
    Eb1 = 39,
    E1 = 40,
    F1 = 41,
    Gb1 = 42,
    G1 = 43,
    Ab1 = 44,
    A1 = 45,
    Bb1 = 46,
    B1 = 47,
    C2 = 48,
    Db2 = 49,
    D2 = 50,
    Eb2 = 51,
    E2 = 52,
    F2 = 53,
    Gb2 = 54,
    G2 = 55,
    Ab2 = 56,
    A2 = 57,
    Bb2 = 58,
    B2 = 59,
    /// Middle C.
    C3 = 60,
    Db3 = 61,
    D3 = 62,
    Eb3 = 63,
    E3 = 64,
    F3 = 65,
    Gb3 = 66,
    G3 = 67,
    Ab3 = 68,
    /// A440.
    A3 = 69,
    Bb3 = 70,
    B3 = 71,
    C4 = 72,
    Db4 = 73,
    D4 = 74,
    Eb4 = 75,
    E4 = 76,
    F4 = 77,
    Gb4 = 78,
    G4 = 79,
    Ab4 = 80,
    A4 = 81,
    Bb4 = 82,
    B4 = 83,
    C5 = 84,
    Db5 = 85,
    D5 = 86,
    Eb5 = 87,
    E5 = 88,
    F5 = 89,
    Gb5 = 90,
    G5 = 91,
    Ab5 = 92,
    A5 = 93,
    Bb5 = 94,
    B5 = 95,
    C6 = 96,
    Db6 = 97,
    D6 = 98,
    Eb6 = 99,
    E6 = 100,
    F6 = 101,
    Gb6 = 102,
    G6 = 103,
    Ab6 = 104,
    A6 = 105,
    Bb6 = 106,
    B6 = 107,
    C7 = 108,
    Db7 = 109,
    D7 = 110,
    Eb7 = 111,
    E7 = 112,
    F7 = 113,
    Gb7 = 114,
    G7 = 115,
    Ab7 = 116,
    A7 = 117,
    Bb7 = 118,
    B7 = 119,
    C8 = 120,
    Db8 = 121,
    D8 = 122,
    Eb8 = 123,
    E8 = 124,
    F8 = 125,
    Gb8 = 126,
    G8 = 127,
}

#[allow(non_upper_case_globals)]
impl Note {
    pub const CSharpMinus2: Note = Note::DbMinus2;
    pub const DSharpMinus2: Note = Note::EbMinus2;
    pub const FSharpMinus2: Note = Note::GbMinus2;
    pub const GSharpMinus2: Note = Note::AbMinus2;
    pub const ASharpMinus2: Note = Note::BbMinus2;
    pub const CSharpMinus1: Note = Note::DbMinus1;
    pub const DSharMinus1: Note = Note::EbMinus1;
    pub const FSharpMinus1: Note = Note::GbMinus1;
    pub const GSharpMinus1: Note = Note::AbMinus1;
    pub const ASharpMinus1: Note = Note::BbMinus1;
    pub const CSharp0: Note = Note::Db0;
    pub const DSharp0: Note = Note::Eb0;
    pub const FSharp0: Note = Note::Gb0;
    pub const GSharp0: Note = Note::Ab0;
    pub const ASharp0: Note = Note::Bb0;
    pub const CSharp1: Note = Note::Db1;
    pub const DSharp1: Note = Note::Eb1;
    pub const FSharp1: Note = Note::Gb1;
    pub const GSharp1: Note = Note::Ab1;
    pub const ASharp1: Note = Note::Bb1;
    pub const CSharp2: Note = Note::Db2;
    pub const DSharp2: Note = Note::Eb2;
    pub const FSharp2: Note = Note::Gb2;
    pub const GSharp2: Note = Note::Ab2;
    pub const ASharp2: Note = Note::Bb2;
    pub const CSharp3: Note = Note::Db3;
    pub const DSharp3: Note = Note::Eb3;
    pub const FSharp3: Note = Note::Gb3;
    pub const GSharp3: Note = Note::Ab3;
    pub const ASharp3: Note = Note::Bb3;
    pub const CSharp4: Note = Note::Db4;
    pub const DSharp4: Note = Note::Eb4;
    pub const FSharp4: Note = Note::Gb4;
    pub const GSharp4: Note = Note::Ab4;
    pub const ASharp4: Note = Note::Bb4;
    pub const CSharp5: Note = Note::Db5;
    pub const DSharp5: Note = Note::Eb5;
    pub const FSharp5: Note = Note::Gb5;
    pub const GSharp5: Note = Note::Ab5;
    pub const ASharp5: Note = Note::Bb5;
    pub const CSharp6: Note = Note::Db6;
    pub const DSharp6: Note = Note::Eb6;
    pub const FSharp6: Note = Note::Gb6;
    pub const GSharp6: Note = Note::Ab6;
    pub const ASharp6: Note = Note::Bb6;
    pub const CSharp7: Note = Note::Db7;
    pub const DSharp7: Note = Note::Eb7;
    pub const FSharp7: Note = Note::Gb7;
    pub const GSharp7: Note = Note::Ab7;
    pub const ASharp7: Note = Note::Bb7;
    pub const CSharp8: Note = Note::Db8;
    pub const DSharp8: Note = Note::Eb8;
    pub const FSharp8: Note = Note::Gb8;

    /// The lowest representable note.
    pub const LOWEST_NOTE: Note = Note::CMinus2;

    /// The highest representable note.
    pub const HIGHEST_NOTE: Note = Note::G8;

    /// Creates a note from a `u8`. `note` must be between [0, 127] inclusive to create a valid
    /// note.
    ///
    /// # Example
    ///```
    /// let parsed_note = 60;
    /// let note = unsafe { wmidi::Note::from_u8_unchecked(parsed_note) };
    ///```
    #[inline(always)]
    pub unsafe fn from_u8_unchecked(note: u8) -> Note {
        core::mem::transmute(note)
    }

    /// The frequency using the standard 440Hz tuning.
    ///
    /// # Example
    /// ```
    /// # fn sing(frequency: f32) {}
    /// let note = wmidi::Note::A3;
    /// sing(note.to_freq_f32());
    /// ```
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn to_freq_f32(self) -> f32 {
        let exp = (f32::from(self as u8) + 36.376_316) / 12.0;
        2f32.powf(exp)
    }

    /// The frequency using the standard 440Hz tuning.
    ///
    /// # Example
    /// ```
    /// # fn sing(frequency: f64) {}
    /// let note = wmidi::Note::A3;
    /// sing(note.to_freq_f64());
    /// ```
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn to_freq_f64(self) -> f64 {
        let exp = (f64::from(self as u8) + 36.376_316_562_295_91) / 12.0;
        2f64.powf(exp)
    }

    /// Get the note relative to `self`.
    ///
    /// # Example
    /// ```
    /// use wmidi::Note;
    /// fn minor_chord(root: Note) -> Result<[Note; 3], wmidi::Error> {
    ///     Ok([root, root.step(3)?, root.step(7)?])
    /// }
    /// assert_eq!(minor_chord(Note::C2), Ok([Note::C2, Note::Eb2, Note::G2]));
    /// ```
    pub fn step(self, half_steps: i8) -> Result<Note, Error> {
        let half_steps: i16 = half_steps.into();
        let raw_note = self as i16 + half_steps;
        if Note::LOWEST_NOTE as i16 <= raw_note && raw_note <= Note::HIGHEST_NOTE as i16 {
            Ok(unsafe { Note::from_u8_unchecked(raw_note as u8) })
        } else {
            Err(Error::NoteOutOfRange)
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Note::CMinus2 => "C-2",
            Note::DbMinus2 => "C#/Db-2",
            Note::DMinus2 => "D-2",
            Note::EbMinus2 => "D#/Eb-2",
            Note::EMinus2 => "E-2",
            Note::FMinus2 => "F-2",
            Note::GbMinus2 => "F#/Gb-2",
            Note::GMinus2 => "G-2",
            Note::AbMinus2 => "G#/Ab-2",
            Note::AMinus2 => "A-2",
            Note::BbMinus2 => "A#/Bb-2",
            Note::BMinus2 => "B-2",
            Note::CMinus1 => "C-1",
            Note::DbMinus1 => "C#/Db-1",
            Note::DMinus1 => "D-1",
            Note::EbMinus1 => "D#/Eb-1",
            Note::EMinus1 => "E-1",
            Note::FMinus1 => "F-1",
            Note::GbMinus1 => "F#/Gb-1",
            Note::GMinus1 => "G-1",
            Note::AbMinus1 => "G#/Ab-1",
            Note::AMinus1 => "A-1",
            Note::BbMinus1 => "A#/Bb-1",
            Note::BMinus1 => "B-1",
            Note::C0 => "C0",
            Note::Db0 => "C#/Db0",
            Note::D0 => "D0",
            Note::Eb0 => "D#/Eb0",
            Note::E0 => "E0",
            Note::F0 => "F0",
            Note::Gb0 => "F#/Gb0",
            Note::G0 => "G0",
            Note::Ab0 => "G#/Ab0",
            Note::A0 => "A0",
            Note::Bb0 => "A#/Bb0",
            Note::B0 => "B0",
            Note::C1 => "C1",
            Note::Db1 => "C#/Db1",
            Note::D1 => "D1",
            Note::Eb1 => "D#/Eb1",
            Note::E1 => "E1",
            Note::F1 => "F1",
            Note::Gb1 => "F#/Gb1",
            Note::G1 => "G1",
            Note::Ab1 => "G#/Ab1",
            Note::A1 => "A1",
            Note::Bb1 => "A#/Bb1",
            Note::B1 => "B1",
            Note::C2 => "C2",
            Note::Db2 => "C#/Db2",
            Note::D2 => "D2",
            Note::Eb2 => "D#/Eb2",
            Note::E2 => "E2",
            Note::F2 => "F2",
            Note::Gb2 => "F#/Gb2",
            Note::G2 => "G2",
            Note::Ab2 => "G#/Ab2",
            Note::A2 => "A2",
            Note::Bb2 => "A#/Bb2",
            Note::B2 => "B2",
            Note::C3 => "C3",
            Note::Db3 => "C#/Db3",
            Note::D3 => "D3",
            Note::Eb3 => "D#/Eb3",
            Note::E3 => "E3",
            Note::F3 => "F3",
            Note::Gb3 => "F#/Gb3",
            Note::G3 => "G3",
            Note::Ab3 => "G#/Ab3",
            Note::A3 => "A3",
            Note::Bb3 => "A#/Bb3",
            Note::B3 => "B3",
            Note::C4 => "C4",
            Note::Db4 => "C#/Db4",
            Note::D4 => "D4",
            Note::Eb4 => "D#/Eb4",
            Note::E4 => "E4",
            Note::F4 => "F4",
            Note::Gb4 => "F#/Gb4",
            Note::G4 => "G4",
            Note::Ab4 => "G#/Ab4",
            Note::A4 => "A4",
            Note::Bb4 => "A#/Bb4",
            Note::B4 => "B4",
            Note::C5 => "C5",
            Note::Db5 => "C#/Db5",
            Note::D5 => "D5",
            Note::Eb5 => "D#/Eb5",
            Note::E5 => "E5",
            Note::F5 => "F5",
            Note::Gb5 => "F#/Gb5",
            Note::G5 => "G5",
            Note::Ab5 => "G#/Ab5",
            Note::A5 => "A5",
            Note::Bb5 => "A#/Bb5",
            Note::B5 => "B5",
            Note::C6 => "C6",
            Note::Db6 => "C#/Db6",
            Note::D6 => "D6",
            Note::Eb6 => "D#/Eb6",
            Note::E6 => "E6",
            Note::F6 => "F6",
            Note::Gb6 => "F#/Gb6",
            Note::G6 => "G6",
            Note::Ab6 => "G#/Ab6",
            Note::A6 => "A6",
            Note::Bb6 => "A#/Bb6",
            Note::B6 => "B6",
            Note::C7 => "C7",
            Note::Db7 => "C#/Db7",
            Note::D7 => "D7",
            Note::Eb7 => "D#/Eb7",
            Note::E7 => "E7",
            Note::F7 => "F7",
            Note::Gb7 => "F#/Gb7",
            Note::G7 => "G7",
            Note::Ab7 => "G#/Ab7",
            Note::A7 => "A7",
            Note::Bb7 => "A#/Bb7",
            Note::B7 => "B7",
            Note::C8 => "C8",
            Note::Db8 => "C#/Db8",
            Note::D8 => "D8",
            Note::Eb8 => "D#/Eb8",
            Note::E8 => "E8",
            Note::F8 => "F8",
            Note::Gb8 => "F#/Gb8",
            Note::G8 => "G8",
        }
    }
}

/// Convert from a `u8` to a `Note`. The `u8` must be in the range [0, 127] inclusive.
impl TryFrom<u8> for Note {
    type Error = Error;
    /// Creates a note from a `u8`. `note` must be between [0, 127] inclusive to create a valid
    /// note.
    ///
    /// # Example
    ///```
    /// use std::convert::TryFrom;
    /// fn decode_note(number: u8) -> Result<wmidi::Note, wmidi::Error> {
    ///     let parsed_note = 60;
    ///     let note = wmidi::Note::try_from(parsed_note)?;
    ///     Ok(note)
    /// }
    ///```
    #[inline(always)]
    fn try_from(note: u8) -> Result<Note, Error> {
        if note > 127 {
            Err(Error::NoteOutOfRange)
        } else {
            Ok(unsafe { Note::from_u8_unchecked(note) })
        }
    }
}

impl From<crate::U7> for Note {
    #[inline(always)]
    fn from(note: crate::U7) -> Note {
        unsafe { Note::from_u8_unchecked(u8::from(note)) }
    }
}

/// Convert from a `Note` to a `u8`.
impl From<Note> for u8 {
    /// # Example
    ///```
    /// use std::convert::TryFrom;
    /// fn encode_note(note: wmidi::Note) -> u8 {
    ///     u8::from(note)
    /// }
    ///```
    #[inline(always)]
    fn from(note: Note) -> u8 {
        note as u8
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.to_str(), *self as u8)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn note_to_frequency() {
        let a440_f64 = Note::A3.to_freq_f64();
        assert!((a440_f64 - 440.0).abs() < 1E-10, "{} != 440", a440_f64);

        let a440_f32 = Note::A3.to_freq_f32();
        assert!((a440_f32 - 440.0).abs() < 1E-10, "{} != 440", a440_f32);
    }

    #[test]
    fn step() {
        assert_eq!(Note::CMinus2.step(12), Ok(Note::CMinus1));
        assert_eq!(Note::CMinus1.step(-12), Ok(Note::CMinus2));
        assert_eq!(Note::B3.step(1), Ok(Note::C4));
        assert_eq!(Note::B3.step(100), Err(Error::NoteOutOfRange));
        assert_eq!(Note::B3.step(-100), Err(Error::NoteOutOfRange));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_debug() {
        let debug_str = format!("{:?}", Note::Bb3);
        assert!(debug_str.contains("Bb"), debug_str);
        assert!(debug_str.contains('3'), debug_str);
        assert!(debug_str.contains("A#"), debug_str);
    }
}
