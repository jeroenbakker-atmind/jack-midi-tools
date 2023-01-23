use std::sync::RwLock;

use crate::model::APP_MODEL;

#[derive(Default)]
pub struct Gui {}

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
            let channel = &device_state[1];
            for active_note_id in &channel.active_notes {
                let note = device_state.note_by_note_index(*active_note_id);
                ui.label(format!("{:?}", note.note));
            }
        });
    }
}

pub fn refresh() {
    if let Some(context) = EGUI_CONTEXT.read().unwrap().as_ref() {
        context.request_repaint();
    }
}
