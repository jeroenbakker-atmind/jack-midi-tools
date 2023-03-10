use egui::{Color32, Pos2, Rect, Rounding, Widget};
use music_notes::{ChromaticNote, ChromaticTone};

/// Configuration parameters for customizing the piano widget.
pub struct PianoConfig {
    /// Most left key on the keyboard to draw.
    pub first_key: ChromaticNote,
    /// Most right key on the keyboard to draw.
    ///
    /// Must be equal to or a higher note than `first_key`.
    pub last_key: ChromaticNote,

    /// Ratio between the width and the height of a white key.
    ///
    /// `height = width_of_white_key * white_key_size_factor`.
    pub white_key_size_ratio: f32,

    /// Ratio between the width and the height of a black key.
    ///
    /// `height = width_of_black_key * black_key_size_factor`.
    pub black_key_size_ratio: f32,

    /// Ratio between the rounding and the width of the key.
    ///
    /// `rounding = width_of_key * ratio_rounding`
    pub ratio_rounding: f32,

    /// Ratio between a white and a black key.
    ///
    /// `black_key_width = white_key_width * ratio_width_white_to_black_keys`
    pub ratio_width_white_to_black_keys: f32,

    /// Color to draw white keys.
    pub color_white_key: Color32,
    /// Color to draw black keys.
    pub color_black_key: Color32,
    /// Color to draw white keys that are pressed down.
    pub color_white_pressed_key: Color32,
    /// Color to draw black keys that are pressed down.
    pub color_black_pressed_key: Color32,

    pub black_key_offsets: BlackKeysOffset,
}

trait KeyOffset {
    /// Get the offset to draw a black key based on the group size and the
    /// item of the group being drawn.
    ///
    /// `group_size` needs to be between 1 and 3 (inclusive).
    /// `elem_index` is zero-based inside the group. Therefore needs to be smaller than `group_size`.
    fn black_key_offset(&self, group_size: usize, group_elem: usize, tone: ChromaticTone) -> f32;
}

/// For each group size the offsets to use for black keys.
///
/// Black keys of a piano aren't centered between white keys,
/// but depends on the size of sequential black keys as the
/// position is optimized for the spacing between the black
/// keys.
///
/// This option will provide the offsets to use for each group
/// size. Eg one sequential black key, two sequential black keys
/// and three sequential black keys.
///
/// One sequential black key can occur as the first group of keys
/// don't need to be have 2 or 3 black keys.
pub struct GroupOffset {
    pub offsets_group_size_1: f32,
    pub offsets_group_size_2: [f32; 2],
    pub offsets_group_size_3: [f32; 3],
}

impl KeyOffset for GroupOffset {
    fn black_key_offset(&self, group_size: usize, group_elem: usize, _tone: ChromaticTone) -> f32 {
        assert!(group_size > 0 && group_size < 4);
        assert!(group_elem < group_size);
        match group_size {
            1 => self.offsets_group_size_1,
            2 => self.offsets_group_size_2[group_elem],
            3 => self.offsets_group_size_3[group_elem],
            _ => unreachable!(),
        }
    }
}
pub struct ToneOffset {
    pub offset_csharp: f32,
    pub offset_dsharp: f32,
    pub offset_fsharp: f32,
    pub offset_gsharp: f32,
    pub offset_asharp: f32,
}
impl KeyOffset for ToneOffset {
    fn black_key_offset(&self, _group_size: usize, _group_elem: usize, tone: ChromaticTone) -> f32 {
        match tone {
            ChromaticTone::CSharp => self.offset_csharp,
            ChromaticTone::DSharp => self.offset_dsharp,
            ChromaticTone::FSharp => self.offset_fsharp,
            ChromaticTone::GSharp => self.offset_gsharp,
            ChromaticTone::ASharp => self.offset_asharp,
            _ => unreachable!(),
        }
    }
}
pub struct NoOffset {}
impl KeyOffset for NoOffset {
    fn black_key_offset(
        &self,
        _group_size: usize,
        _group_elem: usize,
        _tone: ChromaticTone,
    ) -> f32 {
        0.0
    }
}

pub enum BlackKeysOffset {
    NoOffset(NoOffset),
    ToneOffset(ToneOffset),
    GroupOffset(GroupOffset),
}
impl KeyOffset for BlackKeysOffset {
    fn black_key_offset(&self, group_size: usize, group_elem: usize, tone: ChromaticTone) -> f32 {
        match self {
            Self::NoOffset(n) => n.black_key_offset(group_size, group_elem, tone),
            Self::GroupOffset(n) => n.black_key_offset(group_size, group_elem, tone),
            Self::ToneOffset(n) => n.black_key_offset(group_size, group_elem, tone),
        }
    }
}

