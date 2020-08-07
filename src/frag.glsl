#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec3 v_position;
layout(location = 0) out vec4 outColor;

layout(set=0, binding=2)
uniform Light {
    vec3 u_position;
    vec3 u_color;
};

void main() {
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(u_position - v_position);
    float diffuse_str = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = u_color * diffuse_str;

    outColor = vec4(light_dir[0], light_dir[1], light_dir[2], 1.0); //vec4(diffuse_color, 1.0);
}