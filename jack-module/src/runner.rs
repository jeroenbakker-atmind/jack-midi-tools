use std::io::stdin;

use jack::{MidiIn, Port};

use crate::{Module, PortType};

pub trait Runner<T>
where
    T: Module + Sync,
{
    fn run(&self, module: &T);
    fn wait(&self) {
        println!("Press any key to quit");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).ok();
    }
}

#[derive(Default)]
pub struct JackRunner {}
impl<T> Runner<T> for JackRunner
where
    T: Module + Sync,
{
    fn run(&self, module: &T) {
        let (client, _status) =
            jack::Client::new(module.name(), jack::ClientOptions::NO_START_SERVER).unwrap();

        //let (sender, receiver) = sync_channel(64);

        let midi_in_ports: Vec<(&'static str, Port<MidiIn>)> = module
            .port_descriptors()
            .iter()
            .filter(|pd| pd.is_input() && pd.port_type() == PortType::Midi)
            .map(|pd| {
                let port = client.register_port(pd.name(), MidiIn::default()).unwrap();
                (pd.name(), port)
            })
            .collect();

        let cback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            for (midi_in_port_name, port) in &midi_in_ports {
                let midi_events = port.iter(ps);
                for raw_event in midi_events {
                    module.handle_midi_in(midi_in_port_name, &raw_event);
                }
                //let _ = sender.try_send(midi_events);
            }
            jack::Control::Continue
        };

        /*  let active_client = client
        .activate_async((), jack::ClosureProcessHandler::new(cback))
        .unwrap();*/
    }
}
