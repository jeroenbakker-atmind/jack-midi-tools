use composite::{draw_composite, init_composite};
use compute::{dispatch_compute, init_compute};
use context::Context;
use textures::init_textures;
use wgpu::{
    DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode,
    RequestAdapterOptions, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod composite;
mod compute;
mod context;
mod textures;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();

    let instance = Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: Limits::default().using_resolution(adapter.limits()),
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
    let composite = init_composite(&device, &swapchain_format);
    let context = Context {
        textures,
        compute,
        composite,
    };

    let mut config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };
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
                draw_composite(&device, &mut encoder, &context, &view);

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
