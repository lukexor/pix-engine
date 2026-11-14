//! Canvas painter.
//!
//! Draws `epaint` meshes with one pipeline per [`BlendMode`]. The pipelines share a shader and a
//! vertex layout, and differ only in blend state, so selecting one applies `blend_mode`.
//!
//! One set of vertex and index buffers serves every target, so a target is painted and submitted
//! before the next one uploads over them.

use crate::prelude::BlendMode;
use ahash::{HashMap, HashMapExt};
use egui::epaint::{ClippedPrimitive, ImageDelta, Primitive, TextureId, Vertex};
use egui_wgpu::wgpu;
use log::warn;
use std::{mem, num::NonZeroU64, ops::Range};

/// Vertices a freshly created buffer makes room for.
const INITIAL_VERTICES: u64 = 1024;

/// Indices a freshly created buffer makes room for.
const INITIAL_INDICES: u64 = 1536;

/// Screen size handed to the shader, padded to the 16 bytes WebGL requires of a uniform buffer.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    /// Canvas size in pixels.
    screen_size: [f32; 2],
    /// Padding to reach 16 bytes.
    padding: [u32; 2],
}

/// A span of canvas shapes sharing one blend mode, already tessellated.
pub(crate) struct Run {
    /// How this span blends against what the canvas already contains.
    pub(crate) blend_mode: BlendMode,
    /// Tessellated geometry, in draw order.
    pub(crate) primitives: Vec<ClippedPrimitive>,
}

/// A growable GPU buffer.
struct Buffer {
    /// The allocation.
    handle: wgpu::Buffer,
    /// Size of the allocation in bytes.
    capacity: u64,
}

impl Buffer {
    /// Allocates a buffer of `capacity` bytes.
    fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, capacity: u64, label: &str) -> Self {
        Self {
            handle: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: capacity,
                usage: usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            capacity,
        }
    }

    /// Grows the buffer to fit at least `needed` bytes, doubling to avoid reallocating per frame.
    fn reserve(
        &mut self,
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        needed: u64,
        label: &str,
    ) {
        if needed <= self.capacity {
            return;
        }
        let capacity = needed.next_power_of_two();
        *self = Self::new(device, usage, capacity, label);
    }
}

/// Draws canvas geometry, one pipeline per blend mode.
pub(crate) struct Painter {
    /// One pipeline per [`BlendMode`], indexed by [`blend_index`].
    pipelines: Vec<wgpu::RenderPipeline>,
    /// Screen-size uniform, rewritten whenever the target resizes.
    uniform_buffer: wgpu::Buffer,
    /// Bind group for [`Painter::uniform_buffer`].
    uniform_bind_group: wgpu::BindGroup,
    /// Layout every texture bind group is built against.
    texture_layout: wgpu::BindGroupLayout,
    /// Every texture the canvas can draw with, by the identifier a mesh names.
    textures: HashMap<TextureId, Texture>,
    /// Vertex data for the frame being drawn.
    vertices: Buffer,
    /// Index data for the frame being drawn.
    indices: Buffer,
}

