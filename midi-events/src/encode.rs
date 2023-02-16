//! Encode midi event to vector of bytes.

use music_notes::ChromaticNote;

use crate::{
    Channel, Event, Modulation, StatusCode, Velocity, CONTROLLER_ALL_NOTES_OFF,
    CONTROLLER_CHANNEL_PAN, CONTROLLER_CHANNEL_VOLUME, STATUS_CHANNEL_PRESSURE, STATUS_CONTROLLER,
    STATUS_MODULATION_WHEEL, STATUS_NOTE_OFF, STATUS_NOTE_ON, STATUS_PROGRAM_CHANGE,
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

    fn encode_controller_with_value(&mut self, controller: u8, value: u8) {
        self.encode_controller(controller);
        self.encode_value(value);
    }

    fn encode_controller(&mut self, controller: u8) {
        self.write_byte(controller);
    }

    fn encode_value(&mut self, value: Velocity) {
        self.write_byte(value);
    }

    fn encode_modulation(&mut self, modulation: Modulation) {
        let byte2 = (modulation & 0b01111111) as u8;
        let byte1 = (modulation >> 7) as u8;
        self.write_byte(byte1);
        self.write_byte(byte2);
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
                r_result.encode_value(0);
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
            Self::ChannelPan(channel, value) => {
                r_result.encode_status_and_channel(STATUS_CONTROLLER, *channel);
                r_result.encode_controller_with_value(CONTROLLER_CHANNEL_PAN, *value);
            }
            Self::ChannelVolume(channel, volume) => {
                r_result.encode_status_and_channel(STATUS_CONTROLLER, *channel);
                r_result.encode_controller_with_value(CONTROLLER_CHANNEL_VOLUME, *volume);
            }
            Self::KeyPressure(channel, note, pressure) => {
                r_result.encode_status_and_channel(STATUS_CHANNEL_PRESSURE, *channel);
                r_result.encode_note(*note);
                r_result.encode_value(*pressure);
            }
            Self::ProgramChange(channel, program) => {
                r_result.encode_status_and_channel(STATUS_PROGRAM_CHANGE, *channel);
                r_result.encode_value(*program);
            }
            Self::Controller(channel, value1, value2) => {
                r_result.encode_status_and_channel(STATUS_CONTROLLER, *channel);
                r_result.encode_value(*value1);
                r_result.encode_value(*value2);
            }
            Self::ModulationWheel(channel, modulation) => {
                r_result.encode_status_and_channel(STATUS_MODULATION_WHEEL, *channel);
                r_result.encode_modulation(*modulation);
            }

            _ => {
                println!("{self:#?}");
                unimplemented!()
            }
        }
    }
}
