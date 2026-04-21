@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;

// ==========================
// Vertex input (from CPU)
// ==========================
struct VertexInput {
@location(0) position: vec2<f32>,
@location(1) uv: vec2<f32>,
};

// ==========================
// Vertex output → Fragment
// ==========================
struct VertexOutput {
@builtin(position) position: vec4<f32>,
@location(0) uv: vec2<f32>,
};

// ==========================
// Vertex shader
// ==========================
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
var out: VertexOutput;

// position MUST already be in NDC (-1 → 1)
out.position = vec4<f32>(in.position, 0.0, 1.0);

// pass UV to fragment shader
out.uv = in.uv;

return out;

}

// ==========================
// Fragment shader
// ==========================
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
return textureSample(my_texture, my_sampler, in.uv);
}