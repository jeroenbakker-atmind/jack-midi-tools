use jack::RawMidi;

use crate::*;

impl From<&RawMidi<'_>> for Message {
    fn from(jack_event: &RawMidi) -> Self {
        let midi_event = Event::from(jack_event.bytes);
        Self {
            delta_time: jack_event.time as usize,
            event: midi_event,
        }
    }
}
