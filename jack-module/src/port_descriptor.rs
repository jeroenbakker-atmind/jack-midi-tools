#[derive(Clone, Copy, PartialEq)]
pub enum PortType {
    Midi,
}

pub struct PortDescriptor {
    name: &'static str,
    is_input: bool,
    port_type: PortType,
}

impl PortDescriptor {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_input(&self) -> bool {
        self.is_input
    }

    pub fn port_type(&self) -> PortType {
        self.port_type
    }

    pub fn midi_in(name: &'static str) -> Self {
        PortDescriptor {
            name,
            is_input: true,
            port_type: PortType::Midi,
        }
    }

    pub fn midi_out(name: &'static str) -> Self {
        PortDescriptor {
            name,
            is_input: false,
            port_type: PortType::Midi,
        }
    }
}
