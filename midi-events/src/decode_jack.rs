use music_notes::{ChromaticNote, NOTES_PER_OCTAVE};

use crate::*;

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
/// use music_notes::ChromaticNote;
///
/// let mut index = 0;
/// assert_eq!(decode_note(&[60], &mut index), ChromaticNote::C(4));
/// ```
pub fn decode_note(midi_message: &[u8], index: &mut usize) -> ChromaticNote {
    let midi_note = midi_message[*index];
    let octave = (midi_note / NOTES_PER_OCTAVE) - 1;
    let note_index = midi_note % NOTES_PER_OCTAVE;

    *index += 1;

    ChromaticNote::new(note_index, octave)
}

fn decode_velocity(midi_message: &[u8], index: &mut usize) -> Velocity {
    let velocity = midi_message[*index];
    *index += 1;
    velocity
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

fn decode_midi_event(midi_message: &[u8], index: &mut usize) -> Event {
    let (status_code, channel) = decode_status_and_channel(midi_message, index);
    match status_code {
        STATUS_NOTE_ON => Event::NoteOn(
            channel,
            decode_note(midi_message, index),
            decode_velocity(midi_message, index),
        ),
        STATUS_NOTE_OFF => Event::NoteOff(
            channel,
            decode_note(midi_message, index),
            decode_velocity(midi_message, index),
        ),
        STATUS_KEY_PRESSURE => Event::KeyPressure(
            channel,
            decode_note(midi_message, index),
            decode_velocity(midi_message, index),
        ),
        STATUS_CONTROLLER => {
            let (value_1, value_2) = decode_two_values(midi_message, index);
            match value_1 {
                CONTROLLER_CHANNEL_VOLUME => Event::ChannelVolume(channel, value_2),
                CONTROLLER_CHANNEL_PAN => Event::ChannelPan(channel, value_2),
                CONTROLLER_ALL_NOTES_OFF => Event::AllNotesOff(channel),
                _ => Event::Controller(channel, value_1, value_2),
            }
        }
        STATUS_PROGRAM_CHANGE => Event::ProgramChange(channel, decode_value(midi_message, index)),
        STATUS_SYSTEM_EXCLUSIVE => Event::SystemExclusive(channel),
        _ => Event::Unknown(status_code),
    }
}

impl From<&[u8]> for Event {
    fn from(midi_message: &[u8]) -> Self {
        let mut index = 0;
        decode_midi_event(midi_message, &mut index)
    }
}
