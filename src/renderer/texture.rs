//! Textures an application creates and draws into.
//!
//! A texture is the same kind of [`RenderTarget`] as a window canvas, so it belongs to no
//! particular window and the same drawing methods work against either.

use crate::{
    error::{Error, Result},
    prelude::*,
    renderer::{
        backend::{premultiply, Renderer},
        gpu::TARGET_FORMAT,
        shapes,
        target::RenderTarget,
    },
    texture::TextureRenderer,
};
use anyhow::anyhow;
use egui_wgpu::wgpu;

impl TextureRenderer for Renderer {
    /// Creates a texture to draw into.
    ///
    /// Every target is allocated in [`TARGET_FORMAT`], so a requested [`PixelFormat`] only decides
    /// how [`TextureRenderer::update_texture`] reads the pixels handed to it.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        _format: Option<PixelFormat>,
    ) -> Result<TextureId> {
        let texture_id = TextureId(self.next_texture_id);
        self.next_texture_id += 1;
        let paint_id = self.next_paint_id();
        let target = RenderTarget::new(&self.gpu.device, paint_id, [width, height], TARGET_FORMAT);
        self.painter.register_texture(
            &self.gpu.device,
            paint_id,
            target.view(),
            wgpu::FilterMode::Nearest,
        );
        self.textures.insert(texture_id, target);
        self.texture_order.push(texture_id);
        Ok(texture_id)
    }

    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        let Some(target) = self.textures.remove(&texture_id) else {
            return Err(Error::InvalidTexture(texture_id).into());
        };
        self.pending_free.push(target.id());
        self.texture_order.retain(|open| *open != texture_id);
        if self.texture_target == Some(texture_id) {
            self.texture_target = None;
        }
        Ok(())
    }

    /// Writes pixel data into a region of a texture.
    ///
    /// `pitch` is the length of a source row in bytes, and the channel count is read from it. A
    /// row three bytes per pixel wide is widened to the four channels the target stores.
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> Result<()> {
        let target = self
            .textures
            .get_mut(&texture_id)
            .ok_or_else(|| anyhow!(Error::InvalidTexture(texture_id)))?;
        target.keep_contents();
        let [full_width, full_height] = target.size();
        #[allow(clippy::cast_sign_loss)]
        let (x, y, width, height) = rect.map_or((0, 0, full_width, full_height), |rect| {
            (
                rect.x().max(0) as u32,
                rect.y().max(0) as u32,
                rect.width().max(0) as u32,
                rect.height().max(0) as u32,
            )
        });
        if width == 0 || height == 0 {
            return Ok(());
        }
        let channels = if width == 0 {
            4
        } else {
            pitch / width as usize
        };
        let pixels = pixels.as_ref();
        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
        for row in 0..height as usize {
            let start = row * pitch;
            let Some(row) = pixels.get(start..start + width as usize * channels) else {
                return Err(anyhow!(
                    "pixel data is too short for {width}x{height} at {channels} channels"
                ));
            };
            match channels {
                4 => rgba.extend(row.chunks_exact(4).flat_map(premultiply)),
                3 => rgba.extend(
                    row.chunks_exact(3)
                        .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], u8::MAX]),
                ),
                _ => return Err(anyhow!("unsupported pitch {pitch} for a width of {width}")),
            }
        }

        self.gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: target.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        Ok(())
    }

    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: f64,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()> {
        assert_ne!(
            Some(texture_id),
            self.texture_target,
            "`texture_id` must not equal the current `texture_target`"
        );
        let (paint_id, size) = {
            let target = self
                .textures
                .get(&texture_id)
                .ok_or_else(|| anyhow!(Error::InvalidTexture(texture_id)))?;
            (target.id(), target.size())
        };
        #[allow(clippy::cast_possible_wrap)]
        let bounds = Rect::new(0, 0, size[0] as i32, size[1] as i32);
        let target_size = self.target()?.size();
        #[allow(clippy::cast_possible_wrap)]
        let full = Rect::new(0, 0, target_size[0] as i32, target_size[1] as i32);
        let shape = shapes::textured(
            paint_id,
            (size[0], size[1]),
            src.unwrap_or(bounds),
            dst.unwrap_or(full),
            angle,
            center,
            flipped,
            tint,
        );
        self.record_blended(shape, false)
    }

    fn texture_target(&self) -> Option<TextureId> {
        self.texture_target
    }

    fn set_texture_target(&mut self, texture_id: TextureId) -> Result<()> {
        if self.textures.contains_key(&texture_id) {
            self.texture_target = Some(texture_id);
            Ok(())
        } else {
            Err(Error::InvalidTexture(texture_id).into())
        }
    }

    fn clear_texture_target(&mut self) {
        self.texture_target = None;
    }

    fn has_texture_target(&self) -> bool {
        self.texture_target.is_some()
    }

    fn clear_texture_cache(&mut self) {
        for (_, cached) in self.images.iter() {
            self.painter.free_texture(cached.id);
        }
        self.images.clear();
    }
}
