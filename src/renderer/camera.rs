use nalgebra::{Vector3, Point3, Matrix4, Isometry3, Perspective3};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    pub fn build_projection_matrix(&self) -> Matrix4<f32> {
        // Our object is translated along the x axis.
        let model = Isometry3::new(Vector3::x(), nalgebra::zero());
        let view   = Isometry3::look_at_rh(&self.eye, &self.target, &Vector3::y());

        // A perspective projection.
        let projection = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);

        // The combination of the model with the view is still an isometry.
        let model_view = view * model;

        // Convert everything to a `Matrix4` so that they can be combined.
        let mat_model_view = model_view.to_homogeneous();

        // Combine everything.
        projection.as_matrix() * mat_model_view
    }
}