//! A surface the engine records drawing into.
//!
//! One type serves both the per-window canvas and every texture an application creates. Under
//! wgpu they are the same thing, a texture that can be rendered into and sampled from, so a
//! texture belongs to no particular window.
//!
//! Drawing records shapes rather than issuing draw calls. Consecutive shapes sharing a blend mode
//! and an anti-aliasing setting are tessellated and drawn together.

use crate::{
    prelude::{BlendMode, Color, Point, Rect as PixRect},
    renderer::{painter::Run, shapes},
};
use egui::epaint::{ClippedShape, Rect, Shape, Tessellator, TextureId};
use egui_wgpu::wgpu;

/// A recorded shape and the state it was recorded under.
struct Recorded {
    /// How the shape blends against what the target already contains.
    blend_mode: BlendMode,
    /// Whether the shape is anti-aliased.
    smooth: bool,
    /// The shape and the region it is clipped to.
    clipped: ClippedShape,
}

/// A texture the engine draws into and can sample from.
pub(crate) struct RenderTarget {
    /// Backing texture.
    texture: wgpu::Texture,
    /// View used both as a render attachment and as a sampled texture.
    view: wgpu::TextureView,
    /// Identifier the canvas and the UI refer to this target by.
    id: TextureId,
    /// Size in pixels.
    size: [u32; 2],
    /// Shapes recorded since the last flush, in draw order.
    recorded: Vec<Recorded>,
    /// Single-pixel points waiting to be emitted as one mesh.
    points: Vec<(Point<i32>, Color)>,
    /// Blend mode the buffered points were recorded under.
    points_blend: Option<BlendMode>,
    /// Region drawing is confined to, or the whole target.
    clip: Option<PixRect<i32>>,
    /// Scale applied to recorded coordinates.
    scale: [f32; 2],
    /// Translation applied to recorded coordinates.
    offset: Point<i32>,
    /// Color the target is cleared to when a clear is requested.
    background: Color,
    /// Set when the next flush should clear rather than draw over what is there.
    clear_pending: bool,
}

impl RenderTarget {
    /// Allocates a target of the given size.
    ///
    /// `format` matches the surface, so one painter can draw into every target.
    pub(crate) fn new(
        device: &wgpu::Device,
        id: TextureId,
        size: [u32; 2],
        format: wgpu::TextureFormat,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("pix-engine render target"),
            size: wgpu::Extent3d {
                width: size[0].max(1),
                height: size[1].max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            id,
            size: [size[0].max(1), size[1].max(1)],
            recorded: Vec::new(),
            points: Vec::new(),
            points_blend: None,
            clip: None,
            scale: [1.0, 1.0],
            offset: Point::new([0, 0]),
            background: Color::BLACK,
            // The first flush paints the background, so a fresh target does not show whatever the
            // allocation happened to contain.
            clear_pending: true,
        }
    }

    /// Returns the identifier the canvas and the UI refer to this target by.
    pub(crate) const fn id(&self) -> TextureId {
        self.id
    }

    /// Returns the view used as a render attachment and as a sampled texture.
    pub(crate) const fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// Returns the backing texture, for readback.
    pub(crate) const fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Returns the size in pixels.
    pub(crate) const fn size(&self) -> [u32; 2] {
        self.size
    }

    /// Sets the color the next clear paints.
    pub(crate) const fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    /// Reallocates the target at a new size, discarding its contents.
    ///
    /// The backing texture has a fixed size, so a resize is a new allocation. The next flush
    /// paints the background, which keeps the discarded contents from showing through.
    pub(crate) fn resize(
        &mut self,
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
    ) {
        let background = self.background;
        let clip = self.clip;
        let scale = self.scale;
        let offset = self.offset;
        *self = Self::new(device, self.id, size, format);
        self.background = background;
        self.clip = clip;
        self.scale = scale;
        self.offset = offset;
    }

    /// Keeps what the target already contains, cancelling a clear it has not painted yet.
    ///
    /// A target is armed to clear when it is allocated. Writing pixels into it defines its
    /// contents, and clearing over them afterwards would show the background instead.
    pub(crate) const fn keep_contents(&mut self) {
        self.clear_pending = false;
    }

