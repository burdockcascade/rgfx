struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct DrawUniform {
    camera: mat4x4<f32>,
    transform: mat4x4<f32>,
    use_texture: u32,
};

@group(0) @binding(0) var<uniform> draw_uniform: DrawUniform;
@group(1) @binding(0) var my_texture: texture_2d<f32>;
@group(1) @binding(1) var my_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Apply the model matrix to the vertex position.
    let world_position = draw_uniform.transform * vec4<f32>(in.position, 1.0, 1.0);
    out.clip_position = draw_uniform.camera * world_position;

    // Pass the vertex color directly to the fragment shader.
    out.color = in.color;

    // Pass the UV coordinates directly to the fragment shader.
    out.uv = in.uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // The final color starts with the interpolated vertex color.
    var final_color = in.color;

    // Check the 'use_texture' uniform to determine if texturing should be applied.
    if (draw_uniform.use_texture == 1u) {
        // Sample the texture and multiply its color with the vertex color.
        final_color = in.color * textureSample(my_texture, my_sampler, in.uv);
    }

    return final_color;
}