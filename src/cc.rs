//! Documents referred to in this module:
//! * [MIDI 1.0]: The Complete MIDI 1.0 Detailed Specification, Third Edition (1996)
//! * [GM1]: General MIDI System Level 1
//! * [GM2]: General MIDI 2, version 1.2a
//! * [RP-015]: Recommended Practice (RP-015): Response to Reset All Controllers
//! * [RP-018]: Recommended Practice (RP-018): Response to Data Inc/Dec Controllers
//! * [RP-021]: Recommended Practice (RP-021): Sound Controller Defaults (Revised)
//! * [RP-023]: Recommended Practice (RP-023): Renaming of CC91 and CC93 
//! * [CA-026]: CC #88 High Resolution Velocity Prefix (CA-031)
//! * [CA-031]: RPN05 Modulation Depth Range

use crate::byte::U7;

/// A Control Change signal. The names of each variant of the enum
/// are from the 1997 MIDI 1.0 specification. The names and description
/// reflect the standard assignment and behavior for each CC number,
/// though manufacturers may ignore some controllers or use non-standard
/// mappings. MIDI devices should provide a controller allocation table
/// as part of their user manual.
/// 
/// * 0 - 31: Continuous Controller Data (MSB)
/// * 32 - 63: Continuous Controller Data (LSB)
/// * 64 - 119: Single-byte controllers
/// * 120 - 127: Channel mode messages
/// 
/// Setting the MSB on a continous controller resets the LSB to zero.
/// The LSB can be omitted afterwards if the finer resolution is not needed.
/// 
/// Channel mode messages affect the entire instrument and
/// are only valid when sent over the instrument's "basic channel".
#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ControlFunction {
    /// [MIDI 1.0] Specifies which program bank to use when changing program.
    /// The MSB and LSB are always sent as a pair, immediately followed
    /// by a program change.
    BankSelect = 0,
    /// Either known as Molulation Wheel (MIDI 1.0 and GM1) or Modulation Depth (GM2)
    ///
    /// [GM1] "For all instruments, the Modulation Wheel will change the nature of the sound
    /// in the most natural (expected) way. i.e. depth of LFO; change of timbre;
    /// add more tine sound; etc.)"
    /// 
    /// [GM2] Specifies the vibrato (LFO pitch modulation) depth to use for that channel.
    /// The depth response ranges from 0 cents (no modulation) to the value of 
    /// RPN 05.00 "Modulation Depth Range", following a curve that is linear in cents.
    ModulationWheel = 1,
    /// *Effect was never standardized*
    BreathController = 2,
    Undefined3 = 3,
    /// *Effect was never standardized*
    FootController = 4,
    /// [MIDI 1.0] Specifies the pitch increment speed for the portamento effect.
    /// The relation between the parameter value and the speed in cents per ms
    /// is outside the MIDI specification.
    PortamentoTime = 5,
    /// [MIDI 1.0] Sets the value of the last selected RPN/NRPN.
    DataEntryMSB = 6,
    /// [MIDI 1.0] Specifies the mixing volume for that channel.
    /// 
    /// [GM1] The gain in dB should be equivalent to `L = 40 * log10(cc7/127)`.
    ChannelVolume = 7,
    /// [MIDI 1.0] Specifies the volume balance for that channel.
    /// Center is 64, left/bottom is 0 and right/top is 127.
    Balance = 8,
    Undefined9 = 9,
    /// [MIDI 1.0] Specifies the sound location (in stereo) balance for that channel.
    /// Center is 64, left is 0 and right is 127.
    Pan = 10,
    /// [MIDI 1.0] Specifies the volume accent for that channel.
    /// Unlike ChannelVolume (7), this one is meant to change during performance
    /// to create diminuendos and crescendos.
    /// 
    /// [GM1] The gain in dB should be equivalent to
    /// `L = 40 * log10(cc7/127) + 40 * log10(cc11/127)`.
    ExpressionController = 11,
    EffectControl1 = 12,
    EffectControl2 = 13,
    Undefined14 = 14,
    Undefined15 = 15,
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    GeneralPurposeController1 = 16,
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    GeneralPurposeController2 = 17,
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    GeneralPurposeController3 = 18,
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    GeneralPurposeController4 = 19,
    Undefined20 = 20,
    Undefined21 = 21,
    Undefined22 = 22,
    Undefined23 = 23,
    Undefined24 = 24,
    Undefined25 = 25,
    Undefined26 = 26,
    Undefined27 = 27,
    Undefined28 = 28,
    Undefined29 = 29,
    Undefined30 = 30,
    Undefined31 = 31,

    BankSelectLSB = 32,
    ModulationWheelLSB = 33,
    BreathControllerLSB = 34,
    Undefined3LSB = 35,
    FootControllerLSB = 36,
    PortamentoTimeLSB = 37,
    /// [MIDI 1.0] Sets the value of the last selected RPN/NRPN
    DataEntryLSB = 38,
    ChannelVolumeLSB = 39,
    BalanceLSB = 40,
    Undefined9LSB = 41,
    PanLSB = 42,
    ExpressionControllerLSB = 43,
    EffectControl1LSB = 44,
    EffectControl2LSB = 45,
    Undefined14LSB = 46,
    Undefined15LSB = 47,
    GeneralPurposeController1LSB = 48,
    GeneralPurposeController2LSB = 49,
    GeneralPurposeController3LSB = 50,
    GeneralPurposeController4LSB = 51,
    Undefined20LSB = 52,
    Undefined21LSB = 53,
    Undefined22LSB = 54,
    Undefined23LSB = 55,
    Undefined24LSB = 56,
    Undefined25LSB = 57,
    Undefined26LSB = 58,
    Undefined27LSB = 59,
    Undefined28LSB = 60,
    Undefined29LSB = 61,
    Undefined30LSB = 62,
    Undefined31LSB = 63,

    /// Either known as the Hold, Sustain or Damper pedal.
    /// 
    /// [MIDI 1.0] Response to NoteOff and AllNotesOff should be delayed while
    /// this switch is on (value >= 64) until it transitions to off.
    /// 
    /// [GM2] May be treated as a continuous controller instead of a switch
    /// for the "Half Damper" and "re-damper" effects.
    DamperPedal = 64,
    /// [MIDI 1.0] Turns the Portamento effect on (value >= 64) or off.
    PortamentoOnOff = 65,
    /// [MIDI 1.0] Same as DamperPedal (64), but only affects the notes being held
    /// **while** the switch transitions to on (value >= 64). Any note played
    /// while the switch is already on behaves as normal.
    Sostenuto = 66,
    /// [GM2] Notes played while this switch is on (value >= 64) should be
    /// played at a reduced volume.
    SoftPedal = 67,
    /// [MIDI 1.0] This switch turns on (value >= 64) the monophonic legato response
    /// mode for that channel, where recieving a NoteOn while a note is already
    /// playing will change the pitch of the current note accordingly (without
    /// replaying the attack or re-attacking the enveloppes).
    LegatoFootswitch = 68,
    /// [MIDI 1.0] Additionnal controller for hold functions that don't match
    /// the specified definition of DamperPedal (64).
    Hold2 = 69,
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Sound Variation"
    SoundController1 = 70,
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Timbre/Harmonic Intensity"
    /// 
    /// [GM2] Sets the strength of the resonance effect for filter(s) for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController2 = 71,
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Release Time"
    /// 
    /// [GM2] Controls the release time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController3 = 72,
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Attack Time"
    /// 
    /// [GM2] Controls the attack time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController4 = 73,
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Brightness"
    /// 
    /// [GM2] Controls the preset cut-off frequency of the filter.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController5 = 74,
    /// [RP-021] Remappable Sound Controller, Default Name: "Decay Time"
    /// 
    /// [GM2] Controls the decay time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController6 = 75,
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Rate"
    /// 
    /// [GM2] Controls the vibrato rate on the specified Channel relative to the sound's preset rate.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController7 = 76,
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Depth"
    /// 
    /// [GM2] Controls the vibrato depth for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController8 = 77,
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Delay"
    /// 
    /// [GM2] Controls the vibrato delay on the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    SoundController9 = 78,
    /// [MIDI 1.0] Remappable Sound Controller, no default
    SoundController10 = 79,
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    GeneralPurposeController5 = 80,
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    GeneralPurposeController6 = 81,
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    GeneralPurposeController7 = 82,
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    GeneralPurposeController8 = 83,
    /// [MIDI 1.0] Specifies the starting MIDI note from which the next NoteOn will slide.
    PortamentoControl = 84,
    Undefined85 = 85,
    Undefined86 = 86,
    Undefined87 = 87,
    /// [CA-031] High Resolution Velocity Prefix
    /// 
    /// If sent before a NoteOn message, the value of this controller
    /// acts as an LSB for the velocity of that note.
    Undefined88 = 88,
    Undefined89 = 89,
    Undefined90 = 90,
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "External Effects Depth")
    /// 
    /// [RP-023] No-longer general-purpose, renamed to "Reverb Send Level"
    /// 
    /// [GM2] Specifies the Reverb Send Level for that channel, linearly from 0% to 100% of amplitude.
    Effects1Depth = 91,
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Tremolo Depth")
    Effects2Depth = 92,
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Chorus Depth")
    /// 
    /// [RP-023] No-longer general-purpose, renamed to "Chorus Send Level"
    /// 
    /// [GM2] Specifies the Chorus Send Level for that channel, linearly from 0% to 100% of amplitude.
    Effects3Depth = 93,
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Celeste (Detune) Depth")
    Effects4Depth = 94,
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Phaser Depth")
    Effects5Depth = 95,

    /// [RP-018] Increments the value of a RPN/NRPN by 1, regardless of the value byte.
    /// The actual behavior depends on the parameter.
    DataIncrement = 96,
    /// [RP-018] Decrements the value of a RPN/NRPN by 1, regardless of the value byte.
    /// The actual behavior depends on the parameter.
    DataDecrement = 97,
    /// [MIDI 1.0] Selects a parameter to be modified by DataIncrement (96),
    /// DataDecrement (97) and DataEntry (6 & 38). Unlike RPNs, NRPNs are
    /// manufacturer-specific.
    NonRegisteredParameterNumberLSB = 98,
    NonRegisteredParameterNumberMSB = 99,
    /// [MIDI 1.0] Selects a parameter to be modified by DataIncrement (96),
    /// DataDecrement (97) and DataEntry (6 & 38).
    /// 
    /// ###### [MIDI 1.0] `MSB=00` `LSB=00`: Pitch Bend Sensitivity
    /// Specifies the range of the pitch bender (both up and down),
    /// with the MSB value being in semitones and the LSB in cents.
    /// 
    /// [RP-018] Incrementing/decrementing this parameter changes the LSB,
    /// which wraps into the MSB at 100 cents to change it by one semitone.
    /// 
    /// ###### [MIDI 1.0] `MSB=00` `LSB=01`: (Channel) Fine Tuning
    /// Specifies the note displacement from A440 in 8192ths of 100 cents,
    /// with `0x40 0x00` acting as zero, `0x00 0x00` as -8192 (-100 cents)
    /// and `0x7F 0x7F` as +8191 (almost +100 cents).
    /// 
    /// [RP-018] Incrementing/decrementing this parameter changes the LSB by 1.
    /// 
    /// ###### [MIDI 1.0] `MSB=00` `LSB=02`: (Channel) Coarse Tuning
    /// Specifies the note displacement from A440 in increments of 100 cents
    /// for the MSB, (the LSB is ignored), with `0x40` acting as zero,
    /// `0x00` as -64 (-64 semitones) and `0x7F` as +63 (+63 semitones).
    /// 
    /// [RP-018] Incrementing/decrementing this parameter changes the MSB by 1.
    /// 
    /// ###### [CA-026] `MSB=00` `LSB=05`: Modulation Depth Range
    /// [GM2] Specifies the peak value of ModulationWheel (1), with the MSB
    /// value being in semitones and the LSB being in 128ths of 100 cents.
    /// 
    /// [CA-026] Incrementing/decrementing this parameter changes the LSB by 1.
    /// 
    /// ###### [GM2] `MSB=7F` `LSB=7F`: RPN NULL
    /// This RPN is invalid, data entry will be ignored while this parameter
    /// is selected.
    RegisteredParameterNumberLSB = 100,
    RegisteredParameterNumberMSB = 101,

    Undefined102 = 102,
    Undefined103 = 103,
    Undefined104 = 104,
    Undefined105 = 105,
    Undefined106 = 106,
    Undefined107 = 107,
    Undefined108 = 108,
    Undefined109 = 109,
    Undefined110 = 110,
    Undefined111 = 111,
    Undefined112 = 112,
    Undefined113 = 113,
    Undefined114 = 114,
    Undefined115 = 115,
    Undefined116 = 116,
    Undefined117 = 117,
    Undefined118 = 118,
    Undefined119 = 119,

    /// [MIDI 1.0] Indicates that the reciever should immediately silence (without
    /// going through the release phase and ignoring sustain) all notes currently
    /// sounding on that channel. May also be used to turn off lights.
    AllSoundOff = 120,
    /// [MIDI 1.0] Indicates that all controllers (including pitch bend and
    /// pressure) should be reset to an ideal initial state. This message
    /// is ignored if the device is in Omni mode (mode 1 or 2).
    /// 
    /// [RP-015] Indicates that the following controllers should be reset as such:
    /// specified channel:
    /// * Modulation (cc1): Set to 0
    /// * Expression (cc11): Set to 127
    /// * DamperPedal (cc64): Set to 0
    /// * PortamentoOnOff (cc65): Set to 0
    /// * Sostenuto (cc66): Set to 0
    /// * SoftPedal (cc67): Set to 0
    /// * NRPN (cc98 & cc99): Set to NULL NRPN (`0x7F 0x7F`)
    /// * RPN (cc100 & cc101): Set to NULL RPN (`0x7F 0x7F`)
    /// 
    /// Also reset the following for the specified channel:
    /// * Reset pitch-bend to center (`0x40 0x00`)
    /// * Reset channel pressure to 0
    /// * Reset polyphonic pressure of all notes to 0
    ResetAllControllers = 121,
    /// [MIDI 1.0] Specifies whether the instrument should react to notes being
    /// physically played on it (0 = off, 127 = on), as opposed to notes
    /// sent via MIDI-in.
    LocalControl = 122,
    /// [MIDI 1.0] Ignored in Omni mode (mode 1 & 2). In Poly operation (mode 3),
    /// acts as a NoteOff for all notes playing on the instrument's basic channel
    /// (it is ignored for all other channels). In Mono operation (mode 4),
    /// acts as a NoteOff for all notes playing on the specified channel. In all cases,
    /// notes being played on the instrument itself should remain unaffected.
    /// 
    /// [GM2] Turns off all Notes sounding on the specified Channel.
    AllNotesOff = 123,
    /// [MIDI 1.0] Same as AllNotesOff (123), then set reciever to mode 1 (omni-on, poly)
    /// or 2 (omni-on, mono) based on the current mode.
    /// 
    /// [GM2] Same as AllNotesOff (123), since GM2 does not support Omni mode.
    OmniModeOn = 124,
    /// [MIDI 1.0] Same as AllNotesOff (123), then set reciever to mode 3 (omni-off, poly)
    /// or 4 (omni-off, mono) based on the current mode.
    /// 
    /// [GM2] Same as AllNotesOff (123), since GM2 does not support Omni mode.
    OmniModeOff = 125,
    /// [MIDI 1.0] Same as AllNotesOff (123), then set reciever to mode 2 (omni-on, mono)
    /// or 4 (omni-off, mono) based on the current mode. The value byte indicates how many
    /// channels to use, with 0 being "auto".
    /// 
    /// [GM2] Same as AllNotesOff (123), then set the **channel** to mode 4. Will be
    /// ignored if the value byte is not equal to 1 of if the channel is a rhythm channel.
    MonoOperation = 126,
    /// [MIDI 1.0] Same as AllNotesOff (123), then set reciever to mode 1 (omni-on, poly)
    /// or 3 (omni-off, poly) based on the current mode.
    /// 
    /// [GM2] Same as AllNotesOff (123), then set the **channel** to mode 3.
    PolyOperation = 127,
}

impl From<U7> for ControlFunction {
    fn from(data: U7) -> ControlFunction {
        unsafe { std::mem::transmute(data) }
    }
}

impl From<ControlFunction> for U7 {
    fn from(control_function: ControlFunction) -> U7 {
        let data = control_function as u8;
        unsafe { U7::from_unchecked(data) }
    }
}

impl From<ControlFunction> for u8 {
    fn from(control_function: ControlFunction) -> u8 {
        control_function as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::U7;
    use std::convert::TryFrom;

    #[test]
    fn from_u7() {
        for value in 0..128 {
            let data = U7::try_from(value).unwrap();
            let cc = ControlFunction::from(data);
            assert_eq!(value, cc.into());
        }
    }
}
