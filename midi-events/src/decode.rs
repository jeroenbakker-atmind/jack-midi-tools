//! Decode a midi event.
use music_notes::{ChromaticNote, ChromaticScale, Scale};

use crate::{
    Channel, Event, Modulation, StatusCode, Value, Velocity, CONTROLLER_ALL_NOTES_OFF,
    CONTROLLER_CHANNEL_PAN, CONTROLLER_CHANNEL_VOLUME, STATUS_CONTROLLER, STATUS_KEY_PRESSURE,
    STATUS_MODULATION_WHEEL, STATUS_NOTE_OFF, STATUS_NOTE_ON, STATUS_PROGRAM_CHANGE,
    STATUS_SYSTEM_EXCLUSIVE,
};

fn decode_status_and_channel(midi_message: &[u8], index: &mut usize) -> (StatusCode, Channel) {
    let status_code = midi_message[*index] & 0xf0;
    let channel = (midi_message[*index] & 0x0f) + 1;
    *index += 1;
    (status_code, channel)
}

/// Decode the note part of a midi message to a music note.
///
/// # Example
/// ```
/// use midi_events::decode_note;
/// use music_notes::{ChromaticNote, ChromaticTone};
///
/// let mut index = 0;
/// assert_eq!(decode_note(&[60], &mut index), ChromaticNote::new(ChromaticTone::C, 4));
/// ```
pub fn decode_note(midi_message: &[u8], index: &mut usize) -> ChromaticNote {
    let scale = ChromaticScale::default();
    let midi_note = midi_message[*index];
    let octave = (midi_note / scale.tones_per_octave() as u8) - 1;
    let note_index = midi_note % scale.tones_per_octave() as u8;

    *index += 1;

    ChromaticNote::new(note_index, octave)
}

fn decode_velocity(midi_message: &[u8], index: &mut usize) -> Velocity {
    let velocity = midi_message[*index];
    *index += 1;
    velocity
}

fn decode_modulation(midi_message: &[u8], index: &mut usize) -> Modulation {
    let (value1, value2) = decode_two_values(midi_message, index);
    let modulation = ((value1 as u16) << 7) + value2 as u16;
    modulation
}
fn decode_value(midi_message: &[u8], index: &mut usize) -> Value {
    let value = midi_message[*index];
    *index += 1;
    value
}
fn decode_two_values(midi_message: &[u8], index: &mut usize) -> (Value, Value) {
    (
        decode_value(midi_message, index),
        decode_value(midi_message, index),
    )
}

fn decode_midi_event(midi_message: &[u8]) -> Event {
    let mut index = 0;
    let (status_code, channel) = decode_status_and_channel(midi_message, &mut index);
    match status_code {
        STATUS_NOTE_ON => Event::NoteOn(
            channel,
            decode_note(midi_message, &mut index),
            decode_velocity(midi_message, &mut index),
        ),
        STATUS_NOTE_OFF => Event::NoteOff(
            channel,
            decode_note(midi_message, &mut index),
            decode_velocity(midi_message, &mut index),
        ),
        STATUS_KEY_PRESSURE => Event::KeyPressure(
            channel,
            decode_note(midi_message, &mut index),
            decode_velocity(midi_message, &mut index),
        ),
        STATUS_MODULATION_WHEEL => {
            Event::ModulationWheel(channel, decode_modulation(midi_message, &mut index))
        }
        STATUS_CONTROLLER => {
            let (value_1, value_2) = decode_two_values(midi_message, &mut index);
            match value_1 {
                CONTROLLER_CHANNEL_VOLUME => Event::ChannelVolume(channel, value_2),
                CONTROLLER_CHANNEL_PAN => Event::ChannelPan(channel, value_2),
                CONTROLLER_ALL_NOTES_OFF => Event::AllNotesOff(channel),
                _ => Event::Controller(channel, value_1, value_2),
            }
        }
        STATUS_PROGRAM_CHANGE => {
            Event::ProgramChange(channel, decode_value(midi_message, &mut index))
        }
        STATUS_SYSTEM_EXCLUSIVE => Event::SystemExclusive(channel),
        _ => Event::Unknown(status_code),
    }
}

impl From<&[u8]> for Event {
    fn from(midi_message: &[u8]) -> Self {
        decode_midi_event(midi_message)
    }
}
