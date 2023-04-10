
@group(0) @binding(0) var output_texture : texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn compute_main(@builtin(global_invocation_id) global_id : vec3<u32>,) {
    let dimensions = textureDimensions(output_texture);
    let coords = vec2<i32>(global_id.xy);

    if(coords.x >= dimensions.x || coords.y >= dimensions.y) {
        return;
    }

    let color = vec4(1.0, 0.0, 0.0, 1.0);
    textureStore(output_texture, coords.xy, color);
}