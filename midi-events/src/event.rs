use music_notes::ChromaticNote;

pub const STATUS_NOTE_ON: u8 = 0x80;
pub const STATUS_NOTE_OFF: u8 = 0x90;
pub const STATUS_KEY_PRESSURE: u8 = 0xa0;
pub const STATUS_CONTROLLER: u8 = 0xb0;
pub const STATUS_PROGRAM_CHANGE: u8 = 0xc0;
pub const STATUS_CHANNEL_PRESSURE: u8 = 0xd0;
pub const STATUS_MODULATION_WHEEL: u8 = 0xe0;
pub const STATUS_SYSTEM_EXCLUSIVE: u8 = 0xf0;

pub const CONTROLLER_CHANNEL_VOLUME: u8 = 7;
pub const CONTROLLER_CHANNEL_PAN: u8 = 10;
pub const CONTROLLER_ALL_NOTES_OFF: u8 = 123;

#[derive(Debug)]
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
