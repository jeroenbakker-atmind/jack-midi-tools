#version 450 core

void main() {
    float x = float((int(gl_VertexIndex) % 2) * 4 - 1);
    float y = float((int(gl_VertexIndex) / 2) * 4 - 1);
    gl_Position = vec4(x, y, 0.0, 1.0);
}
