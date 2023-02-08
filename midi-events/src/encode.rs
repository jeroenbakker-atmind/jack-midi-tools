//! Encode midi event to vector of bytes.

use music_notes::ChromaticNote;

use crate::{
    Channel, Event, StatusCode, Velocity, CONTROLLER_ALL_NOTES_OFF, STATUS_CONTROLLER,
    STATUS_NOTE_OFF, STATUS_NOTE_ON,
};

trait MidiEventEncoder {
    fn encode_controller_channel(&mut self, channel: Channel, controller_byte: u8) {
        self.encode_status_and_channel(STATUS_CONTROLLER, channel);
        self.write_byte(controller_byte);
    }

    fn encode_status_and_channel(&mut self, status: StatusCode, channel: Channel) {
        let zero_based_channel = channel - 1;
        let byte = status | zero_based_channel;
        self.write_byte(byte);
    }

    fn encode_note(&mut self, note: ChromaticNote) {
        let byte = i32::from(note) as u8;
        self.write_byte(byte);
    }

    fn encode_velocity(&mut self, velocity: Velocity) {
        self.write_byte(velocity);
    }

    fn write_byte(&mut self, byte: u8);
}
impl MidiEventEncoder for Vec<u8> {
    fn write_byte(&mut self, byte: u8) {
        self.push(byte);
    }
}

impl Event {
    pub fn encode_into(&self, r_result: &mut Vec<u8>) {
        match self {
            Self::AllNotesOff(channel) => {
                r_result.encode_controller_channel(*channel, CONTROLLER_ALL_NOTES_OFF);
            }
            Self::NoteOn(channel, note, velocity) => {
                r_result.encode_status_and_channel(STATUS_NOTE_ON, *channel);
                r_result.encode_note(*note);
                r_result.encode_velocity(*velocity);
            }
            Self::NoteOff(channel, note, velocity) => {
                r_result.encode_status_and_channel(STATUS_NOTE_OFF, *channel);
                r_result.encode_note(*note);
                r_result.encode_velocity(*velocity);
            }

            _ => {
                println!("{self:#?}");
                unimplemented!()
            }
        }
    }
}
