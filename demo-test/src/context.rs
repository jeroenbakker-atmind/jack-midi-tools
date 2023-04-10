use wgpu::{ComputePipeline, Texture, TextureView, ShaderModule};

pub struct Compute {
    pub shader: ShaderModule,
    pub pipeline: ComputePipeline,
}
pub struct Textures {
    pub output_texture: Texture,
    pub output_texture_view: TextureView,
}

pub struct Context {
    pub compute: Compute,
    pub textures: Textures,
}
