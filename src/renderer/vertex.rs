use nalgebra::{Vector3, Vector4};
use zerocopy::{FromBytes, AsBytes};
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

#[derive(Clone, Copy, FromBytes, AsBytes)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 3]
}

impl Vertex {
    pub fn new(pos: [f32; 4], col: [f32; 3]) -> Self {
        Self {
            position: pos,
            color: col,
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
