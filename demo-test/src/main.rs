use std::borrow::Cow;

use compute::{dispatch_compute, init_compute};
use context::Context;
use textures::init_textures;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, FragmentState, RenderPipelineDescriptor, SamplerDescriptor,
    ShaderSource, ShaderStages, TextureViewDescriptor, TextureViewDimension, VertexState,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod compute;
mod context;
mod textures;

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

    let textures = init_textures(&device, size);
    let compute = init_compute(&device);
    let context = Context { textures, compute };

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
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
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

                dispatch_compute(&device, &mut encoder, &context);

                let sampler_description = SamplerDescriptor {
                    ..Default::default()
                };
                let sampler = device.create_sampler(&sampler_description);
                let texture_bind_group_descriptor = BindGroupDescriptor {
                    label: Some("Texture bind group"),
                    layout: &render_pipeline.get_bind_group_layout(0),
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(
                                &context.textures.output_texture_view,
                            ),
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
