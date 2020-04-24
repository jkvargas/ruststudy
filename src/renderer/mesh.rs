use crate::renderer::vertex::Vertex;
use nalgebra::{Vector4, Vector3};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

impl Mesh {
    pub fn new() -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let (glft, buffers, _) = gltf::import("monkey.glb").expect("problems");

        for mesh in glft.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        let vertex = Vertex::new(
                            Vector4::new(vertex_position[0], vertex_position[1], vertex_position[2], 0.0),
                            Vector3::new(1.0, 0.0, 0.0),
                        );

                        vertices.push(vertex);
                    }
                }

                indices = if let Some(index_enum) = reader.read_indices() {
                    index_enum.into_u32().collect()
                } else {
                    panic!("model doesn't have indices");
                };
            }
        }

        Self {
            vertices: vertices,
            indices: indices
        }
    }
}