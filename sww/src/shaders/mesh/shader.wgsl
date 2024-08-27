alias Padding = vec2f;
const PADDING: Padding = vec2f(0., 0.);

struct Rectangle {
    top_left: vec2f,
    size: vec2f,
}

fn vec_to_rect(v: vec4f) -> Rectangle {
    return Rectangle(v.xy, v.zw);
}

fn rectangle_then(a: Rectangle, b: Rectangle) -> Rectangle {
    let top_left = a.top_left + b.top_left * a.size;
    let size = a.size * b.size;
    return Rectangle(top_left, size);
}

fn transforms_then(a: Transform, b: Transform) -> Transform {
    return Transform(
        b.matrix * a.matrix,
        b.matrix * a.translation + b.translation,
        PADDING,
        a.color * b.color,
        rectangle_then(a.texture_rect, b.texture_rect),
    );
}

fn apply_transform_point(transform: Transform, point: vec2f) -> vec2f {
    return transform.matrix * point + transform.translation;
}

fn apply_transform_color(transform: Transform, color: vec4f) -> vec4f {
    return transform.color;
}

////////////////////////////////////////////////////////////

struct InVertex {
    @location(0) color: vec4f,
    @location(1) position: vec2f,
    @location(2) texture_coord: vec2f,
}

// should always be changed together
struct Transform {
    matrix: mat2x2f,
    translation: vec2f,
    _1: Padding,
    color: vec4f,
    texture_rect: Rectangle,
}
struct InTransform {
    @location(3) matrix: vec4f,
    @location(4) translation: vec2f,
    @location(5) _1: Padding,
    @location(6) color: vec4f,
    @location(7) texture_rect: vec4f,
}

fn vec_to_mat(v: vec4f) -> mat2x2f {
    return mat2x2f(v.xy, v.zw);
}

fn in_to_transform(transform: InTransform) -> Transform {
    return Transform(
        vec_to_mat(transform.matrix),
        transform.translation,
        PADDING,
        transform.color,
        vec_to_rect(transform.texture_rect),
    );
}

struct OutVertex {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
    @location(1) texture_coord: vec2f,
    @location(2) texture_rect_top_left: vec2f,
    @location(3) texture_rect_size: vec2f,
    @location(4) _1: Padding,
}

struct InFragment {
    @location(0) color: vec4f,
    @location(1) texture_coord: vec2f,
    @location(2) texture_rect_top_left: vec2f,
    @location(3) texture_rect_size: vec2f,
    @location(4) _1: Padding,
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

    return OutVertex(
        vec4f(position, 0., 1.),
        color,
        in_vertex.texture_coord,
        transform.texture_rect.top_left,
        transform.texture_rect.size,
        PADDING
    );
}

@group(1) @binding(0) var texture: texture_2d<f32>;

@fragment
fn fs_main(in: InFragment) -> @location(0) vec4f {
    var texture_coord_f = in.texture_coord;
    texture_coord_f.y = 1. - texture_coord_f.y;
    texture_coord_f = in.texture_rect_top_left + texture_coord_f * in.texture_rect_size;

    let size = textureDimensions(texture);
    var texture_coord = vec2u(texture_coord_f * vec2f(size));

    let texel_color = textureLoad(texture, texture_coord, 0);
    return in.color * texel_color;
}
