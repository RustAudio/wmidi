#![no_std]
#![cfg(feature = "std")]
#[macro_use]
extern crate std;

mod byte;
mod cc;
mod error;
mod midi_message;
mod note;

pub use byte::{U14, U7};
pub use cc::ControlFunction;
pub use error::{FromBytesError, ToSliceError};
pub use midi_message::{
    Channel, ControlValue, MidiMessage, PitchBend, ProgramNumber, Song, SongPosition, Velocity,
};
pub use note::Note;

/// Use `FromBytesError` instead.
pub type Error = FromBytesError;

/// The frequency for `note` using the standard 440Hz tuning.
#[cfg(feature = "std")]
#[inline(always)]
#[deprecated(since = "3.0.0", note = "Use note.to_freq_f32() instead.")]
pub fn note_to_frequency_f32(note: Note) -> f32 {
    note.to_freq_f32()
}

/// The frequency for `note` using the standard 440Hz tuning.
#[cfg(feature = "std")]
#[inline(always)]
#[deprecated(since = "3.0.0", note = "Use note.to_freq_f64() instead.")]
pub fn note_to_frequency_f64(note: Note) -> f64 {
    note.to_freq_f64()
}
