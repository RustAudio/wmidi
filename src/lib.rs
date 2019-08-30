mod midi_message;
mod error;

pub use midi_message::{
    Channel, ControlNumber, ControlValue, MidiMessage, Note, PitchBend, ProgramNumber, Song,
    SongPosition, Velocity, U14, U7,
};
pub use error::Error;

/// The frequency for `note` using the standard 440Hz tuning.
#[inline(always)]
pub fn note_to_frequency_f32(note: Note) -> f32 {
    let exp = (f32::from(note) + 36.376_316) / 12.0;
    2f32.powf(exp)
}

/// The frequency for `note` using the standard 440Hz tuning.
#[inline(always)]
pub fn note_to_frequency_f64(note: Note) -> f64 {
    let exp = (f64::from(note) + 36.376_316_562_295_91) / 12.0;
    2f64.powf(exp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn note_to_frequency() {
        let a440_f64 = note_to_frequency_f64(69);
        assert!((a440_f64 - 440.0).abs() < 1E-10, "{} != 440", a440_f64);

        let a440_f32 = note_to_frequency_f32(69);
        assert!((a440_f32 - 440.0).abs() < 1E-10, "{} != 440", a440_f32);
    }
}
