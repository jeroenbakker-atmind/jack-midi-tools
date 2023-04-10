use crate::{composite::Composite, compute::Compute, textures::Textures};

pub struct Context {
    pub textures: Textures,

    pub compute: Compute,

    pub composite: Composite,
}
