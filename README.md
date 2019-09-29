# WMIDI

Midi encoding and decoding library suitable for real-time execution.

[![crates.io](https://img.shields.io/crates/v/wmidi.svg)](https://crates.io/crates/wmidi)
[![docs.rs](https://docs.rs/wmidi/badge.svg)](https://docs.rs/wmidi)

# Changelog

## 3.0-unreleased

* Instances of U7 and U14 now have bounds checking.
* Note is now an enum instead of a u8. Can be converted with `Note::try_from` and `u8::from`.
