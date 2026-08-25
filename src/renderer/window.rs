//! Windows, their surfaces and the frame that puts a canvas on screen.
//!
//! A window is opened in two steps. [`WindowRenderer::create_window`] allocates the canvas and
//! hands back an identifier straight away, and the operating-system window follows in
//! [`Renderer::create_pending_windows`] once the event loop can create one. Drawing between the
//! two records into the canvas as usual.
//!
//! The canvas is an offscreen texture the size of the window, and [`Renderer::paint_frame`] copies
//! it onto the surface. Separating the two keeps the canvas persistent. A surface texture is a
//! fresh, undefined image every frame.

use crate::{
    error::{Error, Result},
    image::{Icon, Image},
    prelude::*,
    renderer::{
        backend::Renderer, gpu::TARGET_FORMAT, target::RenderTarget, RendererSettings,
        WindowRenderer,
    },
};
use anyhow::{anyhow, Context};
use egui::epaint::TextureId as PaintTextureId;
use egui_wgpu::wgpu;
use log::warn;
use std::{fmt::Write, sync::Arc};
use winit::{
    event_loop::ActiveEventLoop,
    window::{CustomCursor, Fullscreen, Window, WindowAttributes},
};

/// A window, its canvas and the surface the canvas is copied onto.
pub(crate) struct WindowState {
    /// Identifier the engine refers to this window by.
    id: WindowId,
    /// Settings the window was built from, updated as the window changes.
    settings: RendererSettings,
    /// Offscreen texture drawing is recorded into.
    canvas: RenderTarget,
    /// The operating-system window, absent until the event loop creates it.
    window: Option<Arc<Window>>,
    /// Surface the canvas is copied onto, absent until the window exists.
    surface: Option<wgpu::Surface<'static>>,
    /// Format the surface is configured in, which need not match [`TARGET_FORMAT`].
    format: wgpu::TextureFormat,
}

impl WindowState {
    /// Returns the canvas drawing is recorded into.
    pub(crate) const fn canvas(&self) -> &RenderTarget {
        &self.canvas
    }

    /// Returns the canvas drawing is recorded into.
    pub(crate) const fn canvas_mut(&mut self) -> &mut RenderTarget {
        &mut self.canvas
    }

    /// Returns the operating-system window, once the event loop has created it.
    fn window(&self) -> Result<&Window> {
        self.window
            .as_deref()
            .ok_or_else(|| anyhow!("window {} has not been created yet", self.id))
    }
}

impl Renderer {
    /// Allocates a canvas for a window that does not exist yet.
    pub(super) fn open_canvas(&mut self, id: WindowId, settings: RendererSettings) {
        let paint_id = self.next_paint_id();
        let canvas = RenderTarget::new(
            &self.gpu.device,
            paint_id,
            [settings.width, settings.height],
            TARGET_FORMAT,
        );
        self.painter.register_texture(
            &self.gpu.device,
            paint_id,
            canvas.view(),
            wgpu::FilterMode::Nearest,
        );
        self.windows.insert(
            id,
            WindowState {
                id,
                settings,
                canvas,
                window: None,
                surface: None,
                format: TARGET_FORMAT,
            },
        );
        self.window_order.push(id);
    }

    /// Creates the operating-system window behind every canvas that is still waiting for one.
    ///
    /// # Errors
    ///
    /// Returns an error if a window or its surface cannot be created.
    pub(crate) fn create_pending_windows(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        for id in self.window_order.clone() {
            let Some(state) = self.windows.get(&id) else {
                continue;
            };
            if state.window.is_some() {
                continue;
            }
            let attributes = attributes(&state.settings);
            let window = Arc::new(
                event_loop
                    .create_window(attributes)
                    .context("failed to create a window")?,
            );
            let surface = self
                .gpu
                .instance
                .create_surface(Arc::clone(&window))
                .context("failed to create a surface")?;
            self.open_clipboard(&window);

            let Some(state) = self.windows.get_mut(&id) else {
                continue;
            };
            state.window = Some(window);
            state.surface = Some(surface);
            self.configure_surface(id)?;
            self.resize_window(id)?;
            // The new window shows the default pointer until the current cursor reaches it, and
            // `apply_cursor` skips a cursor it has already applied elsewhere.
            self.shown_cursor = None;
            self.apply_cursor(event_loop);
        }
        Ok(())
    }

