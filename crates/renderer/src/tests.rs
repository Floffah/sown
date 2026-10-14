use super::*;
use engine::ecs::scene::{Material, Transform};

fn read_pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    mut encoder: wgpu::CommandEncoder,
    target: &wgpu::Texture,
) -> Vec<u8> {
    let bytes_per_row = target.width() * 4;
    assert_eq!(bytes_per_row % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT, 0);
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u64::from(bytes_per_row) * u64::from(target.height()),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: None,
            },
        },
        target.size(),
    );
    queue.submit([encoder.finish()]);
    let (sender, receiver) = std::sync::mpsc::channel();
    readback.map_async(wgpu::MapMode::Read, .., move |result| {
        sender.send(result).unwrap();
    });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    receiver.recv().unwrap().unwrap();
    let pixels = readback.get_mapped_range(..).unwrap().to_vec();
    readback.unmap();
    pixels
}

#[test]
#[ignore = "requires an accessible GPU adapter"]
#[allow(clippy::too_many_lines)]
fn ecs_cubes_render_offscreen() {
    pollster::block_on(async {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .expect("GPU adapter required for rendering test");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("GPU device");
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let pipeline = create_pipeline(&device, format);
        let mesh = create_cube_mesh(&device);
        let camera = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera.as_entire_binding(),
            }],
        });
        for cube_count in 0..=2 {
            let mut world = World::new();
            let camera_entity = engine::ecs::player::spawn_local_player(&mut world);
            if cube_count > 0 {
                engine::ecs::scene::spawn_cube(&mut world);
            }
            if cube_count > 1 {
                let back = engine::ecs::scene::spawn_cube(&mut world);
                let mut transform = world.get_mut::<Transform>(back).unwrap();
                transform.translation = [-1.5, -1.5, -2.5];
                transform.scale = [3.0; 3];
                transform.rotation = [0.0, 1.0, 0.0, 0.0];
                world.get_mut::<Material>(back).unwrap().color = [0.0, 1.0, 0.0];
            }
            let batches = upload_meshes(&device, extract_meshes(&mut world)).unwrap();
            // Exercise aspect ratios and a camera turned away from the cubes.
            for (width, height, aspect, visible) in [
                (64, 64, 1.0_f32, true),
                (128, 64, 2.0_f32, true),
                (64, 64, 1.0_f32, false),
            ] {
                if !visible {
                    let mut transform = world.get_mut::<Transform>(camera_entity).unwrap();
                    transform.translation = [0.0, 0.0, 5.0];
                    transform.rotation = [0.0, 1.0, 0.0, 0.0];
                }
                let camera_bytes: Vec<u8> = extract_camera(&mut world, aspect)
                    .unwrap()
                    .into_iter()
                    .flat_map(f32::to_le_bytes)
                    .collect();
                queue.write_buffer(&camera, 0, &camera_bytes);
                let target = device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                    view_formats: &[],
                });
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                draw_scene(
                    &mut encoder,
                    &target.create_view(&wgpu::TextureViewDescriptor::default()),
                    &create_depth(&device, width, height),
                    &pipeline,
                    &bind_group,
                    &mesh,
                    &batches,
                );
                let pixels = read_pixels(&device, &queue, encoder, &target);
                if cube_count == 0 || !visible {
                    assert!(pixels.as_chunks::<4>().0.iter().all(|p| p == &pixels[..4]));
                    continue;
                }
                let center = ((height / 2 * width + width / 2) * 4) as usize;
                let marker = &pixels[center..center + 4];
                // The orange cube stays visible when the green cube behind it is drawn later.
                assert!(
                    marker[0] > 150 && marker[1] < 100 && marker[2] < 100,
                    "marker pixel: {marker:?}"
                );
                if cube_count > 1 {
                    assert!(
                        pixels
                            .as_chunks::<4>()
                            .0
                            .iter()
                            .any(|p| p[1] > p[0].saturating_add(20)
                                && p[1] > p[2].saturating_add(20)),
                        "second cube should be visible"
                    );
                }
            }
        }
    });
}
