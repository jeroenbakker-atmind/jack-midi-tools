use crate::NoteStateId;

#[derive(Debug, Default, Clone)]
pub struct ChannelState {
    pub active_notes: Vec<NoteStateId>,
}