    /// Configures a window's surface for its current size and vsync setting.
    fn configure_surface(&mut self, id: WindowId) -> Result<()> {
        let Some(state) = self.windows.get_mut(&id) else {
            return Ok(());
        };
        let (Some(surface), Some(window)) = (&state.surface, &state.window) else {
            return Ok(());
        };
        let size = window.inner_size();
        let (width, height) = (size.width.max(1), size.height.max(1));
        let capabilities = surface.get_capabilities(&self.gpu.adapter);
        // An sRGB surface encodes what the shader writes, matching what the canvas contains. The
        // fallback to the first format leaves the copy slightly bright.
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| capabilities.formats.first().copied())
            .ok_or_else(|| anyhow!("surface for window {id} supports no format"))?;
        let present_mode = if state.settings.vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        surface.configure(
            &self.gpu.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode,
                desired_maximum_frame_latency: 2,
                alpha_mode: alpha_mode(&capabilities),
                view_formats: vec![],
            },
        );
        state.format = format;
        Ok(())
    }

    /// Resizes a window's canvas to match the window, if the two have drifted apart.
    ///
    /// The canvas is the size of the window, so drawing coordinates are window pixels and the copy
    /// onto the surface is one to one.
    fn resize_window(&mut self, id: WindowId) -> Result<()> {
        let Some(state) = self.windows.get_mut(&id) else {
            return Ok(());
        };
        let Some(window) = &state.window else {
            return Ok(());
        };
        let size = window.inner_size();
        let (width, height) = (size.width.max(1), size.height.max(1));
        if state.canvas.size() == [width, height] {
            return Ok(());
        }
        state.settings.width = width;
        state.settings.height = height;
        if id == self.primary_window_id {
            self.settings.width = width;
            self.settings.height = height;
        }
        state
            .canvas
            .resize(&self.gpu.device, [width, height], TARGET_FORMAT);
        let paint_id = state.canvas.id();
        let view = state.canvas.view();
        self.painter
            .register_texture(&self.gpu.device, paint_id, view, wgpu::FilterMode::Nearest);
        Ok(())
    }

    /// Translates a `winit` event and records it for the next poll.
    pub(crate) fn handle_window_event(
        &mut self,
        winit_id: winit::window::WindowId,
        event: &winit::event::WindowEvent,
    ) {
        let Some(id) = self.window_order.iter().copied().find(|id| {
            self.windows
                .get(id)
                .and_then(|state| state.window.as_ref())
                .is_some_and(|window| window.id() == winit_id)
        }) else {
            return;
        };
        if matches!(event, winit::event::WindowEvent::Resized(_)) {
            if let Err(err) = self
                .configure_surface(id)
                .and_then(|()| self.resize_window(id))
            {
                warn!("failed to resize window {id}: {err}");
            }
        }
        let mut events = Vec::new();
        self.input.translate(id.0, event, &mut events);
        self.events.extend(events);
    }

    /// Applies the cursor the application asked for to every open window.
    ///
    /// An image cursor is built here rather than where it was requested, because only the event
    /// loop can create one. It is built once per path and reused.
    pub(crate) fn apply_cursor(&mut self, event_loop: &ActiveEventLoop) {
        let cursor = self.cursor.clone();
        if self.shown_cursor.as_ref() == Some(&cursor) {
            return;
        }
        self.shown_cursor = Some(cursor.clone());
        let icon = match &cursor {
            None => {
                for state in self.windows.values() {
                    if let Some(window) = &state.window {
                        window.set_cursor_visible(false);
                    }
                }
                return;
            }
            Some(Cursor::System(cursor)) => winit::window::Cursor::Icon(system_cursor(*cursor)),
            #[cfg(not(target_arch = "wasm32"))]
            Some(Cursor::Image(path, hotspot)) => {
                match self.custom_cursor(event_loop, path, *hotspot) {
                    Some(cursor) => winit::window::Cursor::Custom(cursor),
                    None => return,
                }
            }
        };
        for state in self.windows.values() {
            if let Some(window) = &state.window {
                window.set_cursor_visible(true);
                window.set_cursor(icon.clone());
            }
        }
    }

    /// Returns the cursor built from an image, building it the first time the path is seen.
    #[cfg(not(target_arch = "wasm32"))]
    fn custom_cursor(
        &mut self,
        event_loop: &ActiveEventLoop,
        path: &std::path::Path,
        hotspot: (i32, i32),
    ) -> Option<CustomCursor> {
        if let Some(cursor) = self.custom_cursors.get(path) {
            return Some(cursor.clone());
        }
        let image = match Image::from_file(path) {
            Ok(image) => image,
            Err(err) => {
                warn!("failed to read cursor image {}: {err}", path.display());
                return None;
            }
        };
        let pixels = match image.format() {
            PixelFormat::Rgba => image.as_bytes().to_vec(),
            PixelFormat::Rgb => image
                .as_bytes()
                .chunks_exact(3)
                .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], u8::MAX])
                .collect(),
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let source = winit::window::CustomCursor::from_rgba(
            pixels,
            image.width() as u16,
            image.height() as u16,
            hotspot.0 as u16,
            hotspot.1 as u16,
        );
        match source {
            Ok(source) => {
                let cursor = event_loop.create_custom_cursor(source);
                self.custom_cursors
                    .insert(path.to_path_buf(), cursor.clone());
                Some(cursor)
            }
            Err(err) => {
                warn!("invalid cursor image {}: {err}", path.display());
                None
            }
        }
    }

    /// Paints what is recorded into the current target, without touching any other.
    ///
    /// # Errors
    ///
    /// Returns an error if no target is current.
    pub(super) fn paint_current_target(&mut self) -> Result<()> {
        if let Some(delta) = self.text.take_image_delta() {
            self.painter.update_texture(
                &self.gpu.device,
                &self.gpu.queue,
                PaintTextureId::Managed(0),
                &delta,
            );
        }
        let pixels_per_point = self.text.pixels_per_point();
        let atlas = self.text.image_size();

        let (runs, load, size, texture) = {
            let target = self.target_mut()?;
            (
                target.flush(pixels_per_point, atlas),
                target.take_load_op(),
                target.size(),
                target.texture().clone(),
            )
        };
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("pix-engine readback encoder"),
            });
        self.painter.paint(
            &self.gpu.device,
            &self.gpu.queue,
            &mut encoder,
            &view,
            load,
            size,
            &runs,
        );
        self.gpu.queue.submit([encoder.finish()]);
        Ok(())
    }

    /// Paints everything recorded this frame and copies each canvas onto its surface.
    ///
    /// Textures are painted before canvases, so a texture drawn onto a canvas in the same frame
    /// already contains what was recorded into it.
    pub(super) fn paint_frame(&mut self) -> Result<()> {
        if let Some(delta) = self.text.take_image_delta() {
            self.painter.update_texture(
                &self.gpu.device,
                &self.gpu.queue,
                PaintTextureId::Managed(0),
                &delta,
            );
        }
        let pixels_per_point = self.text.pixels_per_point();
        let atlas = self.text.image_size();

        let mut frames = Vec::new();
        let mut stale = Vec::new();
        for id in self.window_order.clone() {
            let Some(state) = self.windows.get(&id) else {
                continue;
            };
            let Some(surface) = &state.surface else {
                continue;
            };
            match surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(texture)
                | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                    let view = texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    frames.push((id, texture, view, state.format));
                }
                wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                    stale.push(id);
                }
                wgpu::CurrentSurfaceTexture::Timeout
                | wgpu::CurrentSurfaceTexture::Occluded
                | wgpu::CurrentSurfaceTexture::Validation => (),
            }
        }
        // A surface that has gone stale is reconfigured and drawn on the next frame.
        for id in stale {
            self.configure_surface(id)?;
        }

        // One submit per target. The painter reuses one set of buffers, and a queued write does
        // not land until the next submit, so two targets sharing a submit would both be drawn with
        // whichever geometry was uploaded last.
        for id in self.texture_order.clone() {
            let Some(target) = self.textures.get_mut(&id) else {
                continue;
            };
            let runs = target.flush(pixels_per_point, atlas);
            // A target with nothing recorded and no clear owed would open a pass that draws
            // nothing, and one holding uploaded pixels would have them painted over.
            if runs.is_empty() && !target.clear_pending() {
                continue;
            }
            let load = target.take_load_op();
            let mut encoder =
                self.gpu
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("pix-engine texture encoder"),
                    });
            self.painter.paint(
                &self.gpu.device,
                &self.gpu.queue,
                &mut encoder,
                target.view(),
                load,
                target.size(),
                &runs,
            );
            self.gpu.queue.submit([encoder.finish()]);
        }

        for id in self.window_order.clone() {
            let Some(state) = self.windows.get_mut(&id) else {
                continue;
            };
            let runs = state.canvas.flush(pixels_per_point, atlas);
            let load = state.canvas.take_load_op();
            let mut encoder =
                self.gpu
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("pix-engine canvas encoder"),
                    });
            self.painter.paint(
                &self.gpu.device,
                &self.gpu.queue,
                &mut encoder,
                state.canvas.view(),
                load,
                state.canvas.size(),
                &runs,
            );
            let canvas = state.canvas.view();
            if let Some((_, _, view, format)) = frames.iter().find(|(frame, ..)| *frame == id) {
                self.blit
                    .draw(&self.gpu.device, &mut encoder, canvas, view, *format);
            }
            self.gpu.queue.submit([encoder.finish()]);
        }
        // Frames are driven from the event loop's idle callback rather than by asking for a
        // redraw, which would leave an event pending and defeat the target frame rate.
        for (_, texture, _, _) in frames {
            texture.present();
        }
        // A texture deleted mid-frame can still be named by a shape recorded before it went, so
        // the painter keeps it until that shape has been drawn.
        for id in self.pending_free.drain(..) {
            self.painter.free_texture(id);
        }
        self.frame += 1;
        self.text.begin_frame();
        Ok(())
    }
}

