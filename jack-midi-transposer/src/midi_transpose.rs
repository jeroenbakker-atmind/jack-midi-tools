use std::sync::RwLock;

use jack_module::{Module, PortDescriptor};
use midi_events::Message;

/// Enumeration containing the identifiers of the used ports.
#[derive(Copy, Clone)]
pub enum PortIdentifier {
    /// Port for inputting the midi events that needs to be transposed.
    Input,
    /// Port for outputting the transposed events.
    Output,
}

#[derive(Default)]
pub struct MidiTranspose {
    messages: RwLock<Vec<Message>>,
}

impl Module for MidiTranspose {
    type PortDescriptorIdentifierType = PortIdentifier;

    fn name(&self) -> &'static str {
        "midi-transpose"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>> {
        vec![
            PortDescriptor::midi_in(PortIdentifier::Input, "input"),
            PortDescriptor::midi_out(PortIdentifier::Output, "transposed"),
        ]
    }

    fn handle_midi_in(&self, _port_identifier: &PortIdentifier, midi_message: &Message) {
        self.messages.write().unwrap().push(midi_message.clone());
    }

    fn handle_midi_out(
        &self,
        _port_identifier: &Self::PortDescriptorIdentifierType,
    ) -> Vec<Message> {
        let mut messages = self.messages.write().unwrap();
        let result = messages.clone();
        messages.clear();
        result
    }
}
