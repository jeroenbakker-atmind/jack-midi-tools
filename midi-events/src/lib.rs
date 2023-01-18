/// Used https://www.recordingblogs.com/wiki/midi-event for reference.
use music_notes::{Note, NOTES_PER_OCTAVE};

#[derive(Debug)]
pub enum Event {
    NoteOn(Channel, Note, Velocity),
    NoteOff(Channel, Note, Velocity),
    KeyPressure(Channel, Note, Pressure),

    Controller(Channel, u8, u8),
    ChannelPan(Channel, Value),
    ChannelVolume(Channel, Value),
    AllNotesOff(Channel),
    SystemExclusive(Channel),
    Unknown(u8),
}

const STATUS_NOTE_ON: u8 = 0x80;
const STATUS_NOTE_OFF: u8 = 0x90;
const STATUS_KEY_PRESSURE: u8 = 0xa0;
const STATUS_CONTROLLER: u8 = 0xb0;
const STATUS_SYSTEM_EXCLUSIVE: u8 = 0xe0;

const CONTROLLER_CHANNEL_VOLUME: u8 = 7;
const CONTROLLER_CHANNEL_PAN: u8 = 10;
const CONTROLLER_ALL_NOTES_OFF: u8 = 123;

type StatusCode = u8;
type Channel = u8;
type Ticks = u64;
type Velocity = u8;
type Pressure = u8;
type Value = u8;

pub struct MidiMessage {
    pub delta_ticks: Ticks,
    pub event: Event,
}

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
/// use music_notes::Note;
///
/// let mut index = 0;
/// assert_eq!(decode_note(&[60], &mut index), Note::C(4));
/// ```
pub fn decode_note(midi_message: &[u8], index: &mut usize) -> Note {
    let midi_note = midi_message[*index];
    let octave = (midi_note / NOTES_PER_OCTAVE) - 1;
    let note_index = midi_note % NOTES_PER_OCTAVE;

    *index += 1;

    Note::new(note_index, octave)
}

fn decode_velocity(midi_message: &[u8], index: &mut usize) -> Velocity {
    let velocity = midi_message[*index];
    *index += 1;
    velocity
}

fn decode_two_values(midi_message: &[u8], index: &mut usize) -> (Value, Value) {
    let value_1 = midi_message[*index];
    *index += 1;
    let value_2 = midi_message[*index];
    *index += 1;
    (value_1, value_2)
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
