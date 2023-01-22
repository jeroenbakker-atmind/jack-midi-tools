use crate::{ChromaticTone, Scale};

pub struct Chromatic {}
impl Scale for Chromatic {
    type Tones = ChromaticTone;
    fn tones_per_octave(&self) -> usize {
        12
    }
}
