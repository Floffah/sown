@group(0) @binding(0) var<uniform> camera: vec4<f32>;

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
    let eye = vec3(3.0, 3.0, 5.0);
    let forward = normalize(-eye);
    let right = normalize(cross(forward, vec3(0.0, 1.0, 0.0)));
    let up = cross(right, forward);
    let scaled = vertex * scale;
    let rotated = scaled + 2.0 * cross(rotation.xyz, cross(rotation.xyz, scaled) + rotation.w * scaled);
    let offset = rotated + position - eye;
    let depth = dot(offset, forward);
    // Perspective with a 60-degree vertical field of view and WebGPU's 0..1 depth range.
    let focal_length = 1.7320508;
    let near = 0.1;
    let far = 100.0;

    var output: VertexOutput;
    output.position = vec4(
        dot(offset, right) * focal_length / camera.x,
        dot(offset, up) * focal_length,
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