impl WindowRenderer for Renderer {
    fn window_count(&self) -> usize {
        self.windows.len()
    }

    fn primary_window_id(&self) -> WindowId {
        self.primary_window_id
    }

    fn window_id(&self) -> WindowId {
        self.window_target
    }

    /// Opens a window, returning its identifier before the window itself exists.
    fn create_window(&mut self, s: &mut RendererSettings) -> Result<WindowId> {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        self.open_canvas(id, s.clone());
        Ok(id)
    }

    fn close_window(&mut self, id: WindowId) -> Result<()> {
        let Some(state) = self.windows.remove(&id) else {
            return Err(Error::InvalidWindow(id).into());
        };
        self.painter.free_texture(state.canvas.id());
        self.window_order.retain(|open| *open != id);
        if id == self.window_target {
            self.reset_window_target();
        }
        Ok(())
    }

    /// Records the cursor to show.
    ///
    /// Widgets set a cursor as they are drawn and the frame resets it beforehand, so a frame names
    /// several. Only the last one reaches the window, in [`Renderer::apply_cursor`]. Setting each
    /// in turn makes the pointer flicker between them.
    fn cursor(&mut self, cursor: Option<&Cursor>) -> Result<()> {
        self.cursor = cursor.cloned();
        Ok(())
    }

