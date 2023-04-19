#version 450 core

void main() {
    int vertex_index = gl_VertexIndex;
    float x = float((vertex_index % 2) * 4 - 1);
    float y = float((vertex_index / 2) * 4 - 1);
    gl_Position = vec4(x, y, 0.0, 1.0);
}
