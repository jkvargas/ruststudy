use nalgebra::{Vector3, Point3, Matrix4, Isometry3, Perspective3, U3};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    pub fn new(eye: Point3<f32>, target: Point3<f32>, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Camera {
            eye,
            target,
            aspect,
            fovy,
            znear,
            zfar
        }
    }

    pub fn get_mv_matrix(&self) -> Matrix4<f32> {
        let mv = self.get_mv_isometry();
        mv.to_homogeneous()
    }

    fn get_mv_isometry(&self) -> Isometry3<f32> {
        let model = Isometry3::new(Vector3::x(), nalgebra::zero());
        let view = Isometry3::look_at_rh(&self.eye, &self.target, &Vector3::y());
        view * model
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let projection = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        projection.to_homogeneous()
    }

    pub fn build_projection_matrix(&self) -> Matrix4<f32> {
        let projection = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        let mat_model_view = self.get_mv_matrix();
        projection.as_matrix() * mat_model_view
    }
}