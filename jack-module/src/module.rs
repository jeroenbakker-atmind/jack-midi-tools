use jack::RawMidi;

use crate::{
    runner::{JackRunner, Runner},
    PortDescriptor,
};

pub trait Module {
    fn name(&self) -> &'static str;
    fn port_descriptors(&self) -> Vec<PortDescriptor>;
    fn handle_midi_in(&self, port_name: &'static str, midi_event: &RawMidi);

    fn run(&self)
    where
        Self: Sized + Sync,
    {
        let runner = JackRunner::default();
        runner.run(self);
    }
}
