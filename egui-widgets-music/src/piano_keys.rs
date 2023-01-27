use egui::{Color32, Rect, Widget};
use music_notes::{ChromaticNote, ChromaticTone};

pub struct PianoKeys {
    first_key: ChromaticNote,
    last_key: ChromaticNote,
    pressed_keys: Vec<ChromaticNote>,
}

impl PianoKeys {
    pub fn new(pressed_keys: Vec<ChromaticNote>) -> Self {
        Self {
            first_key: ChromaticNote::new(ChromaticTone::C, 2),
            last_key: ChromaticNote::new(ChromaticTone::C, 7),
            pressed_keys,
        }
    }
}

impl Widget for PianoKeys {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        //for key in self.first_key..=self.last_key {}
        let painter = ui.painter();
        for i in 0..64 {
            let key = i as f32;
            let key_width = 10.0;
            let rect = Rect {
                min: egui::Pos2 {
                    x: key * key_width + 1.0,
                    y: 50.0,
                },
                max: egui::Pos2 {
                    x: (key + 1.0) * key_width - 1.0,
                    y: 100.0,
                },
            };
            ui.painter().rect_filled(rect, 5.0, Color32::WHITE);
        }

        let mut response = ui.label("");
        for key in &self.pressed_keys {
            response = ui.label(format!("{:?}", key));
        }
        response
    }
}
