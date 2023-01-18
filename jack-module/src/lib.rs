mod module;
mod port_descriptor;
mod runner;

pub use module::*;
pub use port_descriptor::*;
use runner::{JackRunner, Runner};

pub fn run_jack_module<T>()
where
    T: Module + Send + 'static + Default,
{
    let module = T::default();
    run_as_jack_module(module);
}

pub fn run_as_jack_module<T>(module: T)
where
    T: Module + Send + 'static,
{
    let runtime = JackRunner::new(&module);
    runtime.run(module);
}
