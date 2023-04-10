use std::borrow::Cow;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, ComputePassDescriptor, Extent3d, FragmentState,
    PipelineLayoutDescriptor, RenderPipelineDescriptor, SamplerDescriptor, ShaderSource,
    ShaderStages, StorageTextureAccess, TextureDescriptor, TextureFormat, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::default().using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    // Load the shaders from disk

    let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("compute.wgsl"))),
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
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[TextureFormat::Rgba8Unorm],
    });

    let output_texture_view = output_texture.create_view(&TextureViewDescriptor::default());
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    /* Render pipeline (result to screen) */
    let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
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
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
            count: None,
        }],
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
        targets: &[Some(swapchain_format.into())],
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

    surface.configure(&device, &config);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

                let texture_bind_group_descriptor = BindGroupDescriptor {
                    label: Some("Texture bind group"),
                    layout: &compute_pipeline.get_bind_group_layout(0),
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&output_texture_view),
                    }],
                };

                let texture_bind_group = device.create_bind_group(&texture_bind_group_descriptor);
                {
                    let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                        label: Some("Compute pass"),
                    });
                    compute_pass.set_pipeline(&compute_pipeline);
                    compute_pass.set_bind_group(0, &texture_bind_group, &[]);
                    compute_pass.dispatch_workgroups(size.width / 16, size.height / 16, 0);
                }
                let sampler_description = SamplerDescriptor {
                    ..Default::default()
                };
                let sampler = device.create_sampler(&sampler_description);
                let texture_bind_group_descriptor = BindGroupDescriptor {
                    label: Some("Texture bind group"),
                    layout: &render_pipeline.get_bind_group_layout(0),
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Sampler(&sampler),
                    }],
                };

                let texture_bind_group = device.create_bind_group(&texture_bind_group_descriptor);
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &texture_bind_group, &[]);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    //window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    pollster::block_on(run(event_loop, window));
}
