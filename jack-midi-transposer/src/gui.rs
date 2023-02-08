use std::sync::RwLock;

#[derive()]
pub struct Gui {}

impl Default for Gui {
    fn default() -> Self {
        Self {}
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
        egui::CentralPanel::default().show(ctx, |ui| ui.label("Transpose"));
    }
}
