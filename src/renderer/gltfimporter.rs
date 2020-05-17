use gltf::{ Image, material::PbrMetallicRoughness};
use std::{ path::Path, ffi::OsStr };
use crate::renderer::{
    material::Material,
    vertex::Vertex,
    Primitive,
    RenderError,
    Mesh
};
use nalgebra::{Vector4, Vector3, Vector2};

static DEFAULT_MATERIAL: &str = "default.png";

pub struct GLTFImporter;

impl GLTFImporter {
    pub fn import_single_mesh<T>(&mut self, path: T) -> Result<(Mesh, Vec<Material>), RenderError>
        where T: Into<String> {
        let cloned_path = path.into().clone();

        let (glft, buffers, _) = gltf::import(cloned_path)?;

        let meshes = glft.meshes().collect::<Vec<gltf::Mesh<'_>>>();
        if meshes.len() != 1 {
            return Err(RenderError::Import("Only able to import one mesh per file".to_string()));
        }

        let mut materials: Vec<Material> = Vec::new();
        let mut primitives: Vec<Primitive> = Vec::new();
        let images = glft.images().collect::<Vec<Image>>();

        let mesh = meshes.first().unwrap();
        for gltf_primitive in mesh.primitives() {
            let mut primitive: Primitive = Default::default();
            Self::fill_positions_for_primitive(&mut primitive, &gltf_primitive, &buffers);
            Self::fill_material_for_primitive(&images, &mut materials,&mut primitive, &gltf_primitive);
            Self::fill_mode_for_primitive(&mut primitive, &gltf_primitive)?;
            primitives.push(primitive);
        }

        Ok((Mesh::new(primitives), materials))
    }

    fn fill_mode_for_primitive(intprimitive: &mut Primitive, primitive: &gltf::Primitive) -> Result<(), RenderError> {
        intprimitive.mode = match primitive.mode() {
            gltf::mesh::Mode::Points => wgpu::PrimitiveTopology::PointList,
            gltf::mesh::Mode::Lines => wgpu::PrimitiveTopology::LineList,
            gltf::mesh::Mode::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            gltf::mesh::Mode::Triangles => wgpu::PrimitiveTopology::TriangleList,
            gltf::mesh::Mode::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            _ => return Err(RenderError::Import("Mode is not available".to_string()))
        };

        Ok(())
    }

    fn fill_material_for_primitive(images: &Vec<gltf::Image<'_>>, materials: &mut Vec<Material>, intprimitive: &mut Primitive, primitive: &gltf::Primitive) {
        let gltf_material: gltf::Material<'_> = primitive.material();
        let pbr = gltf_material.pbr_metallic_roughness();

        let base_color = pbr.base_color_factor();
        let color = Vector4::new(base_color[0], base_color[1], base_color[2], base_color[3]);
        let material_filename = Self::get_material_filename(images, &pbr).unwrap_or(DEFAULT_MATERIAL.to_string());

        let material = Material::new(material_filename, color);

        materials.push(material);

        intprimitive.material_index = materials.len() - 1;
    }

    fn fill_positions_for_primitive(intprimitive: &mut Primitive, primitive: &gltf::Primitive, buffer_data: &Vec<gltf::buffer::Data>) {
        let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

        if let Some(positions) = reader.read_positions() {
            for pos in positions.map(|pos| Vector3::new(pos[0], pos[1], pos[2])).into_iter() {
                let mut vert: Vertex = Default::default();
                vert.set_position(pos);
                intprimitive.vertex.push(vert);
            }
        }

        if let Some(normals) = reader.read_normals() {
            for (i, norm) in normals.enumerate() {
                intprimitive.vertex[i].set_normal(Vector3::new(norm[0], norm[1], norm[2]));
            }
        }

        if let Some(uvs) = reader.read_tex_coords(0) {
            for (i, uv) in uvs.into_f32().enumerate() {
                intprimitive.vertex[i].set_uv(Vector2::new(uv[0], uv[1]));
            }
        }

        if let Some(tangents) = reader.read_tangents() {
            for (i, tan) in tangents.enumerate() {
                intprimitive.vertex[i].set_tangent(Vector4::new(tan[0], tan[1], tan[2], tan[3]));
            }
        }

        if let Some(index_enum) = reader.read_indices() {
            for i in index_enum.into_u32() {
                intprimitive.indices.push(i);
            }
        }
    }

    fn get_material_filename(images: &Vec<gltf::Image<'_>>, pbr: &PbrMetallicRoughness) -> Option<String> {
        if let Some(color_texture) = pbr.base_color_texture() {
            let texture = color_texture.texture();

            if let Some(image) = images.get(texture.index()) {
                return match image.source() {
                    gltf::image::Source::Uri { uri, .. } => {
                        let texture_file_name = Path::new(&uri).file_name().and_then(OsStr::to_str).unwrap().to_string();

                        Some(texture_file_name)
                    }
                    _ => None
                }
            }
        }

        None
    }
}