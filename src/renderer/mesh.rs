use crate::renderer::vertex::Vertex;
use nalgebra::{Vector3, Vector2, Vector4};
use crate::renderer::RenderError;
use gltf::Image;
use std::path::Path;
use std::ffi::OsStr;

pub struct Primitive {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

pub struct Mesh {
    primitives: Vec<Primitive>
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new()
        }
    }

    fn import_single_from_gltf_file<T>(path: T) -> Result<Self, RenderError>
        where T: Into<String> {
        let mut result : Vec<Primitive> = Vec::new();
        let cloned_path = path.into().clone();

        let (glft, buffers, _) = gltf::import(cloned_path)?;

        let meshes = glft.meshes().collect::<Vec<gltf::Mesh<'_>>>();
        if meshes.len() != 1 {
            return Err(RenderError::Import("Only able to import one mesh per file".to_string()));
        }

        let mesh = meshes.first().unwrap();
        let images = glft.images().collect::<Vec<Image>>();

        for primitive in mesh.primitives() {
            result.push(Primitive::import_from_gltf_primitive(&primitive, &buffers, &images));
        }

        Ok(Self {
            primitives: result
        })
    }
}

impl Primitive {
    pub(self) fn import_from_gltf_primitive(primitive: &gltf::Primitive,
                                            buffer_data: &Vec<gltf::buffer::Data>,
                                            images: &Vec<Image>) -> Self {
        let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));
        let mut vertex : Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        if let Some(positions) = reader.read_positions() {
            for pos in positions.map(|pos| Vector3::from(pos)).into_iter() {
                let mut vert = Vertex::default();
                vert.set_position(pos);
                vertex.push(vert);
            }
        }

        if let Some(normals) = reader.read_normals() {
            for (i, norm) in normals.enumerate() {
                vertex[i].set_normal(Vector3::from(norm));
            }
        }

        if let Some(uvs) = reader.read_tex_coords(0) {
            for (i, uv) in uvs.into_f32().enumerate() {
                vertex[i].set_uv(Vector2::from(uv));
            }
        }

        if let Some(tangents) = reader.read_tangents() {
            for (i, tan) in tangents.enumerate() {
                vertex[i].set_tangent(Vector4::from(tan));
            }
        }

        if let Some(index_enum) = reader.read_indices() {
            for i in index_enum.into_u32() {
                indices.push(i);
            }
        }

        let gltf_material: gltf::Material<'_> = primitive.material();
        let pbr = gltf_material.pbr_metallic_roughness();

        let color = Vector4::from(pbr.base_color_factor());
        if let Some(color_texture) = pbr.base_color_texture() {
            let texture = color_texture.texture();

            if let Some(image) = images.get(texture.index()) {
                // match image.source() {
                //     gltf::image::Source::Uri { uri, .. } => {
                //         let texture_file_name = Path::new(&uri).file_name().and_then(OsStr::to_str).unwrap().to_string();
                //     }
                // }
            }
        }

        Self {
            vertices: vertex,
            indices: indices
        }
    }
}