/// Create the default configuration for the piano keys widget.
///
/// Configuration is based on a 88-key piano.
impl Default for PianoConfig {
    fn default() -> Self {
        Self {
            first_key: ChromaticNote::new(ChromaticTone::A, 0),
            last_key: ChromaticNote::new(ChromaticTone::C, 8),
            white_key_size_ratio: 6.0,
            black_key_size_ratio: 3.5,
            ratio_width_white_to_black_keys: 0.8,
            ratio_rounding: 0.1,
            color_black_key: Color32::BLACK,
            color_white_key: Color32::WHITE,
            color_white_pressed_key: Color32::GRAY,
            color_black_pressed_key: Color32::DARK_GRAY,
            black_key_offsets: BlackKeysOffset::ToneOffset(ToneOffset {
                offset_csharp: -0.1,
                offset_dsharp: 0.1,
                offset_fsharp: -0.2,
                offset_gsharp: 0.0,
                offset_asharp: 0.2,
            }),
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
    /// Is the given piano key a white key.
    fn is_white_key(&self) -> bool;
    /// Will the given piano key be followed by a black key.
    fn is_followed_by_black_key(&self) -> bool;
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
    fn is_followed_by_black_key(&self) -> bool {
        match self.tone {
            ChromaticTone::A
            | ChromaticTone::C
            | ChromaticTone::D
            | ChromaticTone::F
            | ChromaticTone::G => true,
            ChromaticTone::E
            | ChromaticTone::B
            | ChromaticTone::ASharp
            | ChromaticTone::CSharp
            | ChromaticTone::DSharp
            | ChromaticTone::FSharp
            | ChromaticTone::GSharp => false,
        }
    }
}

/// Helper struct to determine the offset of sequential black keys
#[derive(Default)]
struct BlackKeys {
    /// The group of sequential keys.
    pub black_keys: Vec<ChromaticNote>,
    /// The white key just before the group of black keys.
    pub starting_white_key: ChromaticNote,
    /// Index given to the start_white_key. The index is the index of the white key of the piano being drawn.
    pub starting_index: usize,
}

impl BlackKeys {
    /// Does this group already have some black keys?
    ///
    /// Groups without any black keys should not be considered valid groups.
    fn is_empty(&self) -> bool {
        self.black_keys.is_empty()
    }

    /// Generate multiple groups for each set of sequential black keys.
    fn groups_from(white_keys: &[(usize, ChromaticNote)]) -> Vec<BlackKeys> {
        let mut groups = Vec::new();
        let mut current_group = BlackKeys::default();
        for i in 0..white_keys.len() {
            let item = &white_keys[i];
            if item.1.is_followed_by_black_key() {
                // Add item to current group. Set starting key when the group doesn't have any keys yet.
                if current_group.is_empty() {
                    current_group.starting_index = item.0;
                    current_group.starting_white_key = item.1;
                }
                current_group.black_keys.push(item.1 + 1);
            } else if !current_group.is_empty() {
                // Group is complete and should be added to the result.
                groups.push(current_group);
                current_group = BlackKeys::default();
            }
        }

        // remove the last black key as it would be drawn right to the last key.
        if !current_group.is_empty() {
            current_group
                .black_keys
                .remove(current_group.black_keys.len() - 1);
        }
        if !current_group.is_empty() {
            groups.push(current_group);
        }

        groups
    }
}

impl Widget for PianoKeys {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let white_keys: Vec<(usize, ChromaticNote)> = (i32::from(self.config.first_key)
            ..=i32::from(self.config.last_key))
            .map(|i| ChromaticNote::from(i))
            .filter(|n| n.is_white_key())
            .enumerate()
            .collect();
        let number_of_white_keys = white_keys.len();
        let canvas_width = ui.available_width();
        let white_key_width = canvas_width / number_of_white_keys as f32;
        let black_key_width = white_key_width * self.config.ratio_width_white_to_black_keys;
        let rounding = white_key_width * self.config.ratio_rounding;
        let white_key_height = white_key_width * self.config.white_key_size_ratio;
        let black_key_height = white_key_width * self.config.black_key_size_ratio;

        let response = ui.allocate_response(
            egui::vec2(canvas_width, white_key_height),
            egui::Sense::hover(),
        );
        let painter = ui.painter();

        // Draw white keys.
        white_keys.iter().for_each(|(key_number, note)| {
            let fill_color = if self.pressed_keys.contains(&note) {
                self.config.color_white_pressed_key
            } else {
                self.config.color_white_key
            };
            let key = *key_number as f32;
            painter.rect_filled(
                Rect {
                    min: Pos2 {
                        x: key * white_key_width + 1.0,
                        y: response.rect.min.y,
                    },
                    max: Pos2 {
                        x: (key + 1.0) * white_key_width - 1.0,
                        y: response.rect.min.y + white_key_height,
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

        // Draw black keys.
        let key_groups = BlackKeys::groups_from(&white_keys);
        key_groups.iter().for_each(|group| {
            group
                .black_keys
                .iter()
                .enumerate()
                .for_each(|(key_in_group_index, black_key)| {
                    let next_white_index = group.starting_index + key_in_group_index;
                    let fill_color = if self.pressed_keys.contains(&black_key) {
                        self.config.color_black_pressed_key
                    } else {
                        self.config.color_black_key
                    };
                    let key = (next_white_index + 1) as f32
                        + self.config.black_key_offsets.black_key_offset(
                            group.black_keys.len(),
                            key_in_group_index,
                            black_key.tone,
                        );
                    painter.rect_filled(
                        Rect {
                            min: Pos2 {
                                x: key * white_key_width - black_key_width * 0.5,
                                y: response.rect.min.y,
                            },
                            max: Pos2 {
                                x: key * white_key_width + black_key_width * 0.5,
                                y: response.rect.min.y + black_key_height,
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
        });

        response
    }
}
