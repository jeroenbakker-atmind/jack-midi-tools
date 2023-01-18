use jack::RawMidi;
use jack_module::{Module, PortDescriptor};

#[derive(Default)]
pub struct MidiLogger {}

impl Module for MidiLogger {
    fn name(&self) -> &'static str {
        "jack-midi-logger"
    }

    fn port_descriptors(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::midi_in("midi-input")]
    }

    fn handle_midi_in(&self, _port_name: &'static str, midi_event: &RawMidi) {
        println!("{:?}", midi_event);
    }
}
