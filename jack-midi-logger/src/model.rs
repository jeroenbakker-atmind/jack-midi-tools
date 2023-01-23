use egui::mutex::RwLock;
use midi_device::DeviceState;

pub struct Model {
    pub channels: RwLock<DeviceState>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            channels: RwLock::new(DeviceState::new(16, 128)),
        }
    }
}

lazy_static! {
    pub static ref APP_MODEL: Model = Model::default();
}
