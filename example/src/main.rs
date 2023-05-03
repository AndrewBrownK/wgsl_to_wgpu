use std::iter;

use crate::shader::{PreparedRenderPass, ENTRY_FS_MAIN};
use futures::executor::block_on;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// Include the bindings generated by build.rs.
mod shader;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    config: wgpu::SurfaceConfiguration,
    pipeline: shader::PipelineStage,
    bind_group0: shader::bind_groups::BindGroup0,
    bind_group1: shader::bind_groups::BindGroup1,
    vertex_buffer: wgpu::Buffer,
}

impl State {
    async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::TEXTURE_COMPRESSION_BC,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: Vec::new(),
        };
        surface.configure(&device, &config);

        // Use the generated bindings to create the pipeline.
        let shader = shader::create_shader_module(&device);
        let render_pipeline_layout = shader::create_pipeline_layout(&device);

        let pipeline = shader::PipelineStage::new(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: shader::vertex_state(
                &shader,
                &shader::vs_main_entry(wgpu::VertexStepMode::Vertex),
            ),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: ENTRY_FS_MAIN,
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        }));

        // Create a gradient texture.
        let texture = device.create_texture_with_data(
            &queue,
            &wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: 4,
                    height: 4,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::all(),
                view_formats: &[],
            },
            &vec![
                [0, 0, 255, 255],
                [64, 0, 255, 255],
                [128, 0, 255, 255],
                [255, 0, 255, 255],
                [0, 64, 255, 255],
                [64, 64, 255, 255],
                [128, 64, 255, 255],
                [255, 64, 255, 255],
                [0, 128, 255, 255],
                [64, 128, 255, 255],
                [128, 128, 255, 255],
                [255, 128, 255, 255],
                [0, 255, 255, 255],
                [64, 255, 255, 255],
                [128, 255, 255, 255],
                [255, 255, 255, 255],
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>(),
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Use the generated types to ensure the correct bind group is assigned to each slot.
        let bind_group0 = shader::bind_groups::BindGroup0::from_bindings(
            &device,
            shader::bind_groups::BindGroupLayout0 {
                color_texture: &view,
                color_sampler: &sampler,
            },
        );

        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::cast_slice(&[shader::Uniforms {
                color_rgb: [1.0, 1.0, 1.0, 1.0],
            }]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group1 = shader::bind_groups::BindGroup1::from_bindings(
            &device,
            shader::bind_groups::BindGroupLayout1 {
                uniforms: uniforms_buffer.as_entire_buffer_binding(),
            },
        );

        // Initialize the vertex buffer based on the expected input structs.
        // For storage buffer compatibility, consider using encase instead.
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&[
                shader::VertexInput {
                    position: [-1.0, -1.0, 0.0],
                },
                shader::VertexInput {
                    position: [3.0, -1.0, 0.0],
                },
                shader::VertexInput {
                    position: [-1.0, 3.0, 0.0],
                },
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            size,
            config,
            pipeline,
            bind_group0,
            bind_group1,
            vertex_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });


        let mut render_pass = self.pipeline.set(render_pass);

        // Before:
        // EnhancedRenderPass<NeedsVertexBuffer0, NeedsBindGroup0, NeedsBindGroup1>

        // You can chain initialization all at once
        let mut render_pass = render_pass
            .set_bind_group_0(&self.bind_group0)
            .set_bind_group_1(&self.bind_group1);

        // Or shadow variables to initialize a bit at a time
        let mut render_pass = render_pass
            .set_vertex_buffer_0(self.vertex_buffer.slice(..));

        // After:
        // EnhancedRenderPass<Ready, Ready, Ready> implements DerefMut<Target=wgpu::RenderPass>

        render_pass.draw(0..3, 0..1);

        drop(render_pass);
        self.queue.submit(iter::once(encoder.finish()));

        // Actually draw the frame.
        output.present();

        Ok(())
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgsl_to_wgpu example")
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(State::new(&window));
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            _ => {}
        },
        Event::RedrawRequested(_) => match state.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{e:?}"),
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
