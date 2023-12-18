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

/// A Control Change signal. The names of each variant of the constants
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
/// Setting the MSB on a continuous controller resets the LSB to zero.
/// The LSB can be omitted afterwards if the finer resolution is not needed.
///
/// Channel mode messages affect the entire instrument and
/// are only valid when sent over the instrument's "basic channel".
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ControlFunction(pub U7);

impl ControlFunction {
    pub const MIN: ControlFunction = ControlFunction(U7::MIN);
    pub const MAX: ControlFunction = ControlFunction(U7::MAX);
    /// [MIDI 1.0] Specifies which program bank to use when changing program.
    /// The MSB and LSB are always sent as a pair, immediately followed
    /// by a program change.
    pub const BANK_SELECT: ControlFunction = ControlFunction(U7(0));

    /// Either known as Modulation Wheel (MIDI 1.0 and GM1) or Modulation Depth (GM2)
    ///
    /// [GM1] "For all instruments, the Modulation Wheel will change the nature of the sound
    /// in the most natural (expected) way. i.e. depth of LFO; change of timbre;
    /// add more tine sound; etc.)"
    ///
    /// [GM2] Specifies the vibrato (LFO pitch modulation) depth to use for that channel.
    /// The depth response ranges from 0 cents (no modulation) to the value of
    /// RPN 05.00 "Modulation Depth Range", following a curve that is linear in cents.
    pub const MODULATION_WHEEL: ControlFunction = ControlFunction(U7(1));
    /// *Effect was never standardized*
    pub const BREATH_CONTROLLER: ControlFunction = ControlFunction(U7(2));
    pub const UNDEFINED_3: ControlFunction = ControlFunction(U7(3));
    /// *Effect was never standardized*
    pub const FOOT_CONTROLLER: ControlFunction = ControlFunction(U7(4));
    /// [MIDI 1.0] Specifies the pitch increment speed for the portamento effect.
    /// The relation between the parameter value and the speed in cents per ms
    /// is outside the MIDI specification.
    pub const PORTAMENTO_TIME: ControlFunction = ControlFunction(U7(5));
    /// [MIDI 1.0] Sets the value of the last selected RPN/NRPN.
    pub const DATA_ENTRY_MSB: ControlFunction = ControlFunction(U7(6));
    /// [MIDI 1.0] Specifies the mixing volume for that channel.
    ///
    /// [GM1] The gain in dB should be equivalent to `L = 40 * log10(cc7/127)`.
    pub const CHANNEL_VOLUME: ControlFunction = ControlFunction(U7(7));
    /// [MIDI 1.0] Specifies the volume balance for that channel.
    /// Center is 64, left/bottom is 0 and right/top is 127.
    pub const BALANCE: ControlFunction = ControlFunction(U7(8));
    pub const UNDEFINED_9: ControlFunction = ControlFunction(U7(9));
    /// [MIDI 1.0] Specifies the sound location (in stereo) balance for that channel.
    /// Center is 64, left is 0 and right is 127.
    pub const PAN: ControlFunction = ControlFunction(U7(10));
    /// [MIDI 1.0] Specifies the volume accent for that channel.
    /// Unlike ChannelVolume (7), this one is meant to change during performance
    /// to create diminuendos and crescendos.
    ///
    /// [GM1] The gain in dB should be equivalent to
    /// `L = 40 * log10(cc7/127) + 40 * log10(cc11/127)`.
    pub const EXPRESSION_CONTROLLER: ControlFunction = ControlFunction(U7(11));
    pub const EFFECT_CONTROL_1: ControlFunction = ControlFunction(U7(12));
    pub const EFFECT_CONTROL_2: ControlFunction = ControlFunction(U7(13));
    pub const UNDEFINED_14: ControlFunction = ControlFunction(U7(14));
    pub const UNDEFINED_15: ControlFunction = ControlFunction(U7(15));
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_1: ControlFunction = ControlFunction(U7(16));
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_2: ControlFunction = ControlFunction(U7(17));
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_3: ControlFunction = ControlFunction(U7(18));
    /// [MIDI 1.0] 2 bytes general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_4: ControlFunction = ControlFunction(U7(19));
    pub const UNDEFINED_20: ControlFunction = ControlFunction(U7(20));
    pub const UNDEFINED_21: ControlFunction = ControlFunction(U7(21));
    pub const UNDEFINED_22: ControlFunction = ControlFunction(U7(22));
    pub const UNDEFINED_23: ControlFunction = ControlFunction(U7(23));
    pub const UNDEFINED_24: ControlFunction = ControlFunction(U7(24));
    pub const UNDEFINED_25: ControlFunction = ControlFunction(U7(25));
    pub const UNDEFINED_26: ControlFunction = ControlFunction(U7(26));
    pub const UNDEFINED_27: ControlFunction = ControlFunction(U7(27));
    pub const UNDEFINED_28: ControlFunction = ControlFunction(U7(28));
    pub const UNDEFINED_29: ControlFunction = ControlFunction(U7(29));
    pub const UNDEFINED_30: ControlFunction = ControlFunction(U7(30));
    pub const UNDEFINED_31: ControlFunction = ControlFunction(U7(31));

