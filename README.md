# WMIDI

Midi encoding and decoding library suitable for real-time execution.

[![crates.io](https://img.shields.io/crates/v/wmidi.svg)](https://crates.io/crates/wmidi)
[![docs.rs](https://docs.rs/wmidi/badge.svg)](https://docs.rs/wmidi)

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![travis-ci](https://api.travis-ci.org/wmedrano/wmidi.svg?branch=master)](https://travis-ci.org/wmedrano/wmidi)

# Usage

```rust
use std::convert::TryFrom;

// Decoding messages from bytes.
fn handle_midi_message(bytes: &[u8]) -> Result<(), wmidi::FromBytesError> {
    let message = wmidi::MidiMessage::try_from(bytes)?;
    if let wmidi::MidiMessage::NoteOn(_, note, _) = message {
        println!("Singing {}", note);
    }
    Ok(())
}

// Encoding messages to bytes.
fn midi_to_bytes(message: wmidi::MidiMessage<'_>) -> Vec<u8> {
    let mut bytes = vec![0u8; message.bytes_size()];
    message.copy_to_slice(bytes.as_mut_slice()).unwrap();
    bytes
}
```

# Changelog

## 3.1

* Rename `MidiMessage::wire_size()` to `MidiMessage::bytes_size()`.
* Introduce `MidiMessage::copy_to_slice()` method.

## 3.0

* Instances of U7 and U14 now have bounds checking.
* Note is now an enum instead of a u8. Can be converted with `Note::try_from` and `u8::from`.
