use ::jack::RawMidi;

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

impl Message {
    pub fn encode_into<'a>(&self, r_result: &'a mut Vec<u8>) -> RawMidi<'a> {
        self.event.encode_into(r_result);
        RawMidi {
            time: self.delta_time as u32,
            bytes: r_result.as_slice(),
        }
    }
}
