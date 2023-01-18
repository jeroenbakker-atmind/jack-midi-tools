use jack::RawMidi;

use crate::PortDescriptor;

pub trait Module {
    /// When using multiple ports use this type as identifier between the ports.
    type PortDescriptorIdentifierType: Copy + Sized;

    /// Name of the module. The name will be used during registration and visible inside jack tools.
    fn name(&self) -> &'static str;

    /// Define the list of inputs and outputs that will be registered.
    fn port_descriptors(&self) -> Vec<PortDescriptor<Self::PortDescriptorIdentifierType>>;

    /// Callback function where midi in events will be send to.
    fn handle_midi_in(
        &self,
        port_identifier: &Self::PortDescriptorIdentifierType,
        midi_event: &RawMidi,
    );
}
