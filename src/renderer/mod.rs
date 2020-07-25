use std::fmt::Debug;
use derive_more::Display;
use crate::renderer::vertex::Vertex;
use wgpu::{Buffer, Device, BufferUsage, ShaderStage, BindGroupLayout, BindGroup};
use bytemuck::Pod;

pub mod vertex;
pub mod camera;
pub mod gltfimporter;
pub mod material;
pub mod light;

#[derive(Debug)]
pub struct Primitive {
    pub vertex: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material_index: usize,
    pub mode: wgpu::PrimitiveTopology,
}

pub trait IntoWgpuEquivalent {
    type Output;

    fn into_wgpu_equivalent(self) -> Self::Output;
}

pub fn create_buffer_and_layout<TPod: Pod>(device: &Device, visibility: ShaderStage, object: &[TPod]) -> (Buffer, BindGroupLayout, BindGroup) {
    let buffer = device.create_buffer_with_data(bytemuck::cast_slice(object), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: visibility,
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
        }],
        label: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &buffer,
                range: 0..std::mem::size_of_val(&buffer) as wgpu::BufferAddress,
            },
        }],
        label: None,
    });

    (buffer, bind_group_layout, bind_group)
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<Primitive>
}

impl Primitive {
    pub fn get_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_with_data(bytemuck::cast_slice(&self.vertex), BufferUsage::VERTEX)
    }

    pub fn get_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_with_data(bytemuck::cast_slice(&self.indices), BufferUsage::INDEX)
    }
}

impl Default for Primitive {
    fn default() -> Self {
        Self {
            vertex: Vec::new(),
            indices: Vec::new(),
            material_index: 0,
            mode: wgpu::PrimitiveTopology::TriangleList,
        }
    }
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self {
            primitives
        }
    }
}

#[derive(Debug, Display)]
pub enum RenderError {
    #[display(fmt = "Import problem: {}", _0)]
    Import(String)
}

impl From<gltf::Error> for RenderError {
    fn from(err: gltf::Error) -> Self {
        RenderError::Import(err.to_string())
    }
}