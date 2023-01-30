use music_notes::{ChromaticNote, ChromaticScale, Scale};

pub const STATUS_NOTE_OFF: u8 = 0x80;
pub const STATUS_NOTE_ON: u8 = 0x90;
pub const STATUS_KEY_PRESSURE: u8 = 0xa0;
pub const STATUS_CONTROLLER: u8 = 0xb0;
pub const STATUS_PROGRAM_CHANGE: u8 = 0xc0;
pub const STATUS_CHANNEL_PRESSURE: u8 = 0xd0;
pub const STATUS_MODULATION_WHEEL: u8 = 0xe0;
pub const STATUS_SYSTEM_EXCLUSIVE: u8 = 0xf0;

pub const CONTROLLER_CHANNEL_VOLUME: u8 = 7;
pub const CONTROLLER_CHANNEL_PAN: u8 = 10;
pub const CONTROLLER_ALL_NOTES_OFF: u8 = 123;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    NoteOn(Channel, ChromaticNote, Velocity),
    NoteOff(Channel, ChromaticNote, Velocity),

    KeyPressure(Channel, ChromaticNote, Pressure),
    ProgramChange(Channel, Program),

    Controller(Channel, u8, u8),
    ChannelPan(Channel, Value),
    ChannelVolume(Channel, Value),
    AllNotesOff(Channel),
    SystemExclusive(Channel),
    Unknown(u8),
}

pub type StatusCode = u8;
pub type Value = u8;
pub type Channel = Value;
pub type Velocity = Value;
pub type Pressure = Value;
pub type Program = Value;
pub type Ticks = usize;

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
