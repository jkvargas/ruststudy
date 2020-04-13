#version 450

layout(location = 0) out vec3 fragColor;
layout(location = 0) in vec4 in_position;
layout(location = 1) in vec3 in_color;

void main() {
    gl_Position = in_position;
    fragColor = in_color;
}