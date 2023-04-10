use std::borrow::Cow;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, CommandEncoder, ComputePassDescriptor, ComputePipeline, Device,
    PipelineLayoutDescriptor, ShaderModule, ShaderSource, ShaderStages, StorageTextureAccess,
    TextureFormat, TextureViewDimension,
};

use crate::context::Context;

pub struct Compute {
    pub shader: ShaderModule,
    pub pipeline: ComputePipeline,
}

pub fn init_compute(device: &Device) -> Compute {
    let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("compute.wgsl"))),
    });

    let bind_group_layout_descriptor = BindGroupLayoutDescriptor {
        label: Some("texture bind group layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,

            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        }],
    };
    let bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

    let compute_pipeline_layout_descriptor = PipelineLayoutDescriptor {
        label: Some("Compute pipeline layout descriptor"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    };
    let compute_pipeline_layout =
        device.create_pipeline_layout(&compute_pipeline_layout_descriptor);

    let compute_pipeline_descriptor = wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline Descriptor"),
        layout: Some(&compute_pipeline_layout),
        module: &compute_shader,
        entry_point: "compute_main",
    };
    let compute_pipeline = device.create_compute_pipeline(&compute_pipeline_descriptor);

    Compute {
        shader: compute_shader,
        pipeline: compute_pipeline,
    }
}

pub fn dispatch_compute(device: &Device, encoder: &mut CommandEncoder, context: &Context) {
    let texture_bind_group_descriptor = BindGroupDescriptor {
        label: Some("Texture bind group"),
        layout: &context.compute.pipeline.get_bind_group_layout(0),
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&context.textures.output_texture_view),
        }],
    };

    let texture_bind_group = device.create_bind_group(&texture_bind_group_descriptor);
    let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
        label: Some("Compute pass"),
    });
    let size = context.textures.output_texture.size();
    compute_pass.set_pipeline(&context.compute.pipeline);
    compute_pass.set_bind_group(0, &texture_bind_group, &[]);
    compute_pass.dispatch_workgroups(size.width / 16, size.height / 16, 1);
}
