struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

struct FragmentInput {
    @location(0) color: vec4f,
}

struct TransformInput {
    @location(2) matrix2_1: vec2f,
    @location(3) matrix2_2: vec2f,
    @location(4) translation: vec2f,
    @location(5) color: vec4f,
}

struct Transform {
    matrix2: mat2x2f,
    translation: vec2f,
    color: vec4f,
}

fn parse_transform_input(transform: TransformInput) -> Transform {
    let matrix2 = mat2x2f(transform.matrix2_1, transform.matrix2_2);
    let translation = transform.translation;
    let color = transform.color;

    return Transform(matrix2, translation, color);
}

fn apply_transform(transform: Transform, point: vec2f) -> vec2f {
    return transform.matrix2 * point + transform.translation;
}

@vertex
fn vs_main(
    @location(0) in_position: vec2f,
    @location(1) in_color: vec4f,
    transform_input: TransformInput,
) -> VertexOutput {
    let transform = parse_transform_input(transform_input);
    let position = apply_transform(transform, in_position);
    let color = in_color * transform.color;

    return VertexOutput(vec4f(position, 0., 1.), color);
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    return in.color;
}
