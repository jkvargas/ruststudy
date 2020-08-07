#version 450

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec4 in_tangent;
layout(location = 3) in vec3 in_uv;
layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec3 v_position;

layout(set = 0, binding = 1) uniform View {
    mat4 view;
};

layout(set = 0, binding = 2) uniform Proj {
    mat4 proj;
};

void main() {
    vec4 normal = view * vec4(in_normal, 1.0);
    v_normal = normal.xyz;
    vec4 pos = proj * view * vec4(in_position, 1.0);
    v_position = pos.xyz;
    gl_Position = pos;
}