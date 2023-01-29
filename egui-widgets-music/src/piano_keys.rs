use egui::{Color32, Rect, Rounding, Widget};
use music_notes::{ChromaticNote, ChromaticTone};

pub struct PianoConfig {
    pub first_key: ChromaticNote,
    pub last_key: ChromaticNote,
    pub white_key_size_ratio: f32,
    pub ratio_rounding: f32,
}
impl Default for PianoConfig {
    fn default() -> Self {
        Self {
            first_key: ChromaticNote::new(ChromaticTone::A, 0),
            last_key: ChromaticNote::new(ChromaticTone::C, 8),
            white_key_size_ratio: 6.0,
            ratio_rounding: 0.1,
        }
    }
}

pub struct PianoKeys {
    config: PianoConfig,
    pressed_keys: Vec<ChromaticNote>,
}

impl PianoKeys {
    pub fn new(config: PianoConfig, pressed_keys: Vec<ChromaticNote>) -> Self {
        Self {
            config,
            pressed_keys,
        }
    }
}

trait PianoKey {
    fn is_white_key(&self) -> bool;
    fn is_black_key(&self) -> bool {
        !self.is_white_key()
    }
}

impl PianoKey for ChromaticNote {
    fn is_white_key(&self) -> bool {
        match self.tone {
            ChromaticTone::A
            | ChromaticTone::B
            | ChromaticTone::C
            | ChromaticTone::D
            | ChromaticTone::E
            | ChromaticTone::F
            | ChromaticTone::G => true,
            ChromaticTone::ASharp
            | ChromaticTone::CSharp
            | ChromaticTone::DSharp
            | ChromaticTone::FSharp
            | ChromaticTone::GSharp => false,
        }
    }
}

impl Widget for PianoKeys {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        //for key in self.first_key..=self.last_key {}
        let number_of_white_keys = (i32::from(self.config.first_key)
            ..=i32::from(self.config.last_key))
            .map(|i| ChromaticNote::from(i))
            .filter(|k| k.is_white_key())
            .count();
        let width = ui.available_width();
        let key_width = width / number_of_white_keys as f32;
        let rounding = key_width * self.config.ratio_rounding;
        let key_height = key_width * self.config.white_key_size_ratio;
        let painter = ui.painter();
        let mut key_number = 0;
        for i in i32::from(self.config.first_key)..=i32::from(self.config.last_key) {
            let note = ChromaticNote::from(i);
            if note.is_black_key() {
                continue;
            }
            let key = key_number as f32;
            let rect = Rect {
                min: egui::Pos2 {
                    x: key * key_width + 1.0,
                    y: 50.0,
                },
                max: egui::Pos2 {
                    x: (key + 1.0) * key_width - 1.0,
                    y: 50.0 + key_height,
                },
            };

            let fill_color = if self.pressed_keys.contains(&note) {
                Color32::GRAY
            } else if note.is_white_key() {
                Color32::WHITE
            } else {
                Color32::BLACK
            };

            painter.rect_filled(
                rect,
                Rounding {
                    ne: 0.0,
                    nw: 0.0,
                    se: rounding,
                    sw: rounding,
                },
                fill_color,
            );
            key_number += 1;
        }

        let mut response = ui.label("");
        for key in &self.pressed_keys {
            response = ui.label(format!("{:?}", key));
        }
        response
    }
}
