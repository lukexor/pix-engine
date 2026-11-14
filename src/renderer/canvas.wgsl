// Mesh shader for the canvas painter.
//
// Matches the vertex layout `epaint` tessellates into, so tessellated geometry reaches the GPU
// unchanged. One pipeline per blend mode shares this shader and differs only in blend state.

struct Uniforms {
    // Canvas size in pixels. Positions arrive in pixels and are mapped to clip space here.
    screen_size: vec2<f32>,
    // WebGL requires a uniform buffer of at least 16 bytes.
    _padding: vec2<u32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var t_texture: texture_2d<f32>;
@group(1) @binding(1) var s_texture: sampler;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

// `epaint` stores vertex colors as sRGB bytes, so they are converted to linear before blending.
fn linear_from_srgb(srgb: vec3<f32>) -> vec3<f32> {
    let cutoff = srgb < vec3<f32>(10.31475);
    let lower = srgb / vec3<f32>(3294.6);
    let higher = pow((srgb + vec3<f32>(14.025)) / vec3<f32>(269.025), vec3<f32>(2.4));
    return select(higher, lower, cutoff);
}

fn linear_from_srgba(srgba: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(linear_from_srgb(srgba.rgb * 255.0), srgba.a);
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: u32,
) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(
        2.0 * position.x / uniforms.screen_size.x - 1.0,
        1.0 - 2.0 * position.y / uniforms.screen_size.y,
        0.0,
        1.0,
    );
    out.uv = uv;
    let unpacked = unpack4x8unorm(color);
    out.color = linear_from_srgba(unpacked);
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sampled = textureSample(t_texture, s_texture, in.uv);
    return in.color * sampled;
}
