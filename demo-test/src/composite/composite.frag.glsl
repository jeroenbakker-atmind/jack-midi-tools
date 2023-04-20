#version 450 core


uniform layout(binding = 0, rgba8) readonly image2D texture_in;
out vec4 frag_color;


void main() {
    ivec2 coord = ivec2(gl_FragCoord.xy);
    vec4 color = imageLoad(texture_in, coord);
    frag_color = color;
}