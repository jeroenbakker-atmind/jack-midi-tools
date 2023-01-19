/// Used https://www.recordingblogs.com/wiki/midi-event for reference.
use music_notes::{Note, NOTES_PER_OCTAVE};

mod decode_jack;
mod event;
mod state;

pub use decode_jack::*;
pub use event::*;
pub use state::*;
