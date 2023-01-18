//! Creates a jack module with one midi input. The application prints
//! out all values sent through the input port.

use jack_module::{run_jack_module, Module};
use midi_logger::MidiLogger;

mod midi_logger;

pub fn main() {
    run_jack_module::<MidiLogger, <MidiLogger as Module>::PortDescriptorIdentifierType>();
}
