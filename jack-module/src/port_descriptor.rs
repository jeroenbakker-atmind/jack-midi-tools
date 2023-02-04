#[derive(Clone, Copy, PartialEq)]
pub enum PortType {
    Midi,
}

pub struct PortDescriptor<I>
where
    I: Copy,
{
    identifier: I,
    name: &'static str,
    is_input: bool,
    port_type: PortType,
}

impl<I> PortDescriptor<I>
where
    I: Copy,
{
    pub fn identifier(&self) -> I {
        self.identifier
    }
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_input(&self) -> bool {
        self.is_input
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }

    pub fn port_type(&self) -> PortType {
        self.port_type
    }

    pub fn midi_in(identifier: I, name: &'static str) -> Self {
        PortDescriptor {
            identifier,
            name,
            is_input: true,
            port_type: PortType::Midi,
        }
    }

    pub fn midi_out(identifier: I, name: &'static str) -> Self {
        PortDescriptor {
            identifier,
            name,
            is_input: false,
            port_type: PortType::Midi,
        }
    }
}
