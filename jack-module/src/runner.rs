use std::io::stdin;

use jack::{Client, MidiIn, Port, ProcessHandler};
use midi_events::Message;

use crate::{Module, PortType};

pub trait Runner<T>
where
    T: Module,
{
    type Runtime;

    fn new(module: &T) -> Self;
    fn init(&self, module: T) -> Self::Runtime;
    fn run(self, module: T);
}

pub struct JackModuleContext<T, I>
where
    T: Module,
    I: Copy,
{
    module: T,
    midi_in_ports: Vec<(I, Port<MidiIn>)>,
}

impl<T, I> JackModuleContext<T, I>
where
    I: Copy,
    T: Module<PortDescriptorIdentifierType = I>,
{
    fn new(jack_client: &Client, module: T) -> Self {
        let midi_in_ports: Vec<(I, Port<MidiIn>)> = module
            .port_descriptors()
            .iter()
            .filter(|pd| pd.is_input() && pd.port_type() == PortType::Midi)
            .map(|pd| {
                let port = jack_client
                    .register_port(pd.name(), MidiIn::default())
                    .unwrap();
                (pd.identifier(), port)
            })
            .collect();

        Self {
            midi_in_ports,
            module,
        }
    }
}

impl<T, I> ProcessHandler for JackModuleContext<T, I>
where
    T: Module<PortDescriptorIdentifierType = I> + Send,
    I: Copy + Send,
{
    fn process(&mut self, _: &Client, process_scope: &jack::ProcessScope) -> jack::Control {
        for (port_identifier, port) in &self.midi_in_ports {
            let midi_events = port.iter(process_scope);
            for raw_event in midi_events {
                let message = Message::from(&raw_event);
                self.module.handle_midi_in(port_identifier, &message);
            }
        }
        jack::Control::Continue
    }
}

pub struct JackRunner {
    jack_client: Client,
}

impl<T, I> Runner<T> for JackRunner
where
    T: Module<PortDescriptorIdentifierType = I> + Send + 'static,
    I: Copy + Send + 'static,
{
    type Runtime = JackModuleContext<T, I>;

    fn new(module: &T) -> Self {
        let (jack_client, _status) =
            jack::Client::new(module.name(), jack::ClientOptions::NO_START_SERVER).unwrap();
        Self { jack_client }
    }

    fn init(&self, module: T) -> Self::Runtime {
        Self::Runtime::new(&self.jack_client, module)
    }

    fn run(self, module: T) {
        let runtime = Self::Runtime::new(&self.jack_client, module);
        let active_client = self.jack_client.activate_async((), runtime).unwrap();
        println!("Press enter to quit");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).ok();
        drop(active_client);
    }
}
