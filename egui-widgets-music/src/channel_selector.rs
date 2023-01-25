use std::num;

use egui::{Slider, Widget};

pub struct ChannelSelector<'a> {
    num_channels: u8,
    selected_channel: &'a mut u8,
}

impl<'a> ChannelSelector<'a> {
    pub fn new(selected_channel: &'a mut u8, num_channels: u8) -> Self {
        Self {
            num_channels,
            selected_channel,
        }
    }
}

impl<'a> Widget for ChannelSelector<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(Slider::new(
            self.selected_channel,
            1..=self.num_channels as u8,
        ))
    }
}