    pub const BANK_SELECT_LSB: ControlFunction = ControlFunction(U7(32));
    pub const MODULATION_WHEEL_LSB: ControlFunction = ControlFunction(U7(33));
    pub const BREATH_CONTROLLER_LSB: ControlFunction = ControlFunction(U7(34));
    pub const UNDEFINED_3_LSB: ControlFunction = ControlFunction(U7(35));
    pub const FOOT_CONTROLLER_LSB: ControlFunction = ControlFunction(U7(36));
    pub const PORTAMENTO_TIME_LSB: ControlFunction = ControlFunction(U7(37));
    /// [MIDI 1.0] Sets the value of the last selected RPN/NRPN
    pub const DATA_ENTRY_LSB: ControlFunction = ControlFunction(U7(38));
    pub const CHANNEL_VOLUME_LSB: ControlFunction = ControlFunction(U7(39));
    pub const BALANCE_LSB: ControlFunction = ControlFunction(U7(40));
    pub const UNDEFINED_9_LSB: ControlFunction = ControlFunction(U7(41));
    pub const PAN_LSB: ControlFunction = ControlFunction(U7(42));
    pub const EXPRESSION_CONTROLLER_LSB: ControlFunction = ControlFunction(U7(43));
    pub const EFFECT_CONTROL_1_LSB: ControlFunction = ControlFunction(U7(44));
    pub const EFFECT_CONTROL_2_LSB: ControlFunction = ControlFunction(U7(45));
    pub const UNDEFINED_14_LSB: ControlFunction = ControlFunction(U7(46));
    pub const UNDEFINED_15_LSB: ControlFunction = ControlFunction(U7(47));
    pub const GENERAL_PURPOSE_CONTROLLER_1_LSB: ControlFunction = ControlFunction(U7(48));
    pub const GENERAL_PURPOSE_CONTROLLER_2_LSB: ControlFunction = ControlFunction(U7(49));
    pub const GENERAL_PURPOSE_CONTROLLER_3_LSB: ControlFunction = ControlFunction(U7(50));
    pub const GENERAL_PURPOSE_CONTROLLER_4_LSB: ControlFunction = ControlFunction(U7(51));
    pub const UNDEFINED_20_LSB: ControlFunction = ControlFunction(U7(52));
    pub const UNDEFINED_21_LSB: ControlFunction = ControlFunction(U7(53));
    pub const UNDEFINED_22_LSB: ControlFunction = ControlFunction(U7(54));
    pub const UNDEFINED_23_LSB: ControlFunction = ControlFunction(U7(55));
    pub const UNDEFINED_24_LSB: ControlFunction = ControlFunction(U7(56));
    pub const UNDEFINED_25_LSB: ControlFunction = ControlFunction(U7(57));
    pub const UNDEFINED_26_LSB: ControlFunction = ControlFunction(U7(58));
    pub const UNDEFINED_27_LSB: ControlFunction = ControlFunction(U7(59));
    pub const UNDEFINED_28_LSB: ControlFunction = ControlFunction(U7(60));
    pub const UNDEFINED_29_LSB: ControlFunction = ControlFunction(U7(61));
    pub const UNDEFINED_30_LSB: ControlFunction = ControlFunction(U7(62));
    pub const UNDEFINED_31_LSB: ControlFunction = ControlFunction(U7(63));

