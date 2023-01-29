use egui::{Color32, Rect, Rounding, Widget};
use music_notes::{ChromaticNote, ChromaticTone};

pub struct PianoConfig {
    pub first_key: ChromaticNote,
    pub last_key: ChromaticNote,
    pub white_key_size_ratio: f32,
    pub ratio_rounding: f32,
    pub color_white_key: Color32,
    pub color_black_key: Color32,
    pub color_pressed_key: Color32,
}

impl Default for PianoConfig {
    fn default() -> Self {
        Self {
            first_key: ChromaticNote::new(ChromaticTone::A, 0),
            last_key: ChromaticNote::new(ChromaticTone::C, 8),
            white_key_size_ratio: 6.0,
            ratio_rounding: 0.1,
            color_black_key: Color32::BLACK,
            color_white_key: Color32::WHITE,
            color_pressed_key: Color32::GRAY,
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
        let number_of_white_keys = (i32::from(self.config.first_key)
            ..=i32::from(self.config.last_key))
            .map(|i| ChromaticNote::from(i))
            .filter(|k| k.is_white_key())
            .count();
        let canvas_width = ui.available_width();
        let white_key_width = canvas_width / number_of_white_keys as f32;
        let rounding = white_key_width * self.config.ratio_rounding;
        let white_key_height = white_key_width * self.config.white_key_size_ratio;
        let painter = ui.painter();

        // Draw white keys.
        (i32::from(self.config.first_key)..=i32::from(self.config.last_key))
            .map(|i| ChromaticNote::from(i))
            .filter(|n| n.is_white_key())
            .enumerate()
            .for_each(|(key_number, note)| {
                let fill_color = if self.pressed_keys.contains(&note) {
                    self.config.color_pressed_key
                } else {
                    self.config.color_white_key
                };
                let key = key_number as f32;
                painter.rect_filled(
                    Rect {
                        min: egui::Pos2 {
                            x: key * white_key_width + 1.0,
                            y: 50.0,
                        },
                        max: egui::Pos2 {
                            x: (key + 1.0) * white_key_width - 1.0,
                            y: 50.0 + white_key_height,
                        },
                    },
                    Rounding {
                        ne: 0.0,
                        nw: 0.0,
                        se: rounding,
                        sw: rounding,
                    },
                    fill_color,
                );
            });

        // TODO: paint black keys.

        // Dummy label as we need to return a response.
        // TODO: we should construct a valid response.
        let response = ui.label("");
        response
    }
}
