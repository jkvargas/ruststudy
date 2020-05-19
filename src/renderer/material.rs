use nalgebra::Vector4;

pub struct Material {
    texture: String,
    normal: String,
    roughness: String,
    color: Vector4<f32>
}

impl Material {
    pub fn new(texture: String,
               normal: String,
               roughness: String,
               color: Vector4<f32>) -> Self {
        Self {
            texture,
            normal,
            roughness,
            color
        }
    }
}