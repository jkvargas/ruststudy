use std::fmt::Debug;
use derive_more::Display;

pub mod vertex;
pub mod camera;
pub mod mesh;
pub mod gltfimporter;
pub mod material;

pub struct Primitive {
    vertex: Vec<Vertex>,
    indices: Vec<u32>,
    material_index: usize,
    mode: wgpu::PrimitiveTopology
}

pub struct Mesh {
    primitives: Vec<Primitive>
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

impl Primitive {
    // we don't want to expose the full vector. So lets just add some simple methods to interact with internals
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertex.push(vertex);
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new()
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