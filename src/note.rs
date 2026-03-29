use crate::Error;
use core::convert::TryFrom;
use core::fmt;

/// Precomputed frequencies (Hz) for all 128 MIDI notes using standard A440 tuning.
/// Formula: 2^((n + 36.37631656229591) / 12)
#[allow(clippy::excessive_precision)]
const FREQ_F64: [f64; 128] = [
    8.175798915643707e+00, // 0  CMinus1
    8.661957218027251e+00, // 1  DbMinus1
    9.177023997418985e+00, // 2  DMinus1
    9.722718241315027e+00, // 3  EbMinus1
    1.030086115352718e+01, // 4  EMinus1
    1.091338223228137e+01, // 5  FMinus1
    1.156232570973857e+01, // 6  GbMinus1
    1.224985737442966e+01, // 7  GMinus1
    1.297827179937328e+01, // 8  AbMinus1
    1.375000000000000e+01, // 9  AMinus1
    1.456761754744030e+01, // 10 BbMinus1
    1.543385316425388e+01, // 11 BMinus1
    1.635159783128741e+01, // 12 C0
    1.732391443605450e+01, // 13 Db0
    1.835404799483797e+01, // 14 D0
    1.944543648263005e+01, // 15 Eb0
    2.060172230705437e+01, // 16 E0
    2.182676446456274e+01, // 17 F0
    2.312465141947714e+01, // 18 Gb0
    2.449971474885933e+01, // 19 G0
    2.595654359874657e+01, // 20 Ab0
    2.749999999999999e+01, // 21 A0
    2.913523509488062e+01, // 22 Bb0
    3.086770632850775e+01, // 23 B0
    3.270319566257481e+01, // 24 C1
    3.464782887210901e+01, // 25 Db1
    3.670809598967594e+01, // 26 D1
    3.889087296526010e+01, // 27 Eb1
    4.120344461410874e+01, // 28 E1
    4.365352892912548e+01, // 29 F1
    4.624930283895428e+01, // 30 Gb1
    4.899942949771867e+01, // 31 G1
    5.191308719749313e+01, // 32 Ab1
    5.499999999999998e+01, // 33 A1
    5.827047018976124e+01, // 34 Bb1
    6.173541265701550e+01, // 35 B1
    6.540639132514963e+01, // 36 C2
    6.929565774421802e+01, // 37 Db2
    7.341619197935188e+01, // 38 D2
    7.778174593052020e+01, // 39 Eb2
    8.240688922821748e+01, // 40 E2
    8.730705785825096e+01, // 41 F2
    9.249860567790856e+01, // 42 Gb2
    9.799885899543733e+01, // 43 G2
    1.038261743949863e+02, // 44 Ab2
    1.100000000000000e+02, // 45 A2
    1.165409403795225e+02, // 46 Bb2
    1.234708253140310e+02, // 47 B2
    1.308127826502993e+02, // 48 C3
    1.385913154884360e+02, // 49 Db3
    1.468323839587038e+02, // 50 D3
    1.555634918610404e+02, // 51 Eb3
    1.648137784564350e+02, // 52 E3
    1.746141157165019e+02, // 53 F3
    1.849972113558171e+02, // 54 Gb3
    1.959977179908747e+02, // 55 G3
    2.076523487899725e+02, // 56 Ab3
    2.199999999999999e+02, // 57 A3
    2.330818807590450e+02, // 58 Bb3
    2.469416506280620e+02, // 59 B3
    2.616255653005987e+02, // 60 C4
    2.771826309768719e+02, // 61 Db4
    2.936647679174075e+02, // 62 D4
    3.111269837220810e+02, // 63 Eb4
    3.296275569128697e+02, // 64 E4
    3.492282314330038e+02, // 65 F4
    3.699944227116345e+02, // 66 Gb4
    3.919954359817490e+02, // 67 G4
    4.153046975799451e+02, // 68 Ab4
    4.400000000000001e+02, // 69 A4
    4.661637615180896e+02, // 70 Bb4
    4.938833012561240e+02, // 71 B4
    5.232511306011974e+02, // 72 C5
    5.543652619537438e+02, // 73 Db5
    5.873295358348150e+02, // 74 D5
    6.222539674441620e+02, // 75 Eb5
    6.592551138257395e+02, // 76 E5
    6.984564628660077e+02, // 77 F5
    7.399888454232689e+02, // 78 Gb5
    7.839908719634981e+02, // 79 G5
    8.306093951598901e+02, // 80 Ab5
    8.800000000000002e+02, // 81 A5
    9.323275230361793e+02, // 82 Bb5
    9.877666025122480e+02, // 83 B5
    1.046502261202395e+03, // 84 C6
    1.108730523907488e+03, // 85 Db6
    1.174659071669630e+03, // 86 D6
    1.244507934888324e+03, // 87 Eb6
    1.318510227651479e+03, // 88 E6
    1.396912925732015e+03, // 89 F6
    1.479977690846538e+03, // 90 Gb6
    1.567981743926996e+03, // 91 G6
    1.661218790319782e+03, // 92 Ab6
    1.760000000000000e+03, // 93 A6
    1.864655046072361e+03, // 94 Bb6
    1.975533205024499e+03, // 95 B6
    2.093004522404789e+03, // 96 C7
    2.217461047814978e+03, // 97 Db7
    2.349318143339263e+03, // 98 D7
    2.489015869776648e+03, // 99 Eb7
    2.637020455302961e+03, // 100 E7
    2.793825851464034e+03, // 101 F7
    2.959955381693076e+03, // 102 Gb7
    3.135963487853996e+03, // 103 G7
    3.322437580639565e+03, // 104 Ab7
    3.520000000000001e+03, // 105 A7
    3.729310092144722e+03, // 106 Bb7
    3.951066410048997e+03, // 107 B7
    4.186009044809579e+03, // 108 C8
    4.434922095629956e+03, // 109 Db8
    4.698636286678526e+03, // 110 D8
    4.978031739553296e+03, // 111 Eb8
    5.274040910605922e+03, // 112 E8
    5.587651702928068e+03, // 113 F8
    5.919910763386151e+03, // 114 Gb8
    6.271926975707993e+03, // 115 G8
    6.644875161279129e+03, // 116 Ab8
    7.040000000000002e+03, // 117 A8
    7.458620184289443e+03, // 118 Bb8
    7.902132820097994e+03, // 119 B8
    8.372018089619158e+03, // 120 C9
    8.869844191259912e+03, // 121 Db9
    9.397272573357051e+03, // 122 D9
    9.956063479106591e+03, // 123 Eb9
    1.054808182121184e+04, // 124 E9
    1.117530340585614e+04, // 125 F9
    1.183982152677230e+04, // 126 Gb9
    1.254385395141599e+04, // 127 G9
];

