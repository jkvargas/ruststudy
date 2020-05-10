use gltf::Image;
use std::path::Path;
use std::ffi::OsStr;
use gltf::material::PbrMetallicRoughness;
use crate::renderer::material::Material;
use gltf::json::mesh::Mode;
use crate::renderer::vertex::Vertex;
use crate::renderer::Primitive;

static DEFAULT_MATERIAL: &str = "default.png";

pub struct glTFImporter {
    images: Vec<gltf::Image>,
    materials: Vec<Material>,
    primitives: Vec<Primitive>
    indices: Vec<u32>
}

impl From<gltf::mesh::Mode> for wgpu::PrimitiveTopology {
    fn from(mode: Mode) -> Self {
        match mode {

        }
    }
}

impl glTFImporter {
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
            self.

            result.push(Primitive::import_from_gltf_primitive(&primitive, &buffers, &images));
        }

        Ok(Self {
            primitives: result
        })
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

    fn get_material_from_primitive(&mut self, primitive: &gltf::Primitive) {
        let gltf_material: gltf::Material<'_> = primitive.material();
        let pbr = gltf_material.pbr_metallic_roughness();

        let color = Vector4::from(pbr.base_color_factor());
        let material_filename = self.get_material_filename(&pbr).unwrap_or(DEFAULT_MATERIAL.to_string());

        let material = Material::new(material_filename, color);

        self.materials.push(material);
    }

    fn fill_positions_for_primitive(intprimitive: &mut Primitive, primitive: &gltf::Primitive, buffer_data: &Vec<gltf::buffer::Data>) {
        let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

        let vert : Vertex = Default::default();

        if let Some(positions) = reader.read_positions() {
            for pos in positions.map(|pos| Vector3::from(pos)).into_iter() {
                vert.set_position(pos);
            }
        }

        if let Some(normals) = reader.read_normals() {
            for (i, norm) in normals.enumerate() {
                self.vertex[i].set_normal(Vector3::from(norm));
            }
        }

        if let Some(uvs) = reader.read_tex_coords(0) {
            for (i, uv) in uvs.into_f32().enumerate() {
                self.vertex[i].set_uv(Vector2::from(uv));
            }
        }

        if let Some(tangents) = reader.read_tangents() {
            for (i, tan) in tangents.enumerate() {
                self.vertex[i].set_tangent(Vector4::from(tan));
            }
        }

        if let Some(index_enum) = reader.read_indices() {
            for i in index_enum.into_u32() {
                self.indices.push(i);
            }
        }

        intprimitive.add_vertex(vert);
    }
}