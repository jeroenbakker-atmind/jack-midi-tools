use std::sync::RwLock;

use jack::RawMidi;
use jack_module::{Module, PortDescriptor};
use midi_device::DeviceState;
use midi_events::Event;

/// Enumeration containing the identifiers of the used ports.
#[derive(Copy, Clone)]
pub enum PortIdentifier {
    /// Identifier for the midi port inputting the midi events that will be logged.
    InputPort,
}

#[derive()]
pub struct MidiLogger {
    channels: RwLock<DeviceState>,
}

impl Default for MidiLogger {
    fn default() -> Self {
        MidiLogger {
            channels: RwLock::new(DeviceState::new(16, 128)),
        }
    }
}

impl Module for MidiLogger {
    type PortDescriptorIdentifierType = PortIdentifier;

    fn name(&self) -> &'static str {
        "midi-logger"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>> {
        vec![PortDescriptor::midi_in(
            PortIdentifier::InputPort,
            "midi-input",
        )]
    }

    fn handle_midi_in(&self, _port_identifier: &PortIdentifier, midi_event: &RawMidi) {
        let event = Event::from(midi_event.bytes);
        let mut channels = self.channels.write().unwrap();
        channels.apply_event(&event);

        println!("{:#?}", channels);
    }
}
