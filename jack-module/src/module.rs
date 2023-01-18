use jack::RawMidi;

use crate::PortDescriptor;

pub trait Module {
    fn name(&self) -> &'static str;
    fn port_descriptors(&self) -> Vec<PortDescriptor>;
    fn handle_midi_in(&self, port_name: &'static str, midi_event: &RawMidi);
}
