use nalgebra::Vector4;

pub struct Material {
    texture: String,
    color: Vector4<f32>
}

impl Material {
    pub fn new(texture: String, color: Vector4<f32>) -> Self {
        Self {
            texture: texture,
            color: color
        }
    }
}