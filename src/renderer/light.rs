use nalgebra::Vector3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Light {
    position: Vector3<f32>,
    color: Vector3<f32>,
}

unsafe impl bytemuck::Zeroable for Light {}
unsafe impl bytemuck::Pod for Light {}

impl Light {
    pub fn new(position: Vector3<f32>,
               color: Vector3<f32>) -> Self {
        Light {
            position,
            color,
        }
    }
}