    /// Reports whether the next flush starts by painting the background.
    pub(crate) const fn clear_pending(&self) -> bool {
        self.clear_pending
    }

    /// Requests that the next flush start from the background color.
    ///
    /// Recorded shapes are dropped, because anything drawn before a clear would be painted over.
    pub(crate) fn clear(&mut self) {
        self.recorded.clear();
        self.points.clear();
        self.points_blend = None;
        self.clear_pending = true;
    }

    /// Confines subsequent drawing to a region, or to the whole target with `None`.
    pub(crate) fn set_clip(&mut self, clip: Option<PixRect<i32>>) {
        // Buffered points are clipped and transformed when they are emitted, so they go out under
        // the state they were recorded with.
        self.flush_points();
        self.clip = clip;
    }

    /// Returns the region drawing is confined to.
    pub(crate) const fn clip(&self) -> Option<PixRect<i32>> {
        self.clip
    }

    /// Scales subsequent drawing.
    pub(crate) fn set_scale(&mut self, x: f32, y: f32) {
        self.flush_points();
        self.scale = [x, y];
    }

    /// Translates subsequent drawing.
    pub(crate) fn set_offset(&mut self, offset: Point<i32>) {
        self.flush_points();
        self.offset = offset;
    }

    /// Records a shape to be drawn at the next flush.
    pub(crate) fn record(&mut self, shape: Shape, blend_mode: BlendMode, smooth: bool) {
        self.flush_points();
        self.push(shape, blend_mode, smooth);
    }

    /// Records a single-pixel point, holding it back so consecutive points share one mesh.
    ///
    /// An application drawing per pixel emits thousands of these a frame, and a shape apiece
    /// makes the tessellator walk a long list for the same geometry.
    pub(crate) fn record_point(&mut self, p: Point<i32>, color: Color, blend_mode: BlendMode) {
        if self.points_blend != Some(blend_mode) {
            self.flush_points();
            self.points_blend = Some(blend_mode);
        }
        self.points.push((p, color));
    }

    /// Emits the buffered points as one mesh, so draw order is preserved.
    fn flush_points(&mut self) {
        if self.points.is_empty() {
            self.points_blend = None;
            return;
        }
        let blend_mode = self.points_blend.take().unwrap_or(BlendMode::None);
        let points = std::mem::take(&mut self.points);
        self.push(shapes::points(&points), blend_mode, false);
    }

    /// Adds a shape to the recording, applying the current scale and translation.
    fn push(&mut self, shape: Shape, blend_mode: BlendMode, smooth: bool) {
        if matches!(shape, Shape::Noop) {
            return;
        }
        let mut shape = shape;
        // Scale and translation are folded in at record time. `TSTransform` scales both axes
        // together, and the drawing API allows a different factor per axis.
        if self.scale != [1.0, 1.0] || self.offset != Point::new([0, 0]) {
            transform(&mut shape, self.scale, self.offset);
        }
        let clip_rect = self.clip.map_or_else(
            || {
                #[allow(clippy::cast_precision_loss)]
                Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::Vec2::new(self.size[0] as f32, self.size[1] as f32),
                )
            },
            shapes::rect,
        );
        self.recorded.push(Recorded {
            blend_mode,
            smooth,
            clipped: ClippedShape { clip_rect, shape },
        });
    }

    /// Tessellates everything recorded, grouping shapes that can be drawn together.
    ///
    /// A run ends where the blend mode or the anti-aliasing setting changes, because feathering is
    /// a tessellator-wide option and the blend mode selects a pipeline. Most frames produce one
    /// run.
    ///
    /// `font_tex_size` is the size of the atlas glyph coordinates are measured against, so it has
    /// to be read after the last text was laid out.
    pub(crate) fn flush(&mut self, pixels_per_point: f32, font_tex_size: [usize; 2]) -> Vec<Run> {
        self.flush_points();
        let tessellate = |shapes: Vec<ClippedShape>, smooth: bool| {
            let options = egui::epaint::TessellationOptions {
                feathering: smooth,
                ..egui::epaint::TessellationOptions::default()
            };
            // Precomputed discs speed up rounded corners and `epaint` documents an empty list as
            // safe, so it recomputes them instead. `PreparedDisc` cannot be named from here.
            Tessellator::new(pixels_per_point, options, font_tex_size, Vec::new())
                .tessellate_shapes(shapes)
        };

        let mut runs: Vec<Run> = Vec::new();
        let mut batch: Vec<ClippedShape> = Vec::new();
        let mut current: Option<(BlendMode, bool)> = None;

        for Recorded {
            blend_mode,
            smooth,
            clipped,
        } in self.recorded.drain(..)
        {
            if current != Some((blend_mode, smooth)) {
                if let Some((mode, was_smooth)) = current.take() {
                    runs.push(Run {
                        blend_mode: mode,
                        primitives: tessellate(std::mem::take(&mut batch), was_smooth),
                    });
                }
                current = Some((blend_mode, smooth));
            }
            batch.push(clipped);
        }
        if let Some((mode, smooth)) = current {
            runs.push(Run {
                blend_mode: mode,
                primitives: tessellate(batch, smooth),
            });
        }
        runs
    }

    /// Returns the load operation the next pass should start with, consuming a pending clear.
    pub(crate) fn take_load_op(&mut self) -> wgpu::LoadOp<wgpu::Color> {
        if self.clear_pending {
            self.clear_pending = false;
            let [r, g, b, a] = self.background.channels();
            wgpu::LoadOp::Clear(wgpu::Color {
                r: linear(r),
                g: linear(g),
                b: linear(b),
                a: f64::from(a) / 255.0,
            })
        } else {
            // Keeping what is already there makes the canvas persistent between frames.
            wgpu::LoadOp::Load
        }
    }
}

