use std::fmt::Debug;
use derive_more::Display;

pub mod vertex;
pub mod camera;
pub mod mesh;

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