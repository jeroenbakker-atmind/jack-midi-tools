use std::sync::RwLock;

use egui_widgets_music::{
    channel_selector::ChannelSelector,
    piano_keys::{PianoConfig, PianoKeys},
};

use crate::model::APP_MODEL;

#[derive()]
pub struct Gui {
    /// Selected midi channel to show. Midi channels are one based, 0 is not allowed.
    selected_channel: u8,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            selected_channel: 1,
        }
    }
}

lazy_static! {
    static ref EGUI_CONTEXT: RwLock<Option<egui::Context>> = RwLock::default();
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        *EGUI_CONTEXT.write().unwrap() = Some(cc.egui_ctx.clone());
        Self::default()
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let device_state = APP_MODEL.channels.read();
            ui.horizontal(|ui| {
                ui.label("Channel: ");

                ui.add(ChannelSelector::new(
                    &mut self.selected_channel,
                    device_state.num_channels() as u8,
                ));
            });

            let channel = &device_state[self.selected_channel];
            let pressed_keys = channel
                .active_notes
                .iter()
                .map(|note_id| device_state.note_by_note_index(*note_id))
                .map(|note_state| note_state.note)
                .collect();
            ui.add(PianoKeys::new(PianoConfig::default(), pressed_keys));
        });
    }
}

pub fn refresh() {
    if let Some(context) = EGUI_CONTEXT.read().unwrap().as_ref() {
        context.request_repaint();
    }
}
