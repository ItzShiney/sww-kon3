struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

struct FragmentInput {
    @location(0) color: vec4f,
}

@vertex
fn vs_main(
    @location(0) position: vec2f,
    @location(1) color: vec4f,
) -> VertexOutput {
    return VertexOutput(vec4f(position, 0., 1.), color);
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    return in.color;
}