/// Precomputed frequencies (Hz) for all 128 MIDI notes using standard A440 tuning.
/// Formula: 2^((n + 36.376316) / 12)
#[allow(clippy::excessive_precision)]
const FREQ_F32: [f32; 128] = [
    8.1757987e+00_f32, // 0  CMinus1
    8.6619569e+00_f32, // 1  DbMinus1
    9.1770237e+00_f32, // 2  DMinus1
    9.7227179e+00_f32, // 3  EbMinus1
    1.0300861e+01_f32, // 4  EMinus1
    1.0913382e+01_f32, // 5  FMinus1
    1.1562325e+01_f32, // 6  GbMinus1
    1.2249857e+01_f32, // 7  GMinus1
    1.2978271e+01_f32, // 8  AbMinus1
    1.3750000e+01_f32, // 9  AMinus1
    1.4567617e+01_f32, // 10 BbMinus1
    1.5433853e+01_f32, // 11 BMinus1
    1.6351597e+01_f32, // 12 C0
    1.7323914e+01_f32, // 13 Db0
    1.8354047e+01_f32, // 14 D0
    1.9445436e+01_f32, // 15 Eb0
    2.0601722e+01_f32, // 16 E0
    2.1826764e+01_f32, // 17 F0
    2.3124651e+01_f32, // 18 Gb0
    2.4499714e+01_f32, // 19 G0
    2.5956543e+01_f32, // 20 Ab0
    2.7499999e+01_f32, // 21 A0
    2.9135234e+01_f32, // 22 Bb0
    3.0867705e+01_f32, // 23 B0
    3.2703195e+01_f32, // 24 C1
    3.4647828e+01_f32, // 25 Db1
    3.6708095e+01_f32, // 26 D1
    3.8890872e+01_f32, // 27 Eb1
    4.1203443e+01_f32, // 28 E1
    4.3653528e+01_f32, // 29 F1
    4.6249301e+01_f32, // 30 Gb1
    4.8999428e+01_f32, // 31 G1
    5.1913086e+01_f32, // 32 Ab1
    5.4999998e+01_f32, // 33 A1
    5.8270468e+01_f32, // 34 Bb1
    6.1735411e+01_f32, // 35 B1
    6.5406389e+01_f32, // 36 C2
    6.9295655e+01_f32, // 37 Db2
    7.3416190e+01_f32, // 38 D2
    7.7781743e+01_f32, // 39 Eb2
    8.2406887e+01_f32, // 40 E2
    8.7307055e+01_f32, // 41 F2
    9.2498603e+01_f32, // 42 Gb2
    9.7998856e+01_f32, // 43 G2
    1.0382617e+02_f32, // 44 Ab2
    1.1000000e+02_f32, // 45 A2
    1.1654094e+02_f32, // 46 Bb2
    1.2347082e+02_f32, // 47 B2
    1.3081278e+02_f32, // 48 C3
    1.3859131e+02_f32, // 49 Db3
    1.4683238e+02_f32, // 50 D3
    1.5556349e+02_f32, // 51 Eb3
    1.6481377e+02_f32, // 52 E3
    1.7461411e+02_f32, // 53 F3
    1.8499721e+02_f32, // 54 Gb3
    1.9599771e+02_f32, // 55 G3
    2.0765234e+02_f32, // 56 Ab3
    2.1999999e+02_f32, // 57 A3
    2.3308187e+02_f32, // 58 Bb3
    2.4694164e+02_f32, // 59 B3
    2.6162556e+02_f32, // 60 C4
    2.7718262e+02_f32, // 61 Db4
    2.9366476e+02_f32, // 62 D4
    3.1112697e+02_f32, // 63 Eb4
    3.2962755e+02_f32, // 64 E4
    3.4922822e+02_f32, // 65 F4
    3.6999441e+02_f32, // 66 Gb4
    3.9199542e+02_f32, // 67 G4
    4.1530468e+02_f32, // 68 Ab4
    4.3999999e+02_f32, // 69 A4
    4.6616375e+02_f32, // 70 Bb4
    4.9388329e+02_f32, // 71 B4
    5.2325111e+02_f32, // 72 C5
    5.5436524e+02_f32, // 73 Db5
    5.8732952e+02_f32, // 74 D5
    6.2225395e+02_f32, // 75 Eb5
    6.5925509e+02_f32, // 76 E5
    6.9845644e+02_f32, // 77 F5
    7.3998882e+02_f32, // 78 Gb5
    7.8399085e+02_f32, // 79 G5
    8.3060937e+02_f32, // 80 Ab5
    8.7999997e+02_f32, // 81 A5
    9.3232749e+02_f32, // 82 Bb5
    9.8776657e+02_f32, // 83 B5
    1.0465022e+03_f32, // 84 C6
    1.1087305e+03_f32, // 85 Db6
    1.1746590e+03_f32, // 86 D6
    1.2445079e+03_f32, // 87 Eb6
    1.3185102e+03_f32, // 88 E6
    1.3969129e+03_f32, // 89 F6
    1.4799776e+03_f32, // 90 Gb6
    1.5679817e+03_f32, // 91 G6
    1.6612187e+03_f32, // 92 Ab6
    1.7599999e+03_f32, // 93 A6
    1.8646550e+03_f32, // 94 Bb6
    1.9755331e+03_f32, // 95 B6
    2.0930045e+03_f32, // 96 C7
    2.2174610e+03_f32, // 97 Db7
    2.3493181e+03_f32, // 98 D7
    2.4890158e+03_f32, // 99 Eb7
    2.6370204e+03_f32, // 100 E7
    2.7938258e+03_f32, // 101 F7
    2.9599553e+03_f32, // 102 Gb7
    3.1359634e+03_f32, // 103 G7
    3.3224375e+03_f32, // 104 Ab7
    3.5199999e+03_f32, // 105 A7
    3.7293100e+03_f32, // 106 Bb7
    3.9510663e+03_f32, // 107 B7
    4.1860089e+03_f32, // 108 C8
    4.4349220e+03_f32, // 109 Db8
    4.6986361e+03_f32, // 110 D8
    4.9780316e+03_f32, // 111 Eb8
    5.2740407e+03_f32, // 112 E8
    5.5876515e+03_f32, // 113 F8
    5.9199106e+03_f32, // 114 Gb8
    6.2719268e+03_f32, // 115 G8
    6.6448749e+03_f32, // 116 Ab8
    7.0399998e+03_f32, // 117 A8
    7.4586199e+03_f32, // 118 Bb8
    7.9021326e+03_f32, // 119 B8
    8.3720178e+03_f32, // 120 C9
    8.8698439e+03_f32, // 121 Db9
    9.3972723e+03_f32, // 122 D9
    9.9560632e+03_f32, // 123 Eb9
    1.0548081e+04_f32, // 124 E9
    1.1175303e+04_f32, // 125 F9
    1.1839821e+04_f32, // 126 Gb9
    1.2543854e+04_f32, // 127 G9
];

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
    #[inline(always)]
    pub fn to_freq_f32(self) -> f32 {
        FREQ_F32[self as usize]
    }

    /// The frequency using the standard 440Hz tuning.
    ///
    /// # Example
    /// ```
    /// # fn sing(frequency: f64) {}
    /// let note = wmidi::Note::A3;
    /// sing(note.to_freq_f64());
    /// ```
    #[inline(always)]
    pub fn to_freq_f64(self) -> f64 {
        FREQ_F64[self as usize]
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
    fn note_to_frequency_a440() {
        let a440_f64 = Note::A4.to_freq_f64();
        assert!((a440_f64 - 440.0).abs() < 1E-10, "{} != 440", a440_f64);

        let a440_f32 = Note::A4.to_freq_f32();
        assert!((a440_f32 - 440.0).abs() < 1E-3, "{} != 440", a440_f32);
    }

    #[cfg(feature = "std")]
    #[test]
    fn note_to_frequency_f64() {
        for midi in 0u8..=127 {
            let note = Note::from_u8_lossy(midi);
            let expected = {
                let exp = (f64::from(midi) + 36.376_316_562_295_91) / 12.0;
                2.0_f64.powf(exp)
            };
            let got = note.to_freq_f64();
            let rel_err = (got - expected).abs() / expected;
            assert!(
                rel_err < 1e-4,
                "note={}: got {} expected {} rel_err={}",
                midi,
                got,
                expected,
                rel_err
            );
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn note_to_frequency_f32() {
        for midi in 0u8..=127 {
            let note = Note::from_u8_lossy(midi);
            let expected = {
                let exp = (f32::from(midi) + 36.376_316_f32) / 12.0;
                2.0_f32.powf(exp)
            };
            let got = note.to_freq_f32();
            let rel_err = (got - expected).abs() / expected;
            assert!(
                rel_err < 1e-4,
                "note={}: got {} expected {} rel_err={}",
                midi,
                got,
                expected,
                rel_err
            );
        }
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