    fn poll_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    fn title(&self) -> &str {
        &self.settings.title
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        self.settings.title.replace_range(.., title);
        if let Some(window) = self.target_window() {
            window.set_title(title);
        }
        Ok(())
    }

    fn set_fps(&mut self, fps: f32) -> Result<()> {
        self.title.clear();
        write!(self.title, "{} - FPS: {fps:.02}", &self.settings.title).context("invalid title")?;
        let title = self.title.clone();
        if let Some(window) = self.target_window() {
            window.set_title(&title);
        }
        Ok(())
    }

    fn dimensions(&self) -> Result<(u32, u32)> {
        let [width, height] = self.target()?.size();
        Ok((width, height))
    }

    fn window_dimensions(&self) -> Result<(u32, u32)> {
        let [width, height] = self.window_state()?.canvas.size();
        Ok((width, height))
    }

    fn window_position(&self) -> Result<(i32, i32)> {
        let position = self
            .window_state()?
            .window()?
            .outer_position()
            .context("window position is unavailable")?;
        Ok((position.x, position.y))
    }

    fn set_window_dimensions(&mut self, (width, height): (u32, u32)) -> Result<()> {
        let id = self.window_target;
        let Some(state) = self.windows.get_mut(&id) else {
            return Err(Error::InvalidWindow(id).into());
        };
        state.settings.width = width;
        state.settings.height = height;
        if id == self.primary_window_id {
            self.settings.width = width;
            self.settings.height = height;
        }
        if let Some(window) = &state.window {
            let _ignored = window.request_inner_size(winit::dpi::PhysicalSize::new(width, height));
        }
        state
            .canvas
            .resize(&self.gpu.device, [width, height], TARGET_FORMAT);
        let paint_id = state.canvas.id();
        let view = state.canvas.view();
        self.painter
            .register_texture(&self.gpu.device, paint_id, view, wgpu::FilterMode::Nearest);
        self.configure_surface(id)
    }

    /// Returns the region drawing is confined to, or the whole target.
    fn viewport(&self) -> Result<Rect<i32>> {
        let target = self.target()?;
        Ok(target.clip().unwrap_or_else(|| {
            let [width, height] = target.size();
            #[allow(clippy::cast_possible_wrap)]
            Rect::new(0, 0, width as i32, height as i32)
        }))
    }

    /// Confines drawing to a region and moves the origin to its corner.
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> Result<()> {
        let target = self.target_mut()?;
        target.set_clip(rect);
        target.set_offset(rect.map_or_else(|| Point::new([0, 0]), |rect| rect.top_left()));
        Ok(())
    }

    /// Returns the size of the display the current window is on.
    ///
    /// Wayland does not tell a client which output it is on, so the first available monitor stands
    /// in there, and the window's own size stands in when even that is unknown.
    fn display_dimensions(&self) -> Result<(u32, u32)> {
        let window = self.window_state()?.window()?;
        let monitor = window
            .current_monitor()
            .or_else(|| window.available_monitors().next());
        Ok(monitor.map_or_else(
            || {
                let size = window.inner_size();
                (size.width, size.height)
            },
            |monitor| {
                let size = monitor.size();
                (size.width, size.height)
            },
        ))
    }

    fn fullscreen(&self) -> Result<bool> {
        Ok(self.window_state()?.window()?.fullscreen().is_some())
    }

    fn set_fullscreen(&mut self, val: bool) -> Result<()> {
        let id = self.window_target;
        let Some(state) = self.windows.get_mut(&id) else {
            return Err(Error::InvalidWindow(id).into());
        };
        state.settings.fullscreen = val;
        if let Some(window) = &state.window {
            window.set_fullscreen(val.then(|| Fullscreen::Borderless(None)));
        }
        Ok(())
    }

    fn vsync(&self) -> bool {
        self.settings.vsync
    }

    /// Reconfigures the surface to synchronize with the display, or to stop doing so.
    ///
    /// The window is kept, so the returned identifier is the one that was already in use.
    fn set_vsync(&mut self, val: bool) -> Result<WindowId> {
        let id = self.window_target;
        self.settings.vsync = val;
        if let Some(state) = self.windows.get_mut(&id) {
            state.settings.vsync = val;
        }
        self.configure_surface(id)?;
        Ok(id)
    }

    fn set_window_target(&mut self, id: WindowId) -> Result<()> {
        if self.windows.contains_key(&id) {
            self.window_target = id;
            Ok(())
        } else {
            Err(Error::InvalidWindow(id).into())
        }
    }

    fn reset_window_target(&mut self) {
        self.window_target = self.primary_window_id;
    }

    fn show(&mut self) -> Result<()> {
        self.set_visible(true)
    }

    fn hide(&mut self) -> Result<()> {
        self.set_visible(false)
    }
}

impl Renderer {
    /// Returns the window drawing currently goes to.
    fn window_state(&self) -> Result<&WindowState> {
        self.windows
            .get(&self.window_target)
            .ok_or_else(|| anyhow!(Error::InvalidWindow(self.window_target)))
    }

