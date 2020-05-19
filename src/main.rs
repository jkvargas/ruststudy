use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use futures::executor::block_on;
use nalgebra::{Vector3, Vector4, Vector, Point3};
use wgpu::{read_spirv, PipelineLayout, PowerPreference, PresentMode, PrimitiveTopology, ProgrammableStageDescriptor, RasterizationStateDescriptor, RenderPipelineDescriptor, RequestAdapterOptions, Surface, SwapChainDescriptor, VertexStateDescriptor, VertexBufferDescriptor, BindGroupDescriptor, BufferUsage};
use rustgraphics::renderer::vertex::Vertex;
use rustgraphics::renderer::camera::Camera;
use rustgraphics::renderer::gltfimporter::GLTFImporter;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    let surface = wgpu::Surface::create(&window);

    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::PRIMARY,
    )
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await;

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let vs = include_bytes!("vert.glsl.spv");
    let vs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());

    let fs = include_bytes!("frag.glsl.spv");
    let fs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

    let (mesh, materials, samplers) = GLTFImporter::import_single_mesh("BoxVertexColors".to_string())?;

    dbg!(&index_data);
    dbg!(&vertex_data);

    let camera = Camera::new(Point3::new(10.0, 0.0, 30.0), Point3::new(0.0, 0.0, 0.0), sc_desc.width as f32 / sc_desc.height as f32, 45f32, 1.0, 100.0);
    let view = camera.build_projection_matrix();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    component_type: wgpu::TextureComponentType::Float,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buf.slice(..)),
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: None,
    });

    let vertex_buf =
        device.create_buffer_with_data(bytemuck::cast_slice(&vertex_data), wgpu::BufferUsage::VERTEX);

    let index_buf =
        device.create_buffer_with_data(bytemuck::cast_slice(&index_data), wgpu::BufferUsage::INDEX);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let uniform_buf = device.create_buffer_with_data(
        bytemuck::cast_slice((&view).as_ref()),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &uniform_buf,
                range: 0..64,
            },
        }],
        label: None,
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: Vertex::get_state_descriptor(),
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });


    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::GREEN,
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_index_buffer(&index_buf, 0, 0);
                    rpass.set_vertex_buffer(0, &vertex_buf, 0, 0);
                    rpass.draw_indexed(0..index_data.len() as u32, 0, 0..1);
                }

                queue.submit(&[encoder.finish()]);
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
    env_logger::init();
    block_on(run(event_loop, window));
}
