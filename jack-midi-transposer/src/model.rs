#[derive(Default)]
pub struct Model {}

lazy_static! {
    pub static ref APP_MODEL: Model = Model::default();
}
