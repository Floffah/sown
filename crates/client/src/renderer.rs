use std::sync::Arc;

use engine::ecs::{
    bevy_ecs::prelude::World,
    scene::{Camera, Material, Mesh, Transform},
};
use wgpu::util::DeviceExt;
use wgpu::{BackendOptions, InstanceFlags, MemoryBudgetThresholds, MemoryHints};
use winit::window::Window;

pub(crate) struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    pipeline: wgpu::RenderPipeline,
    camera: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth: Option<wgpu::TextureView>,
    mesh: wgpu::Buffer,
}

impl Renderer {
    /// # Errors
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        let mesh = create_cube_mesh(&device);
        let pipeline = create_pipeline(&device, surface_format);
        let camera = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ECS camera"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera.as_entire_binding(),
            }],
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            pipeline,
            camera,
            camera_bind_group,
            depth: None,
            mesh,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.is_surface_configured = width > 0 && height > 0;
        if self.is_surface_configured {
            let max = self.device.limits().max_texture_dimension_2d;
            self.config.width = width.min(max);
            self.config.height = height.min(max);
            self.surface.configure(&self.device, &self.config);
            self.depth = Some(create_depth(
                &self.device,
                self.config.width,
                self.config.height,
            ));
            self.window.request_redraw();
        }
    }

    pub(crate) fn render(&mut self, world: &mut World) -> anyhow::Result<()> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // Texture dimensions fit within the useful precision of a GPU float.
        #[allow(clippy::cast_precision_loss)]
        let aspect = self.config.width as f32 / self.config.height as f32;
        let camera = extract_camera(world, aspect)?;
        let camera_bytes: Vec<u8> = camera.into_iter().flat_map(f32::to_le_bytes).collect();
        self.queue.write_buffer(&self.camera, 0, &camera_bytes);

        let output = match self.surface.get_current_texture() {
            #[allow(clippy::match_same_arms)]
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout => {
                self.window.request_redraw();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                anyhow::bail!("Surface validation failed");
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                self.window.request_redraw();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // You could recreate the devices and all resources
                // created with it here, but we'll just bail
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let instances = extract_cubes(world);
        let object_count = u32::try_from(instances.len() / 52)?;
        let objects = upload_objects(&self.device, &instances);
        draw_scene(
            &mut encoder,
            &view,
            self.depth
                .as_ref()
                .expect("Configured surface has a depth buffer"),
            &self.pipeline,
            &self.camera_bind_group,
            &self.mesh,
            &objects,
            object_count,
        );

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

fn extract_camera(world: &mut World, aspect: f32) -> anyhow::Result<[f32; 12]> {
    let mut cameras = world.query::<(&Camera, &Transform)>();
    let (camera, transform) = cameras.single(world).map_err(|error| {
        anyhow::anyhow!("Expected exactly one camera with a Transform: {error}")
    })?;
    anyhow::ensure!(
        camera.fov_y_radians.is_finite()
            && camera.fov_y_radians > 0.0
            && camera.fov_y_radians < std::f32::consts::PI
            && camera.near.is_finite()
            && camera.near > 0.0
            && camera.far.is_finite()
            && camera.far > camera.near,
        "Camera requires a field of view between 0 and pi radians and 0 < near < far"
    );
    let length = transform
        .rotation
        .iter()
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();
    anyhow::ensure!(
        transform.translation.iter().all(|value| value.is_finite())
            && length.is_finite()
            && length > 0.0,
        "Camera requires a finite position and a finite, nonzero quaternion"
    );
    let [x, y, z] = transform.translation;
    let [qx, qy, qz, qw] = transform.rotation.map(|value| value / length);
    Ok([
        x,
        y,
        z,
        0.0,
        qx,
        qy,
        qz,
        qw,
        1.0 / (camera.fov_y_radians / 2.0).tan(),
        aspect,
        camera.near,
        camera.far,
    ])
}

fn create_cube_mesh(device: &wgpu::Device) -> wgpu::Buffer {
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

// Each instance contains translation (3), rotation (4), scale (3), and colour (3).
fn extract_cubes(world: &mut World) -> Vec<u8> {
    world
        .query::<(&Transform, &Mesh, &Material)>()
        .iter(world)
        .flat_map(|(transform, mesh, material)| match mesh {
            Mesh::Cube => transform
                .translation
                .into_iter()
                .chain(transform.rotation)
                .chain(transform.scale)
                .chain(material.color),
        })
        .flat_map(f32::to_le_bytes)
        .collect()
}

fn upload_objects(device: &wgpu::Device, bytes: &[u8]) -> wgpu::Buffer {
    // shortcut: recreate the instance buffer per redraw; reuse it when allocation becomes a bottleneck.
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("ECS cube instances"),
        // Keep a valid vertex buffer bound even when there are no instances to draw.
        contents: if bytes.is_empty() { &[0; 52] } else { bytes },
        usage: wgpu::BufferUsages::VERTEX,
    })
}

fn create_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Cube shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("cube.wgsl").into()),
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Cube pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[
                Some(wgpu::VertexBufferLayout {
                    array_stride: 12,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                }),
                Some(wgpu::VertexBufferLayout {
                    array_stride: 52,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32x4, 3 => Float32x3, 4 => Float32x3],
                }),
            ],
        },
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

