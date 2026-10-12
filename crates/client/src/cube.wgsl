struct Camera {
    position: vec4<f32>,
    rotation: vec4<f32>,
    // Focal length, aspect ratio, near, far.
    projection: vec4<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;

fn rotate_vector(vector: vec3<f32>, rotation: vec4<f32>) -> vec3<f32> {
    return vector + 2.0 * cross(rotation.xyz, cross(rotation.xyz, vector) + rotation.w * vector);
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) rotation: vec4<f32>,
    @location(3) scale: vec3<f32>,
    @location(4) color: vec3<f32>,
) -> VertexOutput {
    let face_shades = array<f32, 6>(1.0, 0.6, 0.8, 0.5, 0.9, 0.4);
    let world_position = rotate_vector(vertex * scale, rotation) + position;
    let inverse_rotation = vec4(-camera.rotation.xyz, camera.rotation.w);
    let view_position = rotate_vector(world_position - camera.position.xyz, inverse_rotation);
    let depth = -view_position.z;
    let focal_length = camera.projection.x;
    let aspect = camera.projection.y;
    let near = camera.projection.z;
    let far = camera.projection.w;

    var output: VertexOutput;
    output.position = vec4(
        view_position.x * focal_length / aspect,
        view_position.y * focal_length,
        far / (far - near) * depth - far * near / (far - near),
        depth,
    );
    output.color = color * face_shades[vertex_index / 6u];
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(input.color, 1.0);
}