impl Painter {
    /// Builds the pipelines and buffers for a target of the given format.
    pub(crate) fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("pix-engine canvas shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("canvas.wgsl").into()),
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("pix-engine canvas uniform layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(mem::size_of::<Uniforms>() as u64),
                },
                count: None,
            }],
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("pix-engine canvas texture layout"),
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

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("pix-engine canvas uniforms"),
            size: mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("pix-engine canvas uniform bind group"),
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pix-engine canvas pipeline layout"),
            bind_group_layouts: &[Some(&uniform_layout), Some(&texture_layout)],
            immediate_size: 0,
        });

        let pipelines = BLEND_MODES
            .iter()
            .map(|mode| {
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("pix-engine canvas pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: mem::size_of::<Vertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![
                                0 => Float32x2,
                                1 => Float32x2,
                                2 => Uint32,
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(blend_state(*mode)),
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
            })
            .collect();

        Self {
            pipelines,
            uniform_buffer,
            uniform_bind_group,
            texture_layout,
            textures: HashMap::new(),
            vertices: Buffer::new(
                device,
                wgpu::BufferUsages::VERTEX,
                INITIAL_VERTICES * mem::size_of::<Vertex>() as u64,
                "pix-engine canvas vertices",
            ),
            indices: Buffer::new(
                device,
                wgpu::BufferUsages::INDEX,
                INITIAL_INDICES * mem::size_of::<u32>() as u64,
                "pix-engine canvas indices",
            ),
        }
    }

    /// Uploads or patches a texture `epaint` asked for.
    ///
    /// The font atlas arrives this way. A delta with a position patches the rows of glyphs packed
    /// since the last frame, so the texture behind it has to be kept. A delta without one is a
    /// whole new image, which happens when the atlas is repacked or grown.
    pub(crate) fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: TextureId,
        delta: &ImageDelta,
    ) {
        let egui::epaint::ImageData::Color(image) = &delta.image;
        let [width, height] = image.size;
        #[allow(clippy::cast_possible_truncation)]
        let size = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };
        let pixels: Vec<u8> = image
            .pixels
            .iter()
            .flat_map(egui::Color32::to_array)
            .collect();
        let filter = match delta.options.magnification {
            egui::TextureFilter::Nearest => wgpu::FilterMode::Nearest,
            egui::TextureFilter::Linear => wgpu::FilterMode::Linear,
        };

        let origin = match delta.pos {
            Some([x, y]) => {
                let Some(Texture::Owned { texture, .. }) = self.textures.get(&id) else {
                    warn!("canvas texture patch for {id:?} arrived without a base texture");
                    return;
                };
                #[allow(clippy::cast_possible_truncation)]
                let origin = wgpu::Origin3d {
                    x: x as u32,
                    y: y as u32,
                    z: 0,
                };
                let texture = texture.clone();
                write_texture(queue, &texture, origin, size, &pixels);
                return;
            }
            None => wgpu::Origin3d::ZERO,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("pix-engine canvas texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        write_texture(queue, &texture, origin, size, &pixels);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.bind_texture(device, &view, filter);
        self.textures.insert(
            id,
            Texture::Owned {
                texture,
                bind_group,
            },
        );
    }

    /// Registers a texture the engine owns, so the canvas can draw it.
    pub(crate) fn register_texture(
        &mut self,
        device: &wgpu::Device,
        id: TextureId,
        view: &wgpu::TextureView,
        filter: wgpu::FilterMode,
    ) {
        let bind_group = self.bind_texture(device, view, filter);
        self.textures.insert(id, Texture::Borrowed { bind_group });
    }

    /// Drops a texture the canvas no longer draws.
    pub(crate) fn free_texture(&mut self, id: TextureId) {
        self.textures.remove(&id);
    }

    /// Builds the bind group a pipeline samples a texture through.
    fn bind_texture(
        &self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        filter: wgpu::FilterMode,
    ) -> wgpu::BindGroup {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("pix-engine canvas sampler"),
            mag_filter: filter,
            min_filter: filter,
            ..wgpu::SamplerDescriptor::default()
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("pix-engine canvas texture bind group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        })
    }

    /// Draws every run into `view`.
    ///
    /// `load` decides whether what the target already contains survives, which keeps a canvas
    /// persistent between frames.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn paint(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        load: wgpu::LoadOp<wgpu::Color>,
        size: [u32; 2],
        runs: &[Run],
    ) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&Uniforms {
                #[allow(clippy::cast_precision_loss)]
                screen_size: [size[0] as f32, size[1] as f32],
                padding: [0, 0],
            }),
        );

        let draws = self.upload(device, queue, runs);

        let mut pass = encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("pix-engine canvas pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
            .forget_lifetime();

        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertices.handle.slice(..));
        pass.set_index_buffer(self.indices.handle.slice(..), wgpu::IndexFormat::Uint32);

        for draw in draws {
            let Some(texture) = self.textures.get(&draw.texture) else {
                warn!("canvas draw references unknown texture {:?}", draw.texture);
                continue;
            };
            let Some(pipeline) = self.pipelines.get(blend_index(draw.blend_mode)) else {
                continue;
            };
            // A clip rect comes from the drawing API and can name a region larger than the target,
            // which a scissor rect may not.
            let (x, y) = (draw.clip.0.min(size[0]), draw.clip.1.min(size[1]));
            let width = draw.clip.2.min(size[0] - x);
            let height = draw.clip.3.min(size[1] - y);
            if width == 0 || height == 0 {
                continue;
            }
            pass.set_pipeline(pipeline);
            pass.set_bind_group(1, texture.bind_group(), &[]);
            pass.set_scissor_rect(x, y, width, height);
            pass.draw_indexed(draw.indices.clone(), draw.base_vertex, 0..1);
        }
    }

    /// Copies every run's geometry into the buffers, returning what to draw.
    fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, runs: &[Run]) -> Vec<Draw> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut draws = Vec::new();

        for run in runs {
            for primitive in &run.primitives {
                let Primitive::Mesh(mesh) = &primitive.primitive else {
                    // A paint callback is an egui feature the canvas API cannot produce.
                    continue;
                };
                if mesh.indices.is_empty() {
                    continue;
                }
                #[allow(clippy::cast_possible_truncation)]
                let base_vertex = vertices.len() as i32;
                let start = indices.len() as u32;
                vertices.extend_from_slice(&mesh.vertices);
                indices.extend_from_slice(&mesh.indices);
                #[allow(clippy::cast_possible_truncation)]
                let end = indices.len() as u32;
                let clip = primitive.clip_rect;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let clip = (
                    clip.min.x.max(0.0) as u32,
                    clip.min.y.max(0.0) as u32,
                    clip.width().max(0.0) as u32,
                    clip.height().max(0.0) as u32,
                );
                draws.push(Draw {
                    blend_mode: run.blend_mode,
                    texture: mesh.texture_id,
                    indices: start..end,
                    base_vertex,
                    clip,
                });
            }
        }

        let vertex_bytes = bytemuck::cast_slice(&vertices);
        let index_bytes = bytemuck::cast_slice(&indices);
        self.vertices.reserve(
            device,
            wgpu::BufferUsages::VERTEX,
            vertex_bytes.len() as u64,
            "pix-engine canvas vertices",
        );
        self.indices.reserve(
            device,
            wgpu::BufferUsages::INDEX,
            index_bytes.len() as u64,
            "pix-engine canvas indices",
        );
        if !vertex_bytes.is_empty() {
            queue.write_buffer(&self.vertices.handle, 0, vertex_bytes);
            queue.write_buffer(&self.indices.handle, 0, index_bytes);
        }
        draws
    }
}

