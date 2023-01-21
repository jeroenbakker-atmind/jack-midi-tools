use crate::{Scale, Tone};

pub struct Chromatic {}
impl Scale for Chromatic {
    type Tones = Tone;
    fn tones_per_octave(&self) -> usize {
        12
    }
}
