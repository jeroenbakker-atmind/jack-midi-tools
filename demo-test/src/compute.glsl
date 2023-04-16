#version 450 core

uniform layout(binding = 0, rgba8) writeonly image2D texture_output;

layout(local_size_x = 16, local_size_y = 16) in;

void main() {
    uvec2 coordinate = uvec2(gl_GlobalInvocationID.xy);
    uvec2 image_size = imageSize(texture_output);

    if (coordinate.x >= image_size.x || coordinate.y >= image_size.y) {
        return;
    }
}