/// A texture the canvas can sample.
enum Texture {
    /// A texture `epaint` asked for, kept so a later delta can patch it.
    Owned {
        /// The allocation a patch is written into.
        texture: wgpu::Texture,
        /// Bind group the pipeline samples it through.
        bind_group: wgpu::BindGroup,
    },
    /// A texture the engine allocated elsewhere and only lends to the painter.
    Borrowed {
        /// Bind group the pipeline samples it through.
        bind_group: wgpu::BindGroup,
    },
}

impl Texture {
    /// Returns the bind group the pipeline samples this texture through.
    const fn bind_group(&self) -> &wgpu::BindGroup {
        match self {
            Self::Owned { bind_group, .. } | Self::Borrowed { bind_group } => bind_group,
        }
    }
}

/// Writes tightly packed RGBA pixels into a region of a texture.
fn write_texture(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    origin: wgpu::Origin3d,
    size: wgpu::Extent3d,
    pixels: &[u8],
) {
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin,
            aspect: wgpu::TextureAspect::All,
        },
        pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: Some(size.height),
        },
        size,
    );
}

/// One indexed draw call.
struct Draw {
    /// Pipeline to draw it with.
    blend_mode: BlendMode,
    /// Texture the mesh samples.
    texture: TextureId,
    /// Range within the shared index buffer.
    indices: Range<u32>,
    /// Offset into the shared vertex buffer.
    base_vertex: i32,
    /// Scissor rect as `(x, y, width, height)`.
    clip: (u32, u32, u32, u32),
}

/// Blend modes, in the order [`blend_index`] reports.
const BLEND_MODES: [BlendMode; 4] = [
    BlendMode::None,
    BlendMode::Blend,
    BlendMode::Add,
    BlendMode::Mod,
];

/// Returns the pipeline slot for a blend mode.
const fn blend_index(mode: BlendMode) -> usize {
    match mode {
        BlendMode::None => 0,
        BlendMode::Blend => 1,
        BlendMode::Add => 2,
        BlendMode::Mod => 3,
    }
}

/// Returns the blend state a mode draws with.
///
/// `epaint` emits premultiplied alpha, so [`BlendMode::Blend`] takes the source unscaled.
const fn blend_state(mode: BlendMode) -> wgpu::BlendState {
    match mode {
        BlendMode::None => wgpu::BlendState::REPLACE,
        BlendMode::Blend => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::OneMinusDstAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
        BlendMode::Add => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Zero,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
        // Modulate: what is drawn scales what is already there.
        BlendMode::Mod => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Dst,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Zero,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
    }
}
