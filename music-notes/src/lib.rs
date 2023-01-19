pub type Octave = u8;
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
}
