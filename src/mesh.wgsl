struct Transform {
    matrix: mat2x2f,
    translation: vec2f,
    color: vec4f,
}

fn transforms_then(a: Transform, b: Transform) -> Transform {
    return Transform(
        b.matrix * a.matrix,
        b.matrix * a.translation + b.translation,
        a.color * b.color,
    );
}

fn apply_transform_point(transform: Transform, point: vec2f) -> vec2f {
    return transform.matrix * point + transform.translation;
}

fn apply_transform_color(transform: Transform, color: vec4f) -> vec4f {
    return transform.color;
}

////////////////////////////////////////////////////////////

struct Rectangle {
    start: vec2f,
    end: vec2f,
}

////////////////////////////////////////////////////////////

struct InVertex {
    @location(0) position: vec2f,
    @location(1) color: vec4f,
    @location(2) texture_coord: vec2f,
}

struct InTransform {
    @location(3) matrix: vec4f,
    @location(4) translation: vec2f,
    @location(5) color: vec4f,
}

fn vec_to_mat(v: vec4f) -> mat2x2f {
    return mat2x2f(v.xy, v.zw);
}

fn in_to_transform(transform: InTransform) -> Transform {
    return Transform(vec_to_mat(transform.matrix), transform.translation, transform.color);
}

struct OutVertex {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
    @location(1) texture_coord: vec2f,
}

struct InFragment {
    @location(0) color: vec4f,
    @location(1) texture_coord: vec2f,
}

////////////////////////////////////////////////////////////

@group(0) @binding(0) var<uniform> global_transform: Transform;

@vertex
fn vs_main(
    in_vertex: InVertex,
    in_transform: InTransform,
) -> OutVertex {
    let transform = transforms_then(in_to_transform(in_transform), global_transform);
    let position = apply_transform_point(transform, in_vertex.position);
    let color = apply_transform_color(transform, in_vertex.color);

    return OutVertex(vec4f(position, 0., 1.), color, in_vertex.texture_coord);
}

@group(1) @binding(0) var<uniform> texture_rect: Rectangle;
@group(1) @binding(1) var texture: texture_2d<f32>;

@fragment
fn fs_main(in: InFragment) -> @location(0) vec4f {
    var texture_coord_f = in.texture_coord;
    texture_coord_f = texture_coord_f * (texture_rect.end - texture_rect.start) + texture_rect.start;
    texture_coord_f.y = 1. - texture_coord_f.y;

    let size = textureDimensions(texture);
    var texture_coord = vec2u(texture_coord_f * vec2f(size));

    let texel_color = textureLoad(texture, texture_coord, 0);
    return in.color * texel_color;
}
