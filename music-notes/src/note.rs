use crate::Octave;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Note<T>
where
    T: Sized,
{
    pub tone: T,
    pub octave: Octave,
}

impl<T> Note<T>
where
    T: Copy + Sized,
    Self: Sized,
{
    pub fn new<I>(tone: I, octave: Octave) -> Self
    where
        I: Into<T>,
    {
        Self {
            tone: tone.into(),
            octave,
        }
    }
}
