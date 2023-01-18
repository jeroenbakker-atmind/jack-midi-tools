use jack::RawMidi;
use jack_module::{Module, PortDescriptor};

#[derive(Copy, Clone)]
pub enum MidiLoggerPort {
    /// Midi port that is being logged.
    MidiInput,
}

#[derive(Default)]
pub struct MidiLogger {}

impl Module for MidiLogger {
    type PortDescriptorIdentifierType = MidiLoggerPort;
    
    fn name(&self) -> &'static str {
        "jack-midi-logger"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>> {
        vec![PortDescriptor::midi_in(
            MidiLoggerPort::MidiInput,
            "midi-input",
        )]
    }

    fn handle_midi_in(&self, _port_identifier: &MidiLoggerPort, midi_event: &RawMidi) {
        println!("{:?}", midi_event);
    }
}
