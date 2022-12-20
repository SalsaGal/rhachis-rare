struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct Transform {
    @location(2) data0: vec4<f32>,
    @location(3) data1: vec4<f32>,
    @location(4) data2: vec4<f32>,
    @location(5) data3: vec4<f32>,
};

@group(1)@binding(0)
var<uniform> camera: Transform;

@vertex
fn vertex_main(in: VertexInput, transform: Transform) -> VertexOutput {
    let transform_matrix = mat4x4<f32>(
        transform.data0,
        transform.data1,
        transform.data2,
        transform.data3,
    );
    let camera_matrix = mat4x4<f32>(
        camera.data0,
        camera.data1,
        camera.data2,
        camera.data3,
    );

    var output: VertexOutput;
    output.pos = camera_matrix * transform_matrix * vec4<f32>(in.pos, 1.0);
    output.tex_coords = in.tex_coords;
    return output;
}

@group(0)@binding(0)
var color_texture: texture_2d<f32>;
@group(0)@binding(1)
var color_texture_sampler: sampler;

struct Light {
    color: vec3<f32>
}

struct LightArray {
    lights: array<Light>
}

@group(2)@binding(0)
var<storage> light: LightArray;

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_texture_sampler, in.tex_coords);
}
