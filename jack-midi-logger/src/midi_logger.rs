use jack_module::{Module, PortDescriptor};
use midi_events::Message;

use crate::{gui::refresh, model::APP_MODEL};

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
        "midi-logger"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>> {
        vec![PortDescriptor::midi_in(
            PortIdentifier::InputPort,
            "midi-input",
        )]
    }

    fn handle_midi_in(&self, _port_identifier: &PortIdentifier, midi_message: &Message) {
        APP_MODEL.channels.write().apply_event(&midi_message.event);
        refresh();
        println!("{:?}", midi_message.event);
    }

    fn handle_midi_out(
        &self,
        _port_identifier: &Self::PortDescriptorIdentifierType,
    ) -> Vec<Message> {
        unimplemented!()
    }
}
