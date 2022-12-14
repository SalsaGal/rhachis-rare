struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn unshaded_vertex(in: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.pos = vec4<f32>(in.pos, 1.0);
    output.tex_coords = in.tex_coords;
    return output;
}

@group(0)@binding(0)
var color_texture: texture_2d<f32>;
@group(0)@binding(1)
var color_texture_sampler: sampler;

@fragment
fn unshaded_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_texture_sampler, in.tex_coords);
}
