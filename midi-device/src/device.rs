use std::ops::{Index, IndexMut};

use midi_events::{Channel, Event, Velocity};
use music_notes::ChromaticNote;

use crate::NoteStateId;

use super::{ChannelState, NoteState};

#[derive(Debug)]
pub struct DeviceState {
    notes: Vec<NoteState>,
    channels: Vec<ChannelState>,
    unused_notes: Vec<NoteStateId>,
}

impl DeviceState {
    /// Create a new midi device state with the given midi channels and polyphonic notes.
    ///
    /// # Examples
    ///
    /// Apply and note-on and a note-off event.
    /// ```
    /// use midi_device::*;
    ///
    /// let mut device = DeviceState::new(1, 16);
    /// assert_eq!(device.unused_notes_size(), 16);
    /// ```
    pub fn new(num_channels: usize, num_polyphony_notes: usize) -> DeviceState {
        DeviceState {
            notes: vec![NoteState::default(); num_polyphony_notes],
            channels: vec![ChannelState::default(); num_channels],
            unused_notes: (0..num_polyphony_notes).collect(),
        }
    }

    /// Apply the given midi event.
    ///
    /// # Examples
    ///
    /// Apply and note-on and a note-off event.
    /// ```
    /// use midi_device::*;
    /// use midi_events::*;
    /// use music_notes::*;
    ///
    /// let mut device = DeviceState::new(1, 8);
    /// assert_eq!(device.unused_notes_size(), 8);
    /// let note_on = Event::NoteOn(1, ChromaticNote::new(ChromaticTone::C, 1), 64);
    /// device.apply_event(&note_on);
    /// assert_eq!(device.unused_notes_size(), 7);
    /// let note_off = Event::NoteOff(1, ChromaticNote::new(ChromaticTone::C, 1), 64);
    /// device.apply_event(&note_off);
    /// assert_eq!(device.unused_notes_size(), 8);
    /// ```
    pub fn apply_event(&mut self, event: &Event) {
        match event {
            Event::AllNotesOff(channel_id) => self.all_notes_off(*channel_id),
            Event::NoteOn(channel_id, note, velocity) => self.note_on(*channel_id, note, *velocity),
            Event::NoteOff(channel_id, note, velocity) => {
                self.note_off(*channel_id, note, *velocity)
            }
            Event::KeyPressure(channel_id, note, velocity) => {
                self.key_pressure(*channel_id, note, *velocity);
            }
            _ => {}
        }
    }

    /// Return the number of unused notes.
    ///
    /// A device has a specific polyphony. This method returns the number of polyphone notes
    /// that aren't linked to a channel.
    ///
    /// # Example
    ///
    /// ```
    /// use midi_device::*;
    /// use midi_events::*;
    /// use music_notes::*;
    ///
    /// let mut device = DeviceState::new(1, 8);
    /// assert_eq!(device.unused_notes_size(), 8);
    /// let note_on = Event::NoteOn(1, ChromaticNote::new(ChromaticTone::C, 4), 64);
    /// device.apply_event(&note_on);
    /// assert_eq!(device.unused_notes_size(), 7);
    /// ```
    pub fn unused_notes_size(&self) -> usize {
        self.unused_notes.len()
    }

    fn note_on(&mut self, channel_id: Channel, note: &ChromaticNote, velocity: Velocity) {
        let channel = &self[channel_id];
        if let None = self.find_note_id(channel, note) {
            if let Some(note_id) = self.unused_notes.pop() {
                self.notes[note_id].on(channel_id, note, velocity);

                let channel = &mut self[channel_id];
                channel.active_notes.push(note_id);
            }
        }
    }

    fn note_off(&mut self, channel_id: Channel, note: &ChromaticNote, velocity: Velocity) {
        let channel = &self[channel_id];
        if let Some((active_note_index, note_id)) = self.find_note_id(channel, note) {
            self.notes[note_id].off(velocity);
            self[channel_id].active_notes.remove(active_note_index);
            self.unused_notes.push(note_id)
        }
    }

    fn key_pressure(&mut self, channel_id: Channel, note: &ChromaticNote, velocity: Velocity) {
        let channel = &self[channel_id];
        if let Some((_active_note_index, note_id)) = self.find_note_id(channel, note) {
            self.notes[note_id].key_pressure(velocity);
        }
    }

    fn find_note_id(
        &self,
        channel: &ChannelState,
        note: &ChromaticNote,
    ) -> Option<(usize, NoteStateId)> {
        channel
            .active_notes
            .iter()
            .enumerate()
            .filter(|(_index, note_id)| &self.notes[**note_id].note == note)
            .map(|(index, note_id)| (index, *note_id))
            .next()
    }

    fn all_notes_off(&mut self, channel_id: Channel) {
        let notes_to_clear = self[channel_id].active_notes.clone();
        self[channel_id].active_notes.clear();
        for note_id in notes_to_clear {
            self.notes[note_id].off(Velocity::default());
            self.unused_notes.push(note_id);
        }
    }

    pub fn note_by_note_index(&self, id: NoteStateId) -> &'_ NoteState {
        &self.notes[id]
    }
}

impl Index<Channel> for DeviceState {
    type Output = ChannelState;
    fn index(&self, channel_id: Channel) -> &Self::Output {
        &self.channels[(channel_id - 1) as usize]
    }
}

impl IndexMut<Channel> for DeviceState {
    fn index_mut(&mut self, channel_id: Channel) -> &mut Self::Output {
        &mut self.channels[(channel_id - 1) as usize]
    }
}
