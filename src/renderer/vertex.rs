use nalgebra::{Vector3, Vector4, Vector2};
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
use bytemuck::{Zeroable, Pod};

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
    tangent: Vector4<f32>,
    uv: Vector2<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            uv: Vector2::<f32>::zeros(),
            tangent: Vector4::<f32>::zeros(),
            normal: Vector3::<f32>::zeros(),
            position: Vector3::<f32>::zeros()
        }
    }
}

impl Vertex {
    pub fn new(position: Vector3<f32>,
               normal: Vector3<f32>,
               tangent: Vector4<f32>,
               uv: Vector2<f32>) -> Self {
        Self {
            position: position,
            normal: normal,
            tangent: tangent,
            uv: uv
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_normal(&mut self, normal: Vector3<f32>) {
        self.normal = normal;
    }

    pub fn set_uv(&mut self, uv: Vector2<f32>) {
        self.uv = uv;
    }

    pub fn set_tangent(&mut self, tangent: Vector4<f32>) {
        self.tangent = tangent;
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
