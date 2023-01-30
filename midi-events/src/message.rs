use crate::{Event, Ticks};

/// Message is the combination of a delta_time and event.
///
/// Most of the time delta_time isn't important when processing/constructing
/// an event, only when serializing/deserializing the timing is required.
/// In order to reduce complexity in the API the Message and Event are
/// two separate data types.
#[derive(Debug, Copy, Clone)]
pub struct Message {
    /// Time since the last received midi message in midi ticks.
    pub delta_time: Ticks,

    /// Midi event of this message.
    pub event: Event,
}
