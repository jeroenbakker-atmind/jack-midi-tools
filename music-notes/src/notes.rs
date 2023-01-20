use std::ops::Add;

pub type Octave = u8;
pub type NoteStep = i32;
pub const NOTES_PER_OCTAVE: u8 = 12;

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Note {
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

impl Default for Note {
    fn default() -> Self {
        Note::C(4)
    }
}

impl Note {
    pub fn new(note_index: u8, octave: Octave) -> Note {
        match note_index {
            0 => Note::C(octave),
            1 => Note::CSharp(octave),
            2 => Note::D(octave),
            3 => Note::DSharp(octave),
            4 => Note::E(octave),
            5 => Note::F(octave),
            6 => Note::FSharp(octave),
            7 => Note::G(octave),
            8 => Note::GSharp(octave),
            9 => Note::A(octave),
            10 => Note::ASharp(octave),
            11 => Note::B(octave),
            _ => unreachable!(),
        }
    }

    fn note_index(&self) -> u8 {
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

impl From<i32> for Note {
    fn from(value: i32) -> Self {
        let octave = value / NOTES_PER_OCTAVE as i32;
        let note_index = value % NOTES_PER_OCTAVE as i32;
        Self::new(note_index as u8, octave as u8)
    }
}

impl From<Note> for i32 {
    fn from(value: Note) -> Self {
        (value.octave() * NOTES_PER_OCTAVE + value.note_index()) as i32
    }
}

impl Add<NoteStep> for Note {
    type Output = Note;
    fn add(self, rhs: NoteStep) -> Self::Output {
        let mut value = i32::from(self);
        value += rhs;
        Note::from(value)
    }
}
