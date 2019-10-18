#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use std::convert::TryFrom;

const MESSAGES: [wmidi::MidiMessage<'static>; 19] = [
    wmidi::MidiMessage::NoteOn(wmidi::Channel::Ch1, wmidi::Note::C3, wmidi::U7::MAX),
    wmidi::MidiMessage::NoteOff(wmidi::Channel::Ch2, wmidi::Note::A3, wmidi::U7::MIN),
    wmidi::MidiMessage::PolyphonicKeyPressure(wmidi::Channel::Ch3, wmidi::Note::B1, wmidi::U7::MAX),
    wmidi::MidiMessage::ControlChange(wmidi::Channel::Ch4, wmidi::U7::MIN, wmidi::U7::MAX),
    wmidi::MidiMessage::ProgramChange(wmidi::Channel::Ch5, wmidi::U7::MIN),
    wmidi::MidiMessage::ChannelPressure(wmidi::Channel::Ch6, wmidi::U7::MAX),
    wmidi::MidiMessage::PitchBendChange(wmidi::Channel::Ch7, wmidi::U14::MAX),
    wmidi::MidiMessage::Start,
    wmidi::MidiMessage::SysEx(&[wmidi::U7::MIN, wmidi::U7::MAX]),
    wmidi::MidiMessage::MidiTimeCode(wmidi::U7::MAX),
    wmidi::MidiMessage::SongPositionPointer(wmidi::U14::MIN),
    wmidi::MidiMessage::SongSelect(wmidi::U7::MIN),
    wmidi::MidiMessage::TuneRequest,
    wmidi::MidiMessage::TimingClock,
    wmidi::MidiMessage::Start,
    wmidi::MidiMessage::Continue,
    wmidi::MidiMessage::Stop,
    wmidi::MidiMessage::ActiveSensing,
    wmidi::MidiMessage::Reset,
];

fn bench_to_slice(c: &mut Criterion) {
    c.bench_function("MidiMessage::copy_to_slice", |b| {
        let message = black_box(wmidi::MidiMessage::NoteOn(
            wmidi::Channel::Ch1,
            wmidi::Note::C3,
            wmidi::U7::MAX,
        ));
        b.iter(|| {
            let mut slice = [0u8; 3];
            message.copy_to_slice(&mut slice).unwrap();
            slice
        })
    });
    c.bench_function("MidiMessage::bytes_size sum", |b| {
        let messages = black_box(MESSAGES.clone());
        b.iter(|| messages.iter().map(|b| b.bytes_size()).sum::<usize>())
    });
    c.bench_function("MidiMessage::copy_to_slice many", |b| {
        let messages = black_box(MESSAGES.clone());
        let mut buffer = vec![0u8; messages.iter().map(|b| b.bytes_size()).sum()];
        b.iter(|| {
            let mut start = 0;
            for message in messages.iter() {
                let end = start
                    + message
                        .copy_to_slice(&mut buffer.as_mut_slice()[start..])
                        .unwrap();
                start = end;
            }
        })
    });
}

fn bench_from_bytes(c: &mut Criterion) {
    let bytes = {
        let mut bytes = vec![0u8; MESSAGES.iter().map(|m| m.bytes_size()).sum()];
        let mut start = 0;
        for message in MESSAGES.iter() {
            let end = start
                + message
                    .copy_to_slice(&mut bytes.as_mut_slice()[start..])
                    .unwrap();
            start = end;
        }
        bytes
    };
    c.bench_function("MidiMessage::try_from<u8>", |b| {
        let bytes = black_box(bytes.clone());
        b.iter(|| wmidi::MidiMessage::try_from(bytes.as_slice()).unwrap());
    });
    c.bench_function("MidiMessage::try_from<u8> many", |b| {
        let bytes = black_box(bytes.clone());
        b.iter(|| {
            let mut messages: Vec<wmidi::MidiMessage> = Vec::with_capacity(MESSAGES.len());
            let mut start = 0;
            while start < bytes.len() {
                let message = wmidi::MidiMessage::try_from(&bytes[start..]).unwrap();
                start += message.bytes_size();
                messages.push(message);
            }
            messages
        });
    });
}

fn bench_notes(c: &mut Criterion) {
    let all_notes: Vec<wmidi::Note> = (0..128)
        .map(|n| wmidi::Note::try_from(n).unwrap())
        .collect();
    let all_notes = black_box(all_notes);
    c.bench_function("Note::try_from", |b| {
        b.iter(|| {
            let mut notes = [wmidi::Note::LOWEST_NOTE; 128];
            for (note_number, dst) in (0..128).zip(notes.iter_mut()) {
                let note_number = black_box(note_number);
                let note = wmidi::Note::try_from(note_number).unwrap();
                *dst = note;
            }
        })
    });
    c.bench_function("Note::from_u8_unchecked", |b| {
        b.iter(|| {
            let mut notes = [wmidi::Note::LOWEST_NOTE; 128];
            for (note_number, dst) in (0..128).zip(notes.iter_mut()) {
                let note_number = black_box(note_number);
                let note = unsafe { wmidi::Note::from_u8_unchecked(note_number) };
                *dst = note;
            }
        })
    });
    c.bench_function("Note::to_freq_f32", |b| {
        b.iter(|| {
            let mut freqs = [0f32; 128];
            for (note, dst) in all_notes.iter().zip(freqs.iter_mut()) {
                *dst = note.to_freq_f32();
            }
            freqs
        })
    });
    c.bench_function("Note::to_freq_f32 result cast into f64", |b| {
        b.iter(|| {
            let mut freqs = [0f64; 128];
            for (note, dst) in all_notes.iter().zip(freqs.iter_mut()) {
                *dst = f64::from(note.to_freq_f32());
            }
            freqs
        })
    });
    c.bench_function("Note::to_freq_f64", |b| {
        b.iter(|| {
            let mut freqs = [0f64; 128];
            for (note, dst) in all_notes.iter().zip(freqs.iter_mut()) {
                *dst = note.to_freq_f64();
            }
            freqs
        })
    });
}

criterion_group!(benchmarks, bench_to_slice, bench_from_bytes, bench_notes);

criterion_main!(benchmarks);
