use nalgebra::{Vector3, Vector4};
use wgpu::{BindGroupLayoutDescriptor,
           BindGroupLayoutEntry,
           BindingType,
           IndexFormat,
           ShaderStage,
           VertexAttributeDescriptor,
           VertexBufferDescriptor,
           VertexFormat,
           VertexStateDescriptor,
           InputStepMode};
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[repr(C)]
pub struct Vertex {
    position: Vector4<f32>,
    color: Vector3<f32>
}

impl Serialize for Vertex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Vertex", 2)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("color", &self.color)?;
        state.end()
    }
}

impl Vertex {
    pub fn new(position: Vector4<f32>, color: Vector3<f32>) -> Self {
        Self {
            position: position,
            color: color
        }
    }

    pub fn get_layout_descriptor<'a>() -> BindGroupLayoutDescriptor<'a> {
        BindGroupLayoutDescriptor {
            label: None,
            bindings: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::VERTEX,
                    ty: BindingType::UniformBuffer { dynamic: false },
                }
            ],
        }
    }

    pub fn get_state_descriptor<'a>() -> VertexStateDescriptor<'a> {
        VertexStateDescriptor {
            index_format: IndexFormat::Uint16,
            vertex_buffers: &[VertexBufferDescriptor {
                stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: InputStepMode::Vertex,
                attributes: &[
                    VertexAttributeDescriptor {
                        format: VertexFormat::Float4,
                        shader_location: 0,
                        offset: 0,
                    },
                    VertexAttributeDescriptor {
                        format: VertexFormat::Float3,
                        shader_location: 1,
                        offset: 4 * 4,
                    },
                ],
            }],
        }
    }
}
