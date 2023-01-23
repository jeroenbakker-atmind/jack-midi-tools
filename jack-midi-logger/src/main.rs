//! Creates a jack module with one midi input. The application prints
//! out all values sent through the input port.

use gui::Gui;
use jack_module::{run_jack_module, Module};
use midi_logger::MidiLogger;

#[macro_use]
extern crate lazy_static;

extern crate egui;

mod gui;
mod midi_logger;
mod model;

pub fn main() {
    let handle = std::thread::spawn(move || {
        run_jack_module::<MidiLogger, <MidiLogger as Module>::PortDescriptorIdentifierType>();
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Jack Midi Logger",
        native_options,
        Box::new(|cc| Box::new(Gui::new(cc))),
    );

    handle.join().unwrap();
}