/// Converts an sRGB channel into the linear value a clear writes.
///
/// A target is allocated in an sRGB format, and the hardware encodes what a pass clears it to.
/// Handing it the sRGB byte would encode it twice and wash the color out.
fn linear(channel: u8) -> f64 {
    let channel = f64::from(channel) / 255.0;
    if channel <= 0.040_45 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

/// Applies a per-axis scale and a translation to a shape.
fn transform(shape: &mut Shape, scale: [f32; 2], offset: Point<i32>) {
    #[allow(clippy::cast_precision_loss)]
    let offset = egui::Vec2::new(offset.x() as f32, offset.y() as f32);
    // A uniform scale goes through `TSTransform`. A different factor per axis does not fit
    // that, so the shape is walked and its positions scaled directly.
    if (scale[0] - scale[1]).abs() < f32::EPSILON {
        shape.transform(egui::emath::TSTransform {
            scaling: scale[0],
            translation: offset,
        });
    } else {
        scale_shape(shape, scale, offset);
    }
}

/// Scales every position in a shape by a different factor per axis.
fn scale_shape(shape: &mut Shape, scale: [f32; 2], offset: egui::Vec2) {
    let apply = |pos: &mut egui::Pos2| {
        pos.x = pos.x * scale[0] + offset.x;
        pos.y = pos.y * scale[1] + offset.y;
    };
    match shape {
        Shape::Noop | Shape::Callback(_) => {}
        Shape::Vec(shapes) => {
            for shape in shapes {
                scale_shape(shape, scale, offset);
            }
        }
        Shape::Circle(circle) => {
            apply(&mut circle.center);
            circle.radius *= scale[0];
        }
        Shape::Ellipse(ellipse) => {
            apply(&mut ellipse.center);
            ellipse.radius.x *= scale[0];
            ellipse.radius.y *= scale[1];
        }
        Shape::LineSegment { points, .. } => {
            for point in points {
                apply(point);
            }
        }
        Shape::Path(path) => {
            for point in &mut path.points {
                apply(point);
            }
        }
        Shape::Rect(rect) => {
            apply(&mut rect.rect.min);
            apply(&mut rect.rect.max);
        }
        Shape::Text(text) => {
            apply(&mut text.pos);
        }
        Shape::Mesh(mesh) => {
            let mesh = std::sync::Arc::make_mut(mesh);
            for vertex in &mut mesh.vertices {
                apply(&mut vertex.pos);
            }
        }
        Shape::QuadraticBezier(curve) => {
            for point in &mut curve.points {
                apply(point);
            }
        }
        Shape::CubicBezier(curve) => {
            for point in &mut curve.points {
                apply(point);
            }
        }
    }
}
