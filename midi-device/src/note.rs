use midi_events::{Channel, Velocity};
use music_notes::Note;

pub type NoteStateId = usize;

#[derive(Debug, Default, Clone)]
pub struct NoteState {
    pub channel_id: Option<Channel>,
    pub note: Note,
    pub velocity: Velocity,
}

impl NoteState {
    pub fn off(&mut self, velocity: Velocity) {
        self.velocity = velocity;
        self.channel_id = None;
    }

    pub fn on(&mut self, channel_id: Channel, note: &Note, velocity: Velocity) {
        self.channel_id = Some(channel_id);
        self.note = *note;
        self.velocity = velocity;
    }

    pub fn key_pressure(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}
