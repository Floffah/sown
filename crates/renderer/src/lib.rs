//! Shared ECS-driven rendering, independent of the client's event loop.

use std::collections::HashMap;

use engine::ecs::{
    bevy_ecs::prelude::World,
    scene::{Mesh, MeshPrimitive},
};
use wgpu::util::DeviceExt;
use wgpu::{BackendOptions, InstanceFlags, MemoryBudgetThresholds, MemoryHints};

use extraction::{extract_camera, extract_meshes};
use pipeline::create_pipeline;
use primitives::create_cube_mesh;

pub use wgpu;

mod extraction;
mod pipeline;
mod primitives;

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pipeline: wgpu::RenderPipeline,
    camera: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth: Option<wgpu::TextureView>,
    cube_mesh: wgpu::Buffer,
}

impl Renderer {
    /// Creates GPU resources for an owned render target and its initial dimensions.
    /// Call `resize` with nonzero dimensions to configure the surface before drawing.
    ///
    /// # Errors
    /// Returns an error if surface creation, adapter selection, or device creation fails.
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
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

        let surface = instance.create_surface(target)?;

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
                // Use WebGL2-compatible limits for the browser backend.
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
        // Prefer an sRGB surface for the shader's linear colour output.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        let cube_mesh = create_cube_mesh(&device);
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
            pipeline,
            camera,
            camera_bind_group,
            depth: None,
            cube_mesh,
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
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    /// Renders the ECS scene, then calls the host's overlay before presentation.
    /// The overlay returns any command buffers that must run before this frame.
    /// The host schedules another redraw for `Retry`.
    ///
    /// # Panics
    /// Panics if a configured surface has no depth buffer, violating an internal invariant.
    ///
    /// # Errors
    /// Returns an error for invalid camera data, surface validation or loss,
    /// or an instance count that cannot be represented by the GPU draw command.
    pub fn render(
        &mut self,
        world: &mut World,
        overlay: impl FnOnce(RenderFrame<'_>) -> Vec<wgpu::CommandBuffer>,
    ) -> anyhow::Result<RenderOutcome> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(RenderOutcome::Skipped);
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
                return Ok(RenderOutcome::Retry);
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(RenderOutcome::Skipped);
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                anyhow::bail!("Surface validation failed");
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(RenderOutcome::Retry);
            }
            wgpu::CurrentSurfaceTexture::Lost => {
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

        let batches = upload_meshes(&self.device, extract_meshes(world))?;
        draw_scene(
            &mut encoder,
            &view,
            self.depth
                .as_ref()
                .expect("Configured surface has a depth buffer"),
            &self.pipeline,
            &self.camera_bind_group,
            &self.cube_mesh,
            &batches,
        );

        let overlay_commands = overlay(RenderFrame {
            device: &self.device,
            queue: &self.queue,
            encoder: &mut encoder,
            color: &view,
            size_in_pixels: [self.config.width, self.config.height],
        });
        self.queue
            .submit(overlay_commands.into_iter().chain([encoder.finish()]));
        self.queue.present(output);

        Ok(RenderOutcome::Presented)
    }
}

/// GPU resources for drawing an overlay onto the scene's current frame.
pub struct RenderFrame<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub color: &'a wgpu::TextureView,
    pub size_in_pixels: [u32; 2],
}

/// Tells the host whether a frame was drawn or another redraw is needed.
#[derive(Debug, PartialEq, Eq)]
pub enum RenderOutcome {
    Presented,
    /// Rendering was skipped because the surface is unconfigured or occluded.
    Skipped,
    /// The surface timed out or was reconfigured; request another redraw.
    Retry,
}

struct MeshBatch {
    mesh: Mesh,
    instances: wgpu::Buffer,
    instance_count: u32,
}

fn upload_meshes(
    device: &wgpu::Device,
    meshes: HashMap<Mesh, Vec<u8>>,
) -> anyhow::Result<Vec<MeshBatch>> {
    // Create an instance buffer for each mesh batch in this redraw.
    meshes
        .into_iter()
        .map(|(mesh, bytes)| {
            let instance_count = u32::try_from(bytes.len() / 52)?;
            let instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ECS mesh instances"),
                contents: &bytes,
                usage: wgpu::BufferUsages::VERTEX,
            });
            Ok(MeshBatch {
                mesh,
                instances,
                instance_count,
            })
        })
        .collect()
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

fn draw_scene(
    encoder: &mut wgpu::CommandEncoder,
    color: &wgpu::TextureView,
    depth: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    camera: &wgpu::BindGroup,
    cube_mesh: &wgpu::Buffer,
    batches: &[MeshBatch],
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
    for batch in batches {
        let (mesh, vertex_count) = match batch.mesh {
            Mesh::Primitive(MeshPrimitive::Cube) => (cube_mesh, 36),
        };
        pass.set_vertex_buffer(0, mesh.slice(..));
        pass.set_vertex_buffer(1, batch.instances.slice(..));
        pass.draw(0..vertex_count, 0..batch.instance_count);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