fn create_depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth buffer"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

#[allow(clippy::too_many_arguments)]
fn draw_scene(
    encoder: &mut wgpu::CommandEncoder,
    color: &wgpu::TextureView,
    depth: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    camera: &wgpu::BindGroup,
    mesh: &wgpu::Buffer,
    objects: &wgpu::Buffer,
    object_count: u32,
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Cube render pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: color,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Discard,
            }),
            stencil_ops: None,
        }),
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, camera, &[]);
    pass.set_vertex_buffer(0, mesh.slice(..));
    pass.set_vertex_buffer(1, objects.slice(..));
    pass.draw(0..36, 0..object_count);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn camera_uniform_follows_ecs_changes() {
        let mut world = World::new();
        assert!(extract_camera(&mut world, 1.0).is_err());
        let entity = engine::ecs::scene::spawn_camera(&mut world);
        let camera = extract_camera(&mut world, 2.0).unwrap();
        assert_eq!(&camera[..4], &[3.0, 3.0, 5.0, 0.0]);
        assert!((camera[8] - 3.0_f32.sqrt()).abs() < 0.00001);
        assert_eq!(&camera[9..], &[2.0, 0.1, 100.0]);

        world.get_mut::<Transform>(entity).unwrap().translation = [0.0, 0.0, 5.0];
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0, 0.0, 0.0, 2.0];
        world.get_mut::<Camera>(entity).unwrap().fov_y_radians = std::f32::consts::FRAC_PI_2;
        world.get_mut::<Camera>(entity).unwrap().near = 1.0;
        world.get_mut::<Camera>(entity).unwrap().far = 10.0;
        let camera = extract_camera(&mut world, 1.0).unwrap();
        assert_eq!(&camera[..8], &[0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        assert!((camera[8] - 1.0).abs() < 0.00001);
        assert_eq!(&camera[9..], &[1.0, 1.0, 10.0]);

        world.get_mut::<Camera>(entity).unwrap().near = 10.0;
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.get_mut::<Camera>(entity).unwrap().near = 1.0;
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0; 4];
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0, 0.0, 0.0, 1.0];
        let second = engine::ecs::scene::spawn_camera(&mut world);
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.despawn(second);
        world.entity_mut(entity).remove::<Transform>();
        assert!(extract_camera(&mut world, 1.0).is_err());
    }

    #[test]
    fn cube_instances_follow_ecs_changes() {
        let mut world = World::new();
        assert_eq!(extract_cubes(&mut world), []);
        world.spawn(Mesh::Cube);
        assert_eq!(extract_cubes(&mut world), []);

        let cube = engine::ecs::scene::spawn_cube(&mut world);
        world.get_mut::<Transform>(cube).unwrap().translation = [1.0, 2.0, 3.0];
        world.get_mut::<Transform>(cube).unwrap().rotation = [0.0, 0.0, 1.0, 0.0];
        world.get_mut::<Transform>(cube).unwrap().scale = [2.0, 3.0, 4.0];
        world.get_mut::<Material>(cube).unwrap().color = [0.0, 1.0, 0.0];
        let expected: Vec<u8> = [
            1.0_f32, 2.0, 3.0, 0.0, 0.0, 1.0, 0.0, 2.0, 3.0, 4.0, 0.0, 1.0, 0.0,
        ]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect();
        assert_eq!(extract_cubes(&mut world), expected);

        let second = engine::ecs::scene::spawn_cube(&mut world);
        assert_eq!(extract_cubes(&mut world).len(), 2 * 52);
        world.entity_mut(second).remove::<Material>();
        assert_eq!(extract_cubes(&mut world), expected);
        world.despawn(cube);
        assert_eq!(extract_cubes(&mut world), []);
    }

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
                let camera_entity = engine::ecs::scene::spawn_camera(&mut world);
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
                let instances = extract_cubes(&mut world);
                let objects = upload_objects(&device, &instances);
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
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::COPY_SRC,
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
                        &objects,
                        cube_count,
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
}
