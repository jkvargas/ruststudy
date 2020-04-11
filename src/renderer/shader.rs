use shaderc::{Compiler, CompileOptions, ShaderKind};
use wgpu::{ShaderModule, Device};

pub struct Shader {
    source_code: String,
    entry_point: String,
    shader_file_name: String
}

impl Shader {
    fn build_shader(&self, shader_kind: ShaderKind) -> crate::Result<Vec<u8>> {
        let mut compiler = Compiler::new().unwrap();
        let options = CompileOptions::new().unwrap();

        let artifact = compiler.compile_into_spirv_assembly(&self.source_code, shader_kind, &self.shader_file_name, &self.entry_point, Some(&options))?;

        Ok(artifact.as_binary_u8().to_vec())
    }

    pub fn new(source_code: String, entry_point: String, shader_file_name: String) -> Self {
        Self {
            source_code,
            entry_point,
            shader_file_name
        }
    }

    pub fn build_module(&self, device: &Device, shader_kind: ShaderKind) -> crate::Result<ShaderModule> {
        let spirv_byte_array = self.build_shader(shader_kind)?;

        Ok(device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&spirv_byte_array[..]))?))
    }

    pub fn create_from_file(device: &Device, path: String, shader_kind: ShaderKind) -> ShaderModule {
        let frag_shader = std::fs::read_to_string(&path).unwrap();
        Self::new(frag_shader, "main".to_string(), path).build_module(&device, shader_kind).unwrap()
    }
}