    /// Returns the operating-system window drawing goes to, if it exists yet.
    fn target_window(&self) -> Option<&Window> {
        self.windows
            .get(&self.window_target)
            .and_then(|state| state.window.as_deref())
    }

    /// Shows or hides the window drawing goes to.
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        let id = self.window_target;
        let Some(state) = self.windows.get_mut(&id) else {
            return Err(Error::InvalidWindow(id).into());
        };
        state.settings.hidden = !visible;
        if let Some(window) = &state.window {
            window.set_visible(visible);
        }
        Ok(())
    }
}

/// Builds the `winit` attributes a window is created from.
fn attributes(settings: &RendererSettings) -> WindowAttributes {
    let mut attributes = Window::default_attributes()
        .with_title(&settings.title)
        .with_inner_size(winit::dpi::PhysicalSize::new(
            settings.width,
            settings.height,
        ))
        .with_resizable(settings.resizable)
        .with_decorations(!settings.borderless)
        .with_visible(!settings.hidden);
    if let (Position::Positioned(x), Position::Positioned(y)) = (settings.x, settings.y) {
        attributes = attributes.with_position(winit::dpi::PhysicalPosition::new(x, y));
    }
    if settings.fullscreen {
        attributes = attributes.with_fullscreen(Some(Fullscreen::Borderless(None)));
    }
    if let Some(icon) = settings.icon.as_ref().and_then(icon) {
        attributes = attributes.with_window_icon(Some(icon));
    }
    attributes
}

/// Builds a window icon, reporting and dropping one that cannot be read.
fn icon(icon: &Icon) -> Option<winit::window::Icon> {
    let image = match icon {
        Icon::Image(image) => image.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        Icon::Path(path) => match Image::from_file(path) {
            Ok(image) => image,
            Err(err) => {
                warn!("failed to read icon {}: {err}", path.display());
                return None;
            }
        },
    };
    let pixels = match image.format() {
        PixelFormat::Rgba => image.as_bytes().to_vec(),
        PixelFormat::Rgb => image
            .as_bytes()
            .chunks_exact(3)
            .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], u8::MAX])
            .collect(),
    };
    match winit::window::Icon::from_rgba(pixels, image.width(), image.height()) {
        Ok(icon) => Some(icon),
        Err(err) => {
            warn!("invalid window icon: {err}");
            None
        }
    }
}

/// Returns the alpha mode a surface is configured with.
///
/// The canvas is opaque, so an opaque surface keeps the compositor from reading its alpha channel
/// and showing what is behind the window.
fn alpha_mode(capabilities: &wgpu::SurfaceCapabilities) -> wgpu::CompositeAlphaMode {
    let modes = &capabilities.alpha_modes;
    for mode in [
        wgpu::CompositeAlphaMode::Opaque,
        wgpu::CompositeAlphaMode::Auto,
    ] {
        if modes.contains(&mode) {
            return mode;
        }
    }
    modes
        .first()
        .copied()
        .unwrap_or(wgpu::CompositeAlphaMode::Auto)
}

/// Returns the `winit` cursor a system cursor names.
fn system_cursor(cursor: SystemCursor) -> winit::window::CursorIcon {
    use winit::window::CursorIcon;
    match cursor {
        SystemCursor::Arrow => CursorIcon::Default,
        SystemCursor::IBeam => CursorIcon::Text,
        SystemCursor::Wait => CursorIcon::Wait,
        SystemCursor::Crosshair => CursorIcon::Crosshair,
        SystemCursor::WaitArrow => CursorIcon::Progress,
        SystemCursor::SizeNWSE => CursorIcon::NwseResize,
        SystemCursor::SizeNESW => CursorIcon::NeswResize,
        SystemCursor::SizeWE => CursorIcon::EwResize,
        SystemCursor::SizeNS => CursorIcon::NsResize,
        SystemCursor::SizeAll => CursorIcon::Move,
        SystemCursor::No => CursorIcon::NotAllowed,
        SystemCursor::Hand => CursorIcon::Pointer,
    }
}
