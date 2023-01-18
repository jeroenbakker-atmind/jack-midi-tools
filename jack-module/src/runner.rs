use std::io::stdin;

use jack::{Client, MidiIn, Port, ProcessHandler};

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

pub struct JackModuleContext<T>
where
    T: Module,
{
    module: T,
    midi_in_ports: Vec<(&'static str, Port<MidiIn>)>,
}

impl<T> JackModuleContext<T>
where
    T: Module,
{
    fn new(jack_client: &Client, module: T) -> Self {
        let midi_in_ports: Vec<(&'static str, Port<MidiIn>)> = module
            .port_descriptors()
            .iter()
            .filter(|pd| pd.is_input() && pd.port_type() == PortType::Midi)
            .map(|pd| {
                let port = jack_client
                    .register_port(pd.name(), MidiIn::default())
                    .unwrap();
                (pd.name(), port)
            })
            .collect();

        Self {
            midi_in_ports,
            module,
        }
    }
}

impl<T> ProcessHandler for JackModuleContext<T>
where
    T: Module + Send,
{
    fn process(&mut self, _: &Client, process_scope: &jack::ProcessScope) -> jack::Control {
        for (midi_in_port_name, port) in &self.midi_in_ports {
            let midi_events = port.iter(process_scope);
            for raw_event in midi_events {
                self.module.handle_midi_in(midi_in_port_name, &raw_event);
            }
        }
        jack::Control::Continue
    }
}

pub struct JackRunner {
    jack_client: Client,
}

impl<T> Runner<T> for JackRunner
where
    T: Module + Send + 'static,
{
    type Runtime = JackModuleContext<T>;

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
