#version 450 core

uniform layout(binding = 0, rgba8) writeonly image2D texture_output;

layout(local_size_x = 16, local_size_y = 16) in;

void main() {
    ivec2 coordinate = ivec2(gl_GlobalInvocationID.xy);
    ivec2 image_size = imageSize(texture_output);

    if (coordinate.x >= image_size.x || coordinate.y >= image_size.y) {
        return;
    }

    float c = float(coordinate.x) / float(image_size.x);

    vec4 color = vec4(1.0, 0.8, c, 1.0);
    imageStore(texture_output, coordinate, color);
}