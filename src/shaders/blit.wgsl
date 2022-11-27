@vertex
struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var result: VertexOutput;
    let x = i32(index) / 2;
    let y = i32(index) & 1;
    let uv = vec2<f32>(
        f32(x) * 2.0,
        f32(y) * 2.0
    );
    result.pos = vec4<f32>(
        uv.x * 2.0 - 1.0,
        1.0 - uv.y * 2.0,
        0.0,
        1.0
    );
    result.uv = uv;
    return result;
}

@group(0)
@binding(0)
var tex: texture_2d<f32>;

@group(0)
@binding(1)
var r_sampler: sampler;

@fragment 
fn fs_main(vert: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(tex, r_sampler, vert.uv);

    return vec4<f32>(color.xyz, 1.0);
}
