use wgpu::{Device, Extent3d, TextureDescriptor, TextureFormat, TextureViewDescriptor, Texture, TextureView};
use winit::dpi::PhysicalSize;

pub struct Textures {
    pub output_texture: Texture,
    pub output_texture_view: TextureView,
}

pub fn init_textures(device: &Device, size: PhysicalSize<u32>) -> Textures {
    let texture_size = Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let output_texture = device.create_texture(&TextureDescriptor {
        label: Some("output texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[TextureFormat::Rgba8Unorm],
    });

    let output_texture_view = output_texture.create_view(&TextureViewDescriptor::default());

    Textures {
        output_texture,
        output_texture_view,
    }
}
