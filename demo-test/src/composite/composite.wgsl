@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32((i32(in_vertex_index) % 2) * 4 - 1);
    let y = f32((i32(in_vertex_index) / 2) * 4 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@group(0) @binding(0) var texture_in: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

@fragment
fn fragment_main(@builtin(position) position_in: vec4<f32>) -> @location(0) vec4<f32> {
    let coord = vec2<i32>(position_in.xy);
    let color: vec4<f32> = textureLoad(texture_in, coord, 0);
    return color;
}