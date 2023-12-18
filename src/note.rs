use crate::Error;
use core::convert::TryFrom;
use core::fmt;

/// A midi note.
///
/// The format for the enum is `$NOTE` `$MODIFIER?` `$OCTAVE`. Note can be a note from `A` to `G`.
/// Modifier can be `b` for flat or `Sharp` for sharp. Octave is the number. The octave `-1` is
/// represented as `Minus1`.
/// # Example
/// ```
/// use wmidi::Note;
/// let ab7_chord = [Note::AbMinus1, Note::C4, Note::Gb4]; // We omit the 5th for a jazzier sound
/// let dmaj_chord = [Note::D2, Note::FSharp3, Note::A3];
/// assert_eq!(u8::from(Note::C3), 48u8);
/// assert_eq!(Note::from_u8_lossy(48), Note::C3);
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Note {
    CMinus1 = 0,
    DbMinus1 = 1,
    DMinus1 = 2,
    EbMinus1 = 3,
    EMinus1 = 4,
    FMinus1 = 5,
    GbMinus1 = 6,
    GMinus1 = 7,
    AbMinus1 = 8,
    AMinus1 = 9,
    BbMinus1 = 10,
    BMinus1 = 11,
    C0 = 12,
    Db0 = 13,
    D0 = 14,
    Eb0 = 15,
    E0 = 16,
    F0 = 17,
    Gb0 = 18,
    G0 = 19,
    Ab0 = 20,
    A0 = 21,
    Bb0 = 22,
    B0 = 23,
    C1 = 24,
    Db1 = 25,
    D1 = 26,
    Eb1 = 27,
    E1 = 28,
    F1 = 29,
    Gb1 = 30,
    G1 = 31,
    Ab1 = 32,
    A1 = 33,
    Bb1 = 34,
    B1 = 35,
    C2 = 36,
    Db2 = 37,
    D2 = 38,
    Eb2 = 39,
    E2 = 40,
    F2 = 41,
    Gb2 = 42,
    G2 = 43,
    Ab2 = 44,
    A2 = 45,
    Bb2 = 46,
    B2 = 47,
    C3 = 48,
    Db3 = 49,
    D3 = 50,
    Eb3 = 51,
    E3 = 52,
    F3 = 53,
    Gb3 = 54,
    G3 = 55,
    Ab3 = 56,
    A3 = 57,
    Bb3 = 58,
    B3 = 59,
    /// Middle C.
    C4 = 60,
    Db4 = 61,
    D4 = 62,
    Eb4 = 63,
    E4 = 64,
    F4 = 65,
    Gb4 = 66,
    G4 = 67,
    Ab4 = 68,
    /// A440.
    A4 = 69,
    Bb4 = 70,
    B4 = 71,
    C5 = 72,
    Db5 = 73,
    D5 = 74,
    Eb5 = 75,
    E5 = 76,
    F5 = 77,
    Gb5 = 78,
    G5 = 79,
    Ab5 = 80,
    A5 = 81,
    Bb5 = 82,
    B5 = 83,
    C6 = 84,
    Db6 = 85,
    D6 = 86,
    Eb6 = 87,
    E6 = 88,
    F6 = 89,
    Gb6 = 90,
    G6 = 91,
    Ab6 = 92,
    A6 = 93,
    Bb6 = 94,
    B6 = 95,
    C7 = 96,
    Db7 = 97,
    D7 = 98,
    Eb7 = 99,
    E7 = 100,
    F7 = 101,
    Gb7 = 102,
    G7 = 103,
    Ab7 = 104,
    A7 = 105,
    Bb7 = 106,
    B7 = 107,
    C8 = 108,
    Db8 = 109,
    D8 = 110,
    Eb8 = 111,
    E8 = 112,
    F8 = 113,
    Gb8 = 114,
    G8 = 115,
    Ab8 = 116,
    A8 = 117,
    Bb8 = 118,
    B8 = 119,
    C9 = 120,
    Db9 = 121,
    D9 = 122,
    Eb9 = 123,
    E9 = 124,
    F9 = 125,
    Gb9 = 126,
    G9 = 127,
}

#[allow(non_upper_case_globals)]
impl Note {
    pub const CSharpMinus1: Note = Note::DbMinus1;
    pub const DSharpMinus1: Note = Note::EbMinus1;
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
    pub const GSharp8: Note = Note::Ab8;
    pub const ASharp8: Note = Note::Bb8;
    pub const CSharp9: Note = Note::Db9;
    pub const DSharp9: Note = Note::Eb9;
    pub const FSharp9: Note = Note::Gb9;

    /// The lowest representable note.
    pub const LOWEST_NOTE: Note = Note::CMinus1;

    /// The highest representable note.
    pub const HIGHEST_NOTE: Note = Note::G9;

    /// Creates a note from a `u8`. `note` must be between [0, 127] inclusive to create a valid
    /// note.
    ///
    /// # Example
    ///```
    /// let parsed_note = 60;
    /// let note = unsafe { wmidi::Note::from_u8_unchecked(parsed_note) };
    ///```
    ///
    /// # Safety
    /// `note` must be less than or equal to 127.
    #[inline(always)]
    pub unsafe fn from_u8_unchecked(note: u8) -> Note {
        core::mem::transmute(note)
    }

    /// Create a note from a `u8`. Only the 7 least significant bits of `note` are used.
    #[inline(always)]
    pub fn from_u8_lossy(note: u8) -> Note {
        Note::from(crate::U7::from_u8_lossy(note))
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
        2_f32.powf(exp)
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
        2_f64.powf(exp)
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

    /// Get a `str` representation of the note. For example: `"C3"` or `"A#/Bb2"`.
    pub fn to_str(self) -> &'static str {
        match self {
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
            Note::Ab8 => "G#/Ab8",
            Note::A8 => "A8",
            Note::Bb8 => "A#/Bb8",
            Note::B8 => "B8",
            Note::C9 => "C9",
            Note::Db9 => "C#/Db9",
            Note::D9 => "D9",
            Note::Eb9 => "D#/Eb9",
            Note::E9 => "E9",
            Note::F9 => "F9",
            Note::Gb9 => "F#/Gb9",
            Note::G9 => "G9",
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

    #[cfg(feature = "std")]
    #[test]
    fn note_to_frequency() {
        let a440_f64 = Note::A4.to_freq_f64();
        assert!((a440_f64 - 440.0).abs() < 1E-10, "{} != 440", a440_f64);

        let a440_f32 = Note::A4.to_freq_f32();
        assert!((a440_f32 - 440.0).abs() < 1E-10, "{} != 440", a440_f32);
    }

    #[test]
    fn step() {
        assert_eq!(Note::CMinus1.step(12), Ok(Note::C0));
        assert_eq!(Note::C0.step(-12), Ok(Note::CMinus1));
        assert_eq!(Note::B3.step(1), Ok(Note::C4));
        assert_eq!(Note::B3.step(100), Err(Error::NoteOutOfRange));
        assert_eq!(Note::B3.step(-100), Err(Error::NoteOutOfRange));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_debug() {
        let debug_str = format!("{:?}", Note::Bb3);
        assert!(debug_str.contains("Bb"), "{}", debug_str);
        assert!(debug_str.contains('3'), "{}", debug_str);
        assert!(debug_str.contains("A#"), "{}", debug_str);
    }
}
