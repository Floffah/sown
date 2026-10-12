@group(0) @binding(0) var<uniform> camera: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
};

// shortcut: geometry is fixed in the shader, move it to mesh buffers when drawing world objects.
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let corners = array<vec3<f32>, 8>(
        vec3(-1.0, -1.0, -1.0), vec3(1.0, -1.0, -1.0),
        vec3(1.0, 1.0, -1.0), vec3(-1.0, 1.0, -1.0),
        vec3(-1.0, -1.0, 1.0), vec3(1.0, -1.0, 1.0),
        vec3(1.0, 1.0, 1.0), vec3(-1.0, 1.0, 1.0),
    );
    let indices = array<u32, 36>(
        4, 5, 6, 4, 6, 7,
        1, 0, 3, 1, 3, 2,
        5, 1, 2, 5, 2, 6,
        0, 4, 7, 0, 7, 3,
        7, 6, 2, 7, 2, 3,
        0, 1, 5, 0, 5, 4,
    );
    let colors = array<vec3<f32>, 6>(
        vec3(0.9, 0.3, 0.15), vec3(0.5, 0.15, 0.1),
        vec3(0.2, 0.6, 0.9), vec3(0.1, 0.3, 0.5),
        vec3(0.9, 0.7, 0.2), vec3(0.4, 0.3, 0.1),
    );

    let eye = vec3(3.0, 2.0, 4.0);
    let forward = normalize(-eye);
    let right = normalize(cross(forward, vec3(0.0, 1.0, 0.0)));
    let up = cross(right, forward);
    let offset = corners[indices[vertex_index]] - eye;
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
    output.color = colors[vertex_index / 6u];
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(input.color, 1.0);
}
