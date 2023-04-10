@vertex
fn vertex_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    //let coord = vec2<i32>(position.xy);
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}