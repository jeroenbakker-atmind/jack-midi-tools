use jack::RawMidi;
use jack_module::{Module, PortDescriptor};
use midi_events::{Event, MidiMessage};

/// Enumeration containing the identifiers of the used ports.
#[derive(Copy, Clone)]
pub enum PortIdentifier {
    /// Identifier for the midi port inputting the midi events that will be logged.
    InputPort,
}

#[derive(Default)]
pub struct MidiLogger {}

impl Module for MidiLogger {
    type PortDescriptorIdentifierType = PortIdentifier;

    fn name(&self) -> &'static str {
        "jack-midi-logger"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>> {
        vec![PortDescriptor::midi_in(
            PortIdentifier::InputPort,
            "midi-input",
        )]
    }

    fn handle_midi_in(&self, _port_identifier: &PortIdentifier, midi_event: &RawMidi) {
        let event = Event::from(midi_event.bytes);
        println!("{:?}", event);
    }
}
