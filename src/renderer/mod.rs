use std::fmt::Debug;
use derive_more::Display;
use crate::renderer::vertex::Vertex;
use wgpu::{Buffer, Device, BufferUsage};

pub mod vertex;
pub mod camera;
pub mod gltfimporter;
pub mod material;

pub struct Primitive {
    vertex: Vec<Vertex>,
    indices: Vec<u32>,
    material_index: usize,
    mode: wgpu::PrimitiveTopology
}

pub trait IntoWgpuEquivalent {
    type Output;

    fn into_wgpu_equivalent(self) -> Self::Output;
}

pub struct Mesh {
    primitives: Vec<Primitive>
}

impl Mesh {
    pub fn get_vertex_buffer_per_primitive(&self, device: &Device) -> Vec<Buffer> {
        self.primitives.iter().map(|x| x.get_vertex_buffer(device)).collect()
    }

    pub fn get_buffer_index(&self, device: &Device) -> Buffer {
    }

    pub fn get_buffer_index_per_primitive(&self, device: &Device) -> Vec<Buffer> {
        self.primitives.iter().map(|x| x.get_index_buffer(device)).collect()
    }
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
            mode: wgpu::PrimitiveTopology::TriangleList
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