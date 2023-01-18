//! Creates a jack midi input and output ports. The application prints
//! out all values sent to it through the input port. It also sends a
//! Note On and Off event, once every cycle, on the output port.

use jack_module::{run_jack_module, Module};
use midi_logger::MidiLogger;

mod midi_logger;

pub fn main() {
    run_jack_module::<MidiLogger, <MidiLogger as Module>::PortDescriptorIdentifierType>();
}
