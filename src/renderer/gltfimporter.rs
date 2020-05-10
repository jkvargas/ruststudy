use gltf::Image;
use std::path::Path;
use std::ffi::OsStr;
use gltf::material::PbrMetallicRoughness;
use crate::renderer::material::Material;
use crate::renderer::vertex::Vertex;
use crate::renderer::{Primitive, RenderError};
use nalgebra::{Vector4, Vector3, Vector2};

static DEFAULT_MATERIAL: &str = "default.png";

pub struct GLTFImporter<'a> {
    images: Vec<gltf::Image<'a>>,
    materials: Vec<Material>,
    primitives: Vec<Primitive>,
    indices: Vec<u32>
}

impl<'a> GLTFImporter<'a> {
    pub fn import_single_mesh<T>(&mut self, path: T) -> Result<Self, RenderError>
        where T: Into<String> {
        let mut result: Vec<Primitive> = Vec::new();
        let cloned_path = path.into().clone();

        let (glft, buffers, _) = gltf::import(cloned_path)?;

        let meshes = glft.meshes().collect::<Vec<gltf::Mesh<'_>>>();
        if meshes.len() != 1 {
            return Err(RenderError::Import("Only able to import one mesh per file".to_string()));
        }

        let mesh = meshes.first().unwrap();
        let images = glft.images().collect::<Vec<Image>>();

        for gltf_primitive in mesh.primitives() {
            let primitive : Primitive = Default::default();
            self.fill_positions_for_primitive(&mut primitive, &gltf_primitive, &buffers);
            self.primitives.push(primitive);
        }

        Ok(Self {
            primitives: result
        })
    }



    fn get_material_from_primitive(&mut self, primitive: &gltf::Primitive) {
        let gltf_material: gltf::Material<'_> = primitive.material();
        let pbr = gltf_material.pbr_metallic_roughness();

        let color = Vector4::from(pbr.base_color_factor());
        let material_filename = self.get_material_filename(&pbr).unwrap_or(DEFAULT_MATERIAL.to_string());

        let material = Material::new(material_filename, color);

        self.materials.push(material);
    }

    fn fill_positions_for_primitive(&self, intprimitive: &mut Primitive, primitive: &gltf::Primitive, buffer_data: &Vec<gltf::buffer::Data>) {
        let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));
        let mut vecvert : Vec<Vertex> = Vec::new();

        if let Some(positions) = reader.read_positions() {
            for pos in positions.map(|pos| Vector3::new(pos[0], pos[1], pos[2])).into_iter() {
                let vert : Vertex = Default::default();
                vert.set_position(pos);
                vecvert.push(vert);
            }
        }

        if let Some(normals) = reader.read_normals() {
            for (i, norm) in normals.enumerate() {
                vecvert[i].set_normal(Vector3::from(norm));
            }
        }

        if let Some(uvs) = reader.read_tex_coords(0) {
            for (i, uv) in uvs.into_f32().enumerate() {
                vecvert[i].set_uv(Vector2::new(uv[0], uv[1]));
            }
        }

        if let Some(tangents) = reader.read_tangents() {
            for (i, tan) in tangents.enumerate() {
                vecvert[i].set_tangent(Vector4::from(tan));
            }
        }

        if let Some(index_enum) = reader.read_indices() {
            for i in index_enum.into_u32() {
                intprimitive.indices.push(i);
            }
        }

        intprimitive.vertex = vecvert;
    }

    fn get_material_filename(&self, pbr: &PbrMetallicRoughness) -> Option<String> {
        if let Some(color_texture) = pbr.base_color_texture() {
            let texture = color_texture.texture();

            if let Some(image) = self.images.get(texture.index()) {
                match image.source() {
                    gltf::image::Source::Uri { uri, .. } => {
                        let texture_file_name = Path::new(&uri).file_name().and_then(OsStr::to_str).unwrap().to_string();

                        return Some(texture_file_name);
                    }
                }
            }
        }

        None
    }
}