use std::ops::Add;

pub type Octave = u8;
pub type NoteStep = i32;
pub const NOTES_PER_OCTAVE: u8 = 12;

// TODO Should be a struct with tone and octave.
#[derive(Debug, Copy, PartialEq, Clone)]
pub enum ChromaticNote {
    C(Octave),
    CSharp(Octave),
    D(Octave),
    DSharp(Octave),
    E(Octave),
    F(Octave),
    FSharp(Octave),
    G(Octave),
    GSharp(Octave),
    A(Octave),
    ASharp(Octave),
    B(Octave),
}

impl Default for ChromaticNote {
    fn default() -> Self {
        ChromaticNote::C(4)
    }
}

impl ChromaticNote {
    pub fn new(note_index: u8, octave: Octave) -> ChromaticNote {
        match note_index {
            0 => ChromaticNote::C(octave),
            1 => ChromaticNote::CSharp(octave),
            2 => ChromaticNote::D(octave),
            3 => ChromaticNote::DSharp(octave),
            4 => ChromaticNote::E(octave),
            5 => ChromaticNote::F(octave),
            6 => ChromaticNote::FSharp(octave),
            7 => ChromaticNote::G(octave),
            8 => ChromaticNote::GSharp(octave),
            9 => ChromaticNote::A(octave),
            10 => ChromaticNote::ASharp(octave),
            11 => ChromaticNote::B(octave),
            _ => unreachable!(),
        }
    }

    fn tone_index(&self) -> u8 {
        match self {
            Self::C(_) => 0,
            Self::CSharp(_) => 1,
            Self::D(_) => 2,
            Self::DSharp(_) => 3,
            Self::E(_) => 4,
            Self::F(_) => 5,
            Self::FSharp(_) => 6,
            Self::G(_) => 7,
            Self::GSharp(_) => 8,
            Self::A(_) => 9,
            Self::ASharp(_) => 10,
            Self::B(_) => 12,
        }
    }
    fn octave(&self) -> u8 {
        *match self {
            Self::C(octave) => octave,
            Self::CSharp(octave) => octave,
            Self::D(octave) => octave,
            Self::DSharp(octave) => octave,
            Self::E(octave) => octave,
            Self::F(octave) => octave,
            Self::FSharp(octave) => octave,
            Self::G(octave) => octave,
            Self::GSharp(octave) => octave,
            Self::A(octave) => octave,
            Self::ASharp(octave) => octave,
            Self::B(octave) => octave,
        }
    }
}

impl From<i32> for ChromaticNote {
    fn from(value: i32) -> Self {
        let octave = value / NOTES_PER_OCTAVE as i32;
        let note_index = value % NOTES_PER_OCTAVE as i32;
        Self::new(note_index as u8, octave as u8)
    }
}

impl From<ChromaticNote> for i32 {
    fn from(value: ChromaticNote) -> Self {
        (value.octave() * NOTES_PER_OCTAVE + value.tone_index()) as i32
    }
}

impl Add<NoteStep> for ChromaticNote {
    type Output = ChromaticNote;
    fn add(self, rhs: NoteStep) -> Self::Output {
        let mut value = i32::from(self);
        value += rhs;
        ChromaticNote::from(value)
    }
}
