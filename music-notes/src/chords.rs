use crate::{Note, NoteStep};

pub enum ChordScale {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    SevenSus2,
    SevenSus4,
    Sixth,
    Seventh,
    Ninth,
    MajorSeventh,
    MajorNinth,
    MajorEleventh,
    MinorSixth,
    MinorSeventh,
    MinorNinth,
    MinorEleventh,
}

impl ChordScale {
    pub fn note_steps(&self) -> Vec<NoteStep> {
        match self {
            Self::Major => vec![0, 4, 7],
            Self::Minor => vec![0, 3, 7],
            Self::Diminished => vec![0, 3, 6],
            Self::Augmented => vec![0, 4, 8],
            Self::Sus2 => vec![0, 2, 7],
            Self::Sus4 => vec![0, 5, 7],
            Self::SevenSus2 => vec![0, 2, 7, 10],
            Self::SevenSus4 => vec![0, 5, 7, 10],
            Self::Sixth => vec![0, 4, 7, 9],
            Self::Seventh => vec![0, 4, 7, 10],
            Self::Ninth => vec![0, 4, 7, 10, 14],
            Self::MajorSeventh => vec![0, 4, 7, 11],
            Self::MajorNinth => vec![0, 4, 7, 11, 14],
            Self::MajorEleventh => vec![0, 4, 7, 11, 14, 17],
            Self::MinorSixth => vec![0, 3, 7, 9],
            Self::MinorSeventh => vec![0, 3, 7, 10],
            Self::MinorNinth => vec![0, 2, 3, 7, 10],
            Self::MinorEleventh => vec![0, 3, 7, 10, 14, 17],
        }
    }

    /// ```
    /// use music_notes::*;
    /// let notes = ChordScale::Major.notes(Note::C(4));
    /// assert_eq!(notes, vec![Note::C(4), Note::E(4), Note::G(4)]);
    /// ```
    pub fn notes(&self, base_note: Note) -> Vec<Note> {
        self.note_steps().iter().map(|s| base_note + *s).collect()
    }
}
