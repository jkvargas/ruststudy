use gltf::{Image, material::PbrMetallicRoughness};
use std::{path::Path, ffi::OsStr};
use crate::renderer::{material::Material, vertex::Vertex, Primitive, RenderError, Mesh, IntoWgpuEquivalent};
use nalgebra::{Vector4, Vector3, Vector2};
use gltf::material::NormalTexture;
use wgpu::{SamplerDescriptor, FilterMode, AddressMode};
use gltf::json::texture::{MagFilter, MinFilter};
use winit::event::VirtualKeyCode::Add;

static DEFAULT_MATERIAL: &str = "default.png";

pub struct GLTFImporter;

impl IntoWgpuEquivalent for MagFilter {
    type Output = wgpu::FilterMode;

    fn into_wgpu_equivalent(self) -> Self::Output {
        match self {
            MagFilter::Linear => FilterMode::Linear,
            MagFilter::Nearest => FilterMode::Nearest
        }
    }
}

impl IntoWgpuEquivalent for gltf::mesh::Mode {
    type Output = wgpu::PrimitiveTopology;

    fn into_wgpu_equivalent(self) -> Self::Output {
        match self {
            gltf::mesh::Mode::Points => wgpu::PrimitiveTopology::PointList,
            gltf::mesh::Mode::Lines => wgpu::PrimitiveTopology::LineList,
            gltf::mesh::Mode::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            gltf::mesh::Mode::Triangles => wgpu::PrimitiveTopology::TriangleList,
            gltf::mesh::Mode::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            gltf::mesh::Mode::LineLoop => wgpu::PrimitiveTopology::LineList,
            gltf::mesh::Mode::TriangleFan => wgpu::PrimitiveTopology::TriangleList
        }
    }
}

impl IntoWgpuEquivalent for gltf::texture::WrappingMode {
    type Output = wgpu::AddressMode;

    fn into_wgpu_equivalent(self) -> Self::Output {
        match self {
            gltf::texture::WrappingMode::ClampToEdge => AddressMode::ClampToEdge,
            gltf::texture::WrappingMode::MirroredRepeat => AddressMode::MirrorRepeat,
            gltf::texture::WrappingMode::Repeat => AddressMode::Repeat
        }
    }
}

impl IntoWgpuEquivalent for gltf::texture::Sampler {
    type Output = wgpu::SamplerDescriptor;

    fn into_wgpu_equivalent(self) -> Self::Output {
        SamplerDescriptor {
            min_filter: self.min_filter().unwrap_or(MinFilter::Linear).into_wgpu_equivalent(),
            mag_filter: self.mag_filter().unwrap_or(MagFilter::Linear).into_wgpu_equivalent(),
            address_mode_u: self.wrap_s().into_wgpu_equivalent(),
            address_mode_v: self.wrap_t().into_wgpu_equivalent(),
            // not quite sure about this one... couldn't find a equivalent in https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md
            address_mode_w: AddressMode::ClampToEdge,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
            mipmap_filter: wgpu::FilterMode::Nearest
        }
    }
}

impl IntoWgpuEquivalent for MinFilter {
    type Output = wgpu::FilterMode;

    fn into_wgpu_equivalent(self) -> Self::Output {
        match self {
            MinFilter::Linear => FilterMode::Linear,
            MinFilter::Nearest => FilterMode::Nearest,
            MinFilter::LinearMipmapLinear => FilterMode::Linear,
            MinFilter::LinearMipmapNearest => FilterMode::Linear,
            MinFilter::NearestMipmapLinear => FilterMode::Nearest,
            MinFilter::NearestMipmapNearest => FilterMode::Nearest
        }
    }
}

impl GLTFImporter {
    pub fn import_single_mesh<T>(path: T) -> Result<(Mesh, Vec<Material>, Vec<SamplerDescriptor>), RenderError>
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
            Self::fill_material_for_primitive(&images, &mut materials, &mut primitive, &gltf_primitive);
            primitive.mode = gltf_primitive.mode().into_wgpu_equivalent();
            primitives.push(primitive);
        }

        Ok((Mesh::new(primitives), materials, glft.samplers().map(|x| x.into_wgpu_equivalent()).collect()))
    }

    fn fill_material_for_primitive(images: &Vec<gltf::Image<'_>>, materials: &mut Vec<Material>, intprimitive: &mut Primitive, primitive: &gltf::Primitive) {
        let gltf_material: gltf::Material<'_> = primitive.material();
        let pbr = gltf_material.pbr_metallic_roughness();

        let base_color = pbr.base_color_factor();
        let color = Vector4::new(base_color[0], base_color[1], base_color[2], base_color[3]);
        let normal_texture = Self::get_normal_map(&gltf_material.normal_texture());
        let main_texture = Self::get_texture_url(&pbr.base_color_texture(), &images);
        let roughness_texture = Self::get_texture_url(&pbr.metallic_roughness_texture(), &images);

        let material = Material::new(main_texture.unwrap_or(DEFAULT_MATERIAL.to_string()),
                                     normal_texture.unwrap_or(DEFAULT_MATERIAL.to_string()),
                                     roughness_texture.unwrap_or(DEFAULT_MATERIAL.to_string()), color);

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

    fn get_texture_url(info: &Option<gltf::texture::Info<'_>>,
                       images: &Vec<gltf::Image<'_>>) -> Option<String> {
        if let Some(res) = info {
            let tex = res.texture();
            let image: Option<&gltf::Image<'_>> = images.get(tex.index());
            if let Some(img) = image {
                return match img.source() {
                    gltf::image::Source::Uri { uri, .. } => {
                        let texture_file_name = Path::new(&uri).file_name().and_then(OsStr::to_str).unwrap().to_string();

                        Some(texture_file_name)
                    }
                    _ => None
                };
            }
        }

        None
    }

    fn get_normal_map(normal_texture: &Option<NormalTexture>) -> Option<String> {
        if let Some(normal) = normal_texture {
            let texture = normal.texture().source().source();
            return match texture {
                gltf::image::Source::Uri { uri, .. } => {
                    let texture_file_name = Path::new(&uri).file_name().and_then(OsStr::to_str).unwrap().to_string();

                    Some(texture_file_name)
                }
                _ => None
            };
        }

        None
    }
}