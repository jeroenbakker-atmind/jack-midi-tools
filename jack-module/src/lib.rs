mod module;
mod port_descriptor;
mod runner;

pub use module::*;
pub use port_descriptor::*;
use runner::{JackRunner, Runner};

pub fn run_jack_module<T, I>()
where
    T: Module<PortDescriptorIdentifierType = I> + Send + 'static + Default,
    I: Copy + Send + 'static,
{
    let module = T::default();
    run_as_jack_module(module);
}

pub fn run_as_jack_module<T, I>(module: T)
where
    T: Module<PortDescriptorIdentifierType = I> + Send + 'static,
    I: Copy + Send + 'static,
{
    let runtime = JackRunner::new(&module);
    runtime.run(module);
}