    /// Either known as the Hold, Sustain or Damper pedal.
    ///
    /// [MIDI 1.0] Response to NoteOff and AllNotesOff should be delayed while
    /// this switch is on (value >= 64) until it transitions to off.
    ///
    /// [GM2] May be treated as a continuous controller instead of a switch
    /// for the "Half Damper" and "re-damper" effects.
    pub const DAMPER_PEDAL: ControlFunction = ControlFunction(U7(64));
    /// [MIDI 1.0] Turns the Portamento effect on (value >= 64) or off.
    pub const PORTAMENTO_ON_OFF: ControlFunction = ControlFunction(U7(65));
    /// [MIDI 1.0] Same as DamperPedal (64), but only affects the notes being held
    /// **while** the switch transitions to on (value >= 64). Any note played
    /// while the switch is already on behaves as normal.
    pub const SOSTENUTO: ControlFunction = ControlFunction(U7(66));
    /// [GM2] Notes played while this switch is on (value >= 64) should be
    /// played at a reduced volume.
    pub const SOFT_PEDAL: ControlFunction = ControlFunction(U7(67));
    /// [MIDI 1.0] This switch turns on (value >= 64) the monophonic legato response
    /// mode for that channel, where receiving a NoteOn while a note is already
    /// playing will change the pitch of the current note accordingly (without
    /// replaying the attack or re-attacking the envelopes).
    pub const LEGATO_FOOTSWITCH: ControlFunction = ControlFunction(U7(68));
    /// [MIDI 1.0] Additional controller for hold functions that don't match
    /// the specified definition of DamperPedal (64).
    pub const HOLD_2: ControlFunction = ControlFunction(U7(69));
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Sound Variation"
    pub const SOUND_CONTROLLER_1: ControlFunction = ControlFunction(U7(70));
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Timbre/Harmonic Intensity"
    ///
    /// [GM2] Sets the strength of the resonance effect for filter(s) for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_2: ControlFunction = ControlFunction(U7(71));
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Release Time"
    ///
    /// [GM2] Controls the release time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_3: ControlFunction = ControlFunction(U7(72));
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Attack Time"
    ///
    /// [GM2] Controls the attack time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_4: ControlFunction = ControlFunction(U7(73));
    /// [MIDI 1.0] Remappable Sound Controller, Default Name: "Brightness"
    ///
    /// [GM2] Controls the preset cut-off frequency of the filter.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_5: ControlFunction = ControlFunction(U7(74));
    /// [RP-021] Remappable Sound Controller, Default Name: "Decay Time"
    ///
    /// [GM2] Controls the decay time of the envelope for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_6: ControlFunction = ControlFunction(U7(75));
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Rate"
    ///
    /// [GM2] Controls the vibrato rate on the specified Channel relative to the sound's preset rate.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_7: ControlFunction = ControlFunction(U7(76));
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Depth"
    ///
    /// [GM2] Controls the vibrato depth for the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_8: ControlFunction = ControlFunction(U7(77));
    /// [RP-021] Remappable Sound Controller, Default Name: "Vibrato Delay"
    ///
    /// [GM2] Controls the vibrato delay on the specified Channel.
    /// Exact behavior is left to the manufacturer's discretion.
    pub const SOUND_CONTROLLER_9: ControlFunction = ControlFunction(U7(78));
    /// [MIDI 1.0] Remappable Sound Controller, no default
    pub const SOUND_CONTROLLER_10: ControlFunction = ControlFunction(U7(79));
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_5: ControlFunction = ControlFunction(U7(80));
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_6: ControlFunction = ControlFunction(U7(81));
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_7: ControlFunction = ControlFunction(U7(82));
    /// [MIDI 1.0] 1 byte general-purpose controller for device-specific features.
    pub const GENERAL_PURPOSE_CONTROLLER_8: ControlFunction = ControlFunction(U7(83));
    /// [MIDI 1.0] Specifies the starting MIDI note from which the next NoteOn will slide.
    pub const PORTAMENTO_CONTROL: ControlFunction = ControlFunction(U7(84));
    pub const UNDEFINED_85: ControlFunction = ControlFunction(U7(85));
    pub const UNDEFINED_86: ControlFunction = ControlFunction(U7(86));
    pub const UNDEFINED_87: ControlFunction = ControlFunction(U7(87));
    /// [CA-031] High Resolution Velocity Prefix
    ///
    /// If sent before a NoteOn message, the value of this controller
    /// acts as an LSB for the velocity of that note.
    pub const UNDEFINED_88: ControlFunction = ControlFunction(U7(88));
    pub const UNDEFINED_89: ControlFunction = ControlFunction(U7(89));
    pub const UNDEFINED_90: ControlFunction = ControlFunction(U7(90));
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "External Effects Depth")
    ///
    /// [RP-023] No-longer general-purpose, renamed to "Reverb Send Level"
    ///
    /// [GM2] Specifies the Reverb Send Level for that channel, linearly from 0% to 100% of amplitude.
    pub const EFFECTS_1_DEPTH: ControlFunction = ControlFunction(U7(91));
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Tremolo Depth")
    pub const EFFECTS_2_DEPTH: ControlFunction = ControlFunction(U7(92));
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Chorus Depth")
    ///
    /// [RP-023] No-longer general-purpose, renamed to "Chorus Send Level"
    ///
    /// [GM2] Specifies the Chorus Send Level for that channel, linearly from 0% to 100% of amplitude.
    pub const EFFECTS_3_DEPTH: ControlFunction = ControlFunction(U7(93));
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Celeste (Detune) Depth")
    pub const EFFECTS_4_DEPTH: ControlFunction = ControlFunction(U7(94));
    /// [MIDI 1.0] General-purpose effect depth Controller (default: "Phaser Depth")
    pub const EFFECTS_5_DEPTH: ControlFunction = ControlFunction(U7(95));

    /// [RP-018] Increments the value of a RPN/NRPN by 1, regardless of the value byte.
    /// The actual behavior depends on the parameter.
    pub const DATA_INCREMENT: ControlFunction = ControlFunction(U7(96));
    /// [RP-018] Decrements the value of a RPN/NRPN by 1, regardless of the value byte.
    /// The actual behavior depends on the parameter.
    pub const DATA_DECREMENT: ControlFunction = ControlFunction(U7(97));
    /// [MIDI 1.0] Selects a parameter to be modified by DataIncrement (96),
    /// DataDecrement (97) and DataEntry (6 & 38). Unlike RPNs, NRPNs are
    /// manufacturer-specific.
    pub const NON_REGISTERED_PARAMETER_NUMBER_LSB: ControlFunction = ControlFunction(U7(98));
    pub const NON_REGISTERED_PARAMETER_NUMBER_MSB: ControlFunction = ControlFunction(U7(99));
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
    pub const REGISTERED_PARAMETER_NUMBER_LSB: ControlFunction = ControlFunction(U7(100));
    pub const REGISTERED_PARAMETER_NUMBER_MSB: ControlFunction = ControlFunction(U7(101));

    pub const UNDEFINED_102: ControlFunction = ControlFunction(U7(102));
    pub const UNDEFINED_103: ControlFunction = ControlFunction(U7(103));
    pub const UNDEFINED_104: ControlFunction = ControlFunction(U7(104));
    pub const UNDEFINED_105: ControlFunction = ControlFunction(U7(105));
    pub const UNDEFINED_106: ControlFunction = ControlFunction(U7(106));
    pub const UNDEFINED_107: ControlFunction = ControlFunction(U7(107));
    pub const UNDEFINED_108: ControlFunction = ControlFunction(U7(108));
    pub const UNDEFINED_109: ControlFunction = ControlFunction(U7(109));
    pub const UNDEFINED_110: ControlFunction = ControlFunction(U7(110));
    pub const UNDEFINED_111: ControlFunction = ControlFunction(U7(111));
    pub const UNDEFINED_112: ControlFunction = ControlFunction(U7(112));
    pub const UNDEFINED_113: ControlFunction = ControlFunction(U7(113));
    pub const UNDEFINED_114: ControlFunction = ControlFunction(U7(114));
    pub const UNDEFINED_115: ControlFunction = ControlFunction(U7(115));
    pub const UNDEFINED_116: ControlFunction = ControlFunction(U7(116));
    pub const UNDEFINED_117: ControlFunction = ControlFunction(U7(117));
    pub const UNDEFINED_118: ControlFunction = ControlFunction(U7(118));
    pub const UNDEFINED_119: ControlFunction = ControlFunction(U7(119));

    /// [MIDI 1.0] Indicates that the receiver should immediately silence (without
    /// going through the release phase and ignoring sustain) all notes currently
    /// sounding on that channel. May also be used to turn off lights.
    pub const ALL_SOUND_OFF: ControlFunction = ControlFunction(U7(120));
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
    pub const RESET_ALL_CONTROLLERS: ControlFunction = ControlFunction(U7(121));
    /// [MIDI 1.0] Specifies whether the instrument should react to notes being
    /// physically played on it (0: ControlFunction = off, 127 = on), as opposed to notes
    /// sent via MIDI-in.
    pub const LOCAL_CONTROL: ControlFunction = ControlFunction(U7(122));
    /// [MIDI 1.0] Ignored in Omni mode (mode 1 & 2). In Poly operation (mode 3),
    /// acts as a NoteOff for all notes playing on the instrument's basic channel
    /// (it is ignored for all other channels). In Mono operation (mode 4),
    /// acts as a NoteOff for all notes playing on the specified channel. In all cases,
    /// notes being played on the instrument itself should remain unaffected.
    ///
    /// [GM2] Turns off all Notes sounding on the specified Channel.
    pub const ALL_NOTES_OFF: ControlFunction = ControlFunction(U7(123));
    /// [MIDI 1.0] Same as AllNotesOff (123), then set receiver to mode 1 (omni-on, poly)
    /// or 2 (omni-on, mono) based on the current mode.
    ///
    /// [GM2] Same as AllNotesOff (123), since GM2 does not support Omni mode.
    pub const OMNI_MODE_ON: ControlFunction = ControlFunction(U7(124));
    /// [MIDI 1.0] Same as AllNotesOff (123), then set receiver to mode 3 (omni-off, poly)
    /// or 4 (omni-off, mono) based on the current mode.
    ///
    /// [GM2] Same as AllNotesOff (123), since GM2 does not support Omni mode.
    pub const OMNI_MODE_OFF: ControlFunction = ControlFunction(U7(125));
    /// [MIDI 1.0] Same as AllNotesOff (123), then set receiver to mode 2 (omni-on, mono)
    /// or 4 (omni-off, mono) based on the current mode. The value byte indicates how many
    /// channels to use, with 0 being "auto".
    ///
    /// [GM2] Same as AllNotesOff (123), then set the **channel** to mode 4. Will be
    /// ignored if the value byte is not equal to 1 of if the channel is a rhythm channel.
    pub const MONO_OPERATION: ControlFunction = ControlFunction(U7(126));
    /// [MIDI 1.0] Same as AllNotesOff (123), then set receiver to mode 1 (omni-on, poly)
    /// or 3 (omni-off, poly) based on the current mode.
    ///
    /// [GM2] Same as AllNotesOff (123), then set the **channel** to mode 3.
    pub const POLY_OPERATION: ControlFunction = ControlFunction(U7(127));
}

impl From<U7> for ControlFunction {
    fn from(data: U7) -> ControlFunction {
        ControlFunction(data)
    }
}

impl From<ControlFunction> for U7 {
    fn from(control_function: ControlFunction) -> U7 {
        control_function.0
    }
}

impl From<ControlFunction> for u8 {
    fn from(control_function: ControlFunction) -> u8 {
        control_function.0.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::U7;

    #[test]
    fn from_u7() {
        for value in 0..128 {
            let data = U7::new(value).unwrap();
            let cc = ControlFunction::from(data);
            assert_eq!(value, cc.into());
        }
    }
}
