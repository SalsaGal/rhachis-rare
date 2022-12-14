struct VertexInput {
    @location(0) pos: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn unshaded_vertex(in: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.pos = vec4<f32>(in.pos, 1.0);
    return output;
}

@fragment
fn unshaded_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.1, 0.3, 0.2, 1.0);
}
