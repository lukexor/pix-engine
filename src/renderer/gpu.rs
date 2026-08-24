//! Device setup and the copy that puts a canvas on screen.
//!
//! The adapter is chosen before any window exists, because a window can only be created once the
//! event loop is running while [`Rendering::new`] runs before it. Every render target is allocated
//! in [`TARGET_FORMAT`] rather than in whatever a surface happens to prefer, so one painter serves
//! every target. [`Blit`] copies a canvas onto a surface of any format.
//!
//! [`Rendering::new`]: crate::renderer::Rendering::new

use crate::error::Result;
use anyhow::{anyhow, Context};
use egui_wgpu::wgpu;
use std::collections::HashMap;

/// Format every render target is allocated in.
///
/// sRGB, so the shader writes linear values and the hardware encodes them, matching what `epaint`
/// expects. One format lets a single painter draw into the canvas and into every texture an
/// application creates.
pub(crate) const TARGET_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

/// The graphics device and the queue that feeds it.
pub(crate) struct Gpu {
    /// Backend entry point, kept because surfaces are created from it.
    pub(crate) instance: wgpu::Instance,
    /// Physical device the surfaces are configured against.
    pub(crate) adapter: wgpu::Adapter,
    /// Logical device every resource is allocated from.
    pub(crate) device: wgpu::Device,
    /// Queue every upload and command buffer is submitted to.
    pub(crate) queue: wgpu::Queue,
}

impl Gpu {
    /// Opens a device, without requiring a surface to exist yet.
    ///
    /// # Errors
    ///
    /// Returns an error if no adapter is available, or if the adapter refuses a device.
    pub(crate) fn new() -> Result<Self> {
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // No window exists yet. Every desktop adapter can present to a window it did not
            // name here. A browser cannot, which is one reason wasm is still unsupported.
            compatible_surface: None,
        }))
        .map_err(|err| anyhow!("no graphics adapter available: {err}"))?;

        let info = adapter.get_info();
        log::debug!(
            "Using {:?} adapter `{}` on {:?}",
            info.device_type,
            info.name,
            info.backend
        );

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("pix-engine device"),
            required_features: wgpu::Features::empty(),
            required_limits: adapter.limits(),
            ..wgpu::DeviceDescriptor::default()
        }))
        .context("failed to open a graphics device")?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}

/// Copies a canvas onto a surface, stretched to fill it.
///
/// A canvas is allocated in [`TARGET_FORMAT`] and a surface takes whatever format it reports, so
/// the copy is also where the two formats meet. A pipeline is built per surface format and cached,
/// because the target format is baked into a pipeline and two monitors can report different ones.
pub(crate) struct Blit {
    /// Shader shared by every pipeline.
    shader: wgpu::ShaderModule,
    /// Layout the source bind group is built against.
    layout: wgpu::BindGroupLayout,
    /// Pipeline layout shared by every pipeline.
    pipeline_layout: wgpu::PipelineLayout,
    /// One pipeline per surface format seen so far.
    pipelines: HashMap<wgpu::TextureFormat, wgpu::RenderPipeline>,
    /// Sampler the source is read through.
    sampler: wgpu::Sampler,
}

impl Blit {
    /// Builds the shared pieces. Pipelines are built as formats turn up.
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("pix-engine blit shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("blit.wgsl").into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("pix-engine blit layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pix-engine blit pipeline layout"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("pix-engine blit sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            ..wgpu::SamplerDescriptor::default()
        });
        Self {
            shader,
            layout,
            pipeline_layout,
            pipelines: HashMap::new(),
            sampler,
        }
    }

    /// Draws `source` over the whole of `destination`.
    pub(crate) fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source: &wgpu::TextureView,
        destination: &wgpu::TextureView,
        format: wgpu::TextureFormat,
    ) {
        if !self.pipelines.contains_key(&format) {
            let pipeline = self.build_pipeline(device, format);
            self.pipelines.insert(format, pipeline);
        }
        #[allow(clippy::expect_used)]
        let pipeline = self
            .pipelines
            .get(&format)
            .expect("pipeline was just built");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("pix-engine blit bind group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pix-engine blit pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: destination,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    // The copy covers the whole surface, so nothing needs clearing first.
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);
    }

    /// Builds the pipeline that writes to a surface of the given format.
    fn build_pipeline(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pix-engine blit pipeline"),
            layout: Some(&self.pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }
}
