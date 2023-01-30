/// Used https://www.recordingblogs.com/wiki/midi-event for reference.
#[cfg(feature = "with-jack")]
pub mod decode_jack;
mod event;
mod message;

pub use event::*;
pub use message::*;
