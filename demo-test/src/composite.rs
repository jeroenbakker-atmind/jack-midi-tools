use std::borrow::Cow;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, CommandEncoder, Device, FragmentState, RenderPipeline,
    RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat, TextureSampleType,
    TextureView, TextureViewDimension, VertexState,
};

use crate::context::Context;

pub struct Composite {
    pub shader: ShaderModule,
    pub pipeline: RenderPipeline,
}

pub fn init_composite(device: &Device, swapchain_format: &TextureFormat) -> Composite {
    let render_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Copy result to screen shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });
    let vertex_state = VertexState {
        module: &render_shader,
        entry_point: "vertex_main",
        buffers: &[],
    };
    let render_bind_group_layout_descriptor = BindGroupLayoutDescriptor {
        label: Some("texture bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    };
    let render_bind_group_layout =
        device.create_bind_group_layout(&render_bind_group_layout_descriptor);
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&render_bind_group_layout],
        push_constant_ranges: &[],
    });
    let fragment_state = FragmentState {
        module: &render_shader,
        entry_point: "fragment_main",
        targets: &[Some((*swapchain_format).into())],
    };
    let render_pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("Render Pipeline Descriptor"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    };
    let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);
    Composite {
        shader: render_shader,
        pipeline: render_pipeline,
    }
}

pub fn draw_composite(
    device: &Device,
    encoder: &mut CommandEncoder,
    context: &Context,
    framebuffer_view: &TextureView,
) {
    let sampler_description = SamplerDescriptor {
        ..Default::default()
    };
    let sampler = device.create_sampler(&sampler_description);
    let texture_bind_group_descriptor = BindGroupDescriptor {
        label: Some("Texture bind group"),
        layout: &context.composite.pipeline.get_bind_group_layout(0),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&context.textures.output_texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&sampler),
            },
        ],
    };

    let texture_bind_group = device.create_bind_group(&texture_bind_group_descriptor);
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &framebuffer_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(&context.composite.pipeline);
        rpass.set_bind_group(0, &texture_bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}
