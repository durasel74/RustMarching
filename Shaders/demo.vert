#version 330 core
layout(location = 0) in vec3 screen_position;
void main() { gl_Position = vec4(screen_position, 1.0); }