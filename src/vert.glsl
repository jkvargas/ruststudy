#version 450

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec4 in_tangent;
layout(location = 3) in vec3 in_uv;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out vec3 v_position;

layout(set = 0, binding = 0)
uniform ModelView {
    mat4 model_view;
};

layout(set = 1, binding = 0)
uniform Projection {
    mat4 proj;
};

void main() {
    vec4 normal = model_view * vec4(in_normal, 1.0);
    v_normal = normal.xyz;
    vec4 pos = model_view * proj * vec4(in_position, 1.0);
    v_position = pos.xyz;
    gl_Position = pos;
}