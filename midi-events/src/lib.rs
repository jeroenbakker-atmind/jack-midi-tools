/// Used https://www.recordingblogs.com/wiki/midi-event for reference.
use music_notes::{Note, NOTES_PER_OCTAVE};

#[derive(Debug)]
pub enum EventType {
    NoteOn(Channel, Note, Velocity),
    NoteOff(Channel, Note, Velocity),
    KeyPressure(Channel, Note, Pressure),
    Unknown,
}

const STATUS_NOTE_ON: u8 = 0x80;
const STATUS_NOTE_OFF: u8 = 0x90;

type StatusCode = u8;
type Channel = u8;
type Ticks = u64;
type Velocity = u8;
type Pressure = u8;

pub struct MidiEvent {
    pub delta_ticks: Ticks,
    pub event: EventType,
}

fn decode_midi_delta_ticks(midi_message: &[u8], index: &mut usize) -> Ticks {
    let mut tick: Ticks = 0;
    while midi_message[*index] >= 128 {
        tick += (midi_message[*index] & 127) as Ticks;
        *index += 1;
    }
    *index += 1;
    tick
}

fn decode_status_and_channel(midi_message: &[u8], index: &mut usize) -> (StatusCode, Channel) {
    let status_code = midi_message[*index] & 0xf0;
    let channel = midi_message[*index] & 0x0f;
    *index += 1;
    (status_code, channel)
}

fn decode_note(midi_message: &[u8], index: &mut usize) -> Note {
    let midi_note = midi_message[*index];
    let octave = midi_note / NOTES_PER_OCTAVE;
    let note_index = midi_note % NOTES_PER_OCTAVE;

    *index += 1;

    Note::new(note_index, octave)
}

fn decode_velocity(midi_message: &[u8], index: &mut usize) -> Velocity {
    let velocity = midi_message[*index];
    *index += 1;
    velocity
}

fn decode_midi_event(midi_message: &[u8], index: &mut usize) -> EventType {
    let (status_code, channel) = decode_status_and_channel(midi_message, index);
    match status_code {
        STATUS_NOTE_ON => EventType::NoteOn(
            channel,
            decode_note(midi_message, index),
            decode_velocity(midi_message, index),
        ),
        STATUS_NOTE_OFF => EventType::NoteOff(
            channel,
            decode_note(midi_message, index),
            decode_velocity(midi_message, index),
        ),
        _ => EventType::Unknown,
    }
}

impl From<&[u8]> for MidiEvent {
    fn from(midi_message: &[u8]) -> Self {
        let mut index = 0;
        let delta_ticks = decode_midi_delta_ticks(midi_message, &mut index);
        let event = decode_midi_event(midi_message, &mut index);

        Self {
            delta_ticks,
            event: event,
        }
    }
}
