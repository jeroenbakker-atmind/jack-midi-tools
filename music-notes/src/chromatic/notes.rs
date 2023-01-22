use std::ops::Add;

use crate::{ChromaticTone, Note};

pub type NoteStep = i32;
pub const NOTES_PER_OCTAVE: u8 = 12;

pub type ChromaticNote = Note<ChromaticTone>;

impl Default for ChromaticNote {
    fn default() -> Self {
        ChromaticNote::new(ChromaticTone::C, 4)
    }
}

// TODO Make notes constant for easy access.
// TODO move to Note.
impl From<i32> for ChromaticNote {
    fn from(value: i32) -> Self {
        let octave = value / NOTES_PER_OCTAVE as i32;
        let note_index = value % NOTES_PER_OCTAVE as i32;
        Self::new(note_index as u8, octave as u8)
    }
}

impl From<ChromaticNote> for i32 {
    fn from(value: ChromaticNote) -> Self {
        (value.octave * NOTES_PER_OCTAVE + u8::from(value.tone)) as i32
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
