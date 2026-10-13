use wgpu::util::DeviceExt;

pub(super) fn create_cube_mesh(device: &wgpu::Device) -> wgpu::Buffer {
    let corners: [[f32; 3]; 8] = [
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ];
    let indices = [
        4, 5, 6, 4, 6, 7, 1, 0, 3, 1, 3, 2, 5, 1, 2, 5, 2, 6, 0, 4, 7, 0, 7, 3, 7, 6, 2, 7, 2, 3,
        0, 1, 5, 0, 5, 4,
    ];
    let bytes: Vec<u8> = indices
        .into_iter()
        .flat_map(|index| corners[index])
        .flat_map(f32::to_le_bytes)
        .collect();
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Shared cube mesh"),
        contents: &bytes,
        usage: wgpu::BufferUsages::VERTEX,
    })
}
