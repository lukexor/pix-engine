use self::window::WindowCanvas;
use crate::{
    gui::theme::{FontId, FontSrc},
    prelude::*,
    renderer::{RendererSettings, Rendering},
    shape::{LineI2, PointI2, QuadI2, TriI2},
};
use anyhow::Context;
use lazy_static::lazy_static;
use log::{debug, warn};
use lru::LruCache;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    controller::GameController,
    gfx::primitives::{DrawRenderer, ToColor},
    image::{InitFlag, Sdl2ImageContext},
    mouse::{Cursor, SystemCursor},
    pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat},
    rect::{Point as SdlPoint, Rect as SdlRect},
    render::{BlendMode as SdlBlendMode, Canvas, TextureQuery},
    rwops::RWops,
    ttf::{Font as SdlFont, FontStyle as SdlFontStyle, Sdl2TtfContext},
    video::Window,
    EventPump, GameControllerSubsystem, Sdl,
};
use std::{cmp, collections::HashMap, fmt};

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
    static ref IMAGE: Sdl2ImageContext =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("sdl2_image initialized");
}

mod audio;
mod event;
mod texture;
mod window;

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    context: Sdl,
    event_pump: EventPump,
    audio_device: AudioQueue<f32>,
    controller_subsys: GameControllerSubsystem,
    controllers: HashMap<ControllerId, GameController>,
    title: String,
    fps: usize,
    settings: RendererSettings,
    cursor: Cursor,
    blend_mode: SdlBlendMode,
    current_font: FontId,
    font_size: u16,
    font_style: SdlFontStyle,
    primary_window_id: WindowId,
    window_target: WindowId,
    texture_target: Option<TextureId>,
    windows: HashMap<WindowId, WindowCanvas>,
    next_texture_id: usize,
    font_data: LruCache<FontId, Font>,
    loaded_fonts: LruCache<(FontId, u16), SdlFont<'static, 'static>>,
}

impl Renderer {
    /// Update the current render target canvas.
    fn update_canvas<F>(&mut self, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut Canvas<Window>) -> PixResult<()>,
    {
        if let Some(texture_id) = self.texture_target {
            let window = self.windows.values_mut().find_map(|w| {
                if w.textures.contains_key(&texture_id) {
                    Some(w)
                } else {
                    None
                }
            });
            if let Some(window) = window {
                // We ensured there's a valid texture above
                let texture = window.textures.get(&texture_id).expect("valid texture");
                let mut result = Ok(());
                window
                    .canvas
                    .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                        result = f(canvas);
                    })
                    .with_context(|| format!("failed to update texture target {}", texture_id))?;
                result
            } else {
                Err(PixError::InvalidTexture(texture_id).into())
            }
        } else {
            f(self.canvas_mut()?)
        }
    }

    /// Load font if family or size has not already been loaded. Returns `true` if a font was
    /// loaded.
    fn load_font(&mut self) -> PixResult<bool> {
        let key = (self.current_font, self.font_size);
        if self.loaded_fonts.contains(&key) {
            return Ok(false);
        }

        let font_data = self
            .font_data
            .get(&self.current_font)
            .expect("valid loaded font");
        let loaded_font = match font_data.source {
            FontSrc::Bytes(bytes) => {
                let rwops = RWops::from_bytes(bytes).map_err(PixError::Renderer)?;
                TTF.load_font_from_rwops(rwops, self.font_size)
                    .map_err(PixError::Renderer)?
            }
            FontSrc::Path(ref path) => TTF
                .load_font(path, self.font_size)
                .map_err(PixError::Renderer)?,
        };
        self.loaded_fonts.put(key, loaded_font);
        Ok(true)
    }

    /// Returns a reference to the current SDL font.
    #[inline]
    fn font(&self) -> &SdlFont<'static, 'static> {
        self.loaded_fonts
            .peek(&(self.current_font, self.font_size))
            .expect("valid font")
    }

    /// Returns a mutable reference the current SDL font.
    #[inline]
    fn font_mut(&mut self) -> &mut SdlFont<'static, 'static> {
        self.loaded_fonts
            .get_mut(&(self.current_font, self.font_size))
            .expect("valid font")
    }

    /// Returns the current window canvas, holding the canvas and texture creators for a window.
    #[inline]
    fn window_canvas(&self) -> PixResult<&WindowCanvas> {
        Ok(self
            .windows
            .get(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?)
    }

    /// Returns the current window canvas, holding the canvas and texture creators for a window.
    #[inline]
    fn window_canvas_mut(&mut self) -> PixResult<&mut WindowCanvas> {
        Ok(self
            .windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?)
    }

    /// Returns the current SDL canvas.
    #[inline]
    fn canvas(&self) -> PixResult<&Canvas<Window>> {
        Ok(&self.window_canvas()?.canvas)
    }

    /// Returns the current SDL canvas.
    #[inline]
    fn canvas_mut(&mut self) -> PixResult<&mut Canvas<Window>> {
        Ok(&mut self.window_canvas_mut()?.canvas)
    }

    /// Returns the current SDL window.
    #[inline]
    fn window(&self) -> PixResult<&Window> {
        Ok(self.window_canvas()?.canvas.window())
    }

    /// Returns the current SDL window.
    #[inline]
    fn window_mut(&mut self) -> PixResult<&mut Window> {
        Ok(self.window_canvas_mut()?.canvas.window_mut())
    }
}

impl Rendering for Renderer {
    /// Initializes the `Sdl2Renderer` using the given settings and opens a new window.
    #[inline]
    fn new(mut s: RendererSettings) -> PixResult<Self> {
        debug!("Initializing SDLRenderer");

        let context = sdl2::init().map_err(PixError::Renderer)?;
        let event_pump = context.event_pump().map_err(PixError::Renderer)?;

        let title = s.title.clone();
        let primary_window = WindowCanvas::new(&context, &mut s)?;
        let cursor = Cursor::from_system(SystemCursor::Arrow).map_err(PixError::Renderer)?;
        cursor.set();
        let window_target = primary_window.id;
        let mut windows = HashMap::new();
        windows.insert(primary_window.id, primary_window);

        // Set up Audio
        let audio_sub = context.audio().map_err(PixError::Renderer)?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(2),
            samples: None,
        };
        let audio_device = audio_sub
            .open_queue(None, &desired_spec)
            .map_err(PixError::Renderer)?;
        audio_device.resume();
        let controller_subsys = context.game_controller().map_err(PixError::Renderer)?;

        let default_font = Font::default();
        let current_font = default_font.id();
        let mut font_data = LruCache::new(s.text_cache_size);
        font_data.put(current_font, default_font);

        let texture_cache_size = s.texture_cache_size;
        let mut renderer = Self {
            context,
            event_pump,
            audio_device,
            controller_subsys,
            controllers: HashMap::new(),
            settings: s,
            title,
            fps: 0,
            cursor,
            blend_mode: SdlBlendMode::None,
            current_font,
            font_size: 14,
            font_style: SdlFontStyle::NORMAL,
            primary_window_id: window_target,
            window_target,
            texture_target: None,
            windows,
            next_texture_id: 0,
            font_data,
            loaded_fonts: LruCache::new(texture_cache_size),
        };
        renderer.load_font()?;

        Ok(renderer)
    }

    /// Clears the canvas to the current clear color.
    #[inline]
    fn clear(&mut self) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            canvas.clear();
            Ok(())
        })
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            canvas.set_draw_color(color);
            Ok(())
        })
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            canvas.set_clip_rect(rect.map(|rect| rect.into()));
            Ok(())
        })
    }

    /// Sets the blend mode used by the renderer to drawing.
    #[inline]
    fn blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode.into();
    }

    /// Updates the canvas from the current back buffer.
    #[inline]
    fn present(&mut self) {
        for window in self.windows.values_mut() {
            window.canvas.present();
        }
    }

    /// Scale the current canvas.
    #[inline]
    fn scale(&mut self, x: f32, y: f32) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            Ok(canvas.set_scale(x, y).map_err(PixError::Renderer)?)
        })
    }

    /// Set the font size for drawing to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> PixResult<()> {
        self.font_size = size as u16;
        self.load_font()?;
        Ok(())
    }

    /// Set the font style for drawing to the current canvas.
    #[inline]
    fn font_style(&mut self, style: FontStyle) {
        let style = style.into();
        if self.font_style != style {
            self.font_style = style;
            self.font_mut().set_style(style);
        }
    }

    /// Set the font family for drawing to the current canvas.
    #[inline]
    fn font_family(&mut self, font: &Font) -> PixResult<()> {
        self.current_font = font.id();
        if !self.font_data.contains(&self.current_font) {
            self.font_data.put(self.current_font, font.clone());
        }
        self.load_font()?;
        Ok(())
    }

    /// Draw text to the current canvas.
    #[inline]
    fn text(
        &mut self,
        pos: PointI2,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<Scalar>,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        fill: Option<Color>,
        outline: u8,
    ) -> PixResult<(u32, u32)> {
        if text.is_empty() {
            return Ok((0, 0));
        }

        if let Some(fill) = fill {
            let window = self
                .windows
                .get_mut(&self.window_target)
                .ok_or(PixError::InvalidWindow(self.window_target))?;
            let texture = WindowCanvas::text_texture_mut(
                &mut window.text_cache,
                &window.texture_creator,
                text,
                wrap_width,
                fill,
                outline,
                self.loaded_fonts
                    .get_mut(&(self.current_font, self.font_size))
                    .expect("valid font"),
                self.current_font,
                self.font_size,
            )?;
            let TextureQuery { width, height, .. } = texture.query();

            let update = |canvas: &mut Canvas<_>| -> PixResult<()> {
                let src = None;
                let dst = Some(SdlRect::new(pos.x(), pos.y(), width, height));
                let result = if angle.is_some() || center.is_some() || flipped.is_some() {
                    let angle = angle.unwrap_or(0.0);
                    let center = center.map(|c| c.into());
                    let horizontal = matches!(flipped, Some(Flipped::Horizontal | Flipped::Both));
                    let vertical = matches!(flipped, Some(Flipped::Vertical | Flipped::Both));
                    canvas.copy_ex(texture, src, dst, angle, center, horizontal, vertical)
                } else {
                    canvas.copy(texture, src, dst)
                };
                Ok(result.map_err(PixError::Renderer)?)
            };

            if let Some(texture_id) = self.texture_target {
                if let Some(texture) = window.textures.get(&texture_id) {
                    let mut result = Ok(());
                    window
                        .canvas
                        .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                            result = update(canvas);
                        })
                        .with_context(|| {
                            format!("failed to update texture target {}", texture_id)
                        })?;
                    result?;
                } else {
                    return Err(PixError::InvalidTexture(texture_id).into());
                }
            } else {
                update(&mut window.canvas)?;
            }

            Ok((width, height))
        } else {
            let font = self.font_mut();
            let surface = wrap_width
                .map_or_else(
                    || font.render(text).blended(Color::BLACK),
                    |width| font.render(text).blended_wrapped(Color::BLACK, width),
                )
                .context("invalid text")?;
            Ok(surface.size())
        }
    }

    /// Get clipboard text from the system clipboard.
    #[inline]
    fn clipboard_text(&self) -> String {
        if let Ok(video) = self.context.video() {
            video.clipboard().clipboard_text().unwrap_or_default()
        } else {
            String::default()
        }
    }

    /// Set clipboard text to the system clipboard.
    #[inline]
    fn set_clipboard_text(&self, value: &str) -> PixResult<()> {
        Ok(self
            .context
            .video()
            .map_err(PixError::Renderer)?
            .clipboard()
            .set_clipboard_text(value)
            .map_err(PixError::Renderer)?)
    }

    /// Open a URL in the default system browser.
    #[inline]
    fn open_url(&self, url: &str) -> PixResult<()> {
        sdl2::url::open_url(url).context("invalid url")
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    #[inline]
    fn size_of(&self, text: &str, wrap_width: Option<u32>) -> PixResult<(u32, u32)> {
        let font = self.font();
        if text.is_empty() {
            return Ok(font.size_of("").unwrap_or_default());
        }
        if let Some(width) = wrap_width {
            Ok(font
                .render(text)
                .blended_wrapped(Color::BLACK, width)
                .context("invalid text")?
                .size())
        } else if text.contains('\n') {
            let mut size = (0, 0);
            for line in text.lines() {
                let (w, h) = font.size_of(line).context("failed to get text size")?;
                size.0 = cmp::max(size.0, w);
                size.1 += h;
            }
            Ok(size)
        } else {
            Ok(font.size_of(text)?)
        }
    }

    /// Draw a pixel to the current canvas.
    #[inline]
    fn point(&mut self, p: PointI2, color: Color) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x, y] = p.map(|v| v as i16);
            Ok(canvas.pixel(x, y, color).map_err(PixError::Renderer)?)
        })
    }

    /// Draw a line to the current canvas.
    #[inline]
    fn line(&mut self, line: LineI2, width: u8, color: Color) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x1, y1] = line.start().map(|v| v as i16);
            let [x2, y2] = line.end().map(|v| v as i16);
            let result = if width == 1 {
                if y1 == y2 {
                    canvas.hline(x1, x2, y1, color)
                } else if x1 == x2 {
                    canvas.vline(x1, y1, y2, color)
                } else {
                    canvas.aa_line(x1, y1, x2, y2, color)
                }
            } else {
                canvas.thick_line(x1, y1, x2, y2, width, color)
            };
            Ok(result.map_err(PixError::Renderer)?)
        })
    }

    /// Draw a triangle to the current canvas.
    #[inline]
    fn triangle(
        &mut self,
        tri: TriI2,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x1, y1] = tri.p1().map(|v| v as i16);
            let [x2, y2] = tri.p2().map(|v| v as i16);
            let [x3, y3] = tri.p3().map(|v| v as i16);
            if let Some(fill) = fill {
                canvas
                    .filled_trigon(x1, y1, x2, y2, x3, y3, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .trigon(x1, y1, x2, y2, x3, y3, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a rectangle to the current canvas.
    #[inline]
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x, y, width, height] = rect.map(|v| v as i16);
            if let Some(radius) = radius {
                let radius = radius as i16;
                if let Some(fill) = fill {
                    canvas
                        .rounded_box(x, y, x + width, y + height, radius, fill)
                        .map_err(PixError::Renderer)?;
                }
                if let Some(stroke) = stroke {
                    canvas
                        .rounded_rectangle(x, y, x + width, y + height, radius, stroke)
                        .map_err(PixError::Renderer)?;
                }
            } else {
                // EXPL: SDL2_gfx renders this 1px bigger than it should.
                if let Some(fill) = fill {
                    canvas
                        .box_(x, y, x + width - 1, y + height - 1, fill)
                        .map_err(PixError::Renderer)?;
                }
                if let Some(stroke) = stroke {
                    canvas
                        .rectangle(x, y, x + width, y + height, stroke)
                        .map_err(PixError::Renderer)?;
                }
            }
            Ok(())
        })
    }

    /// Draw a quadrilateral to the current canvas.
    #[inline]
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x1, y1] = quad.p1().map(|v| v as i16);
            let [x2, y2] = quad.p2().map(|v| v as i16);
            let [x3, y3] = quad.p3().map(|v| v as i16);
            let [x4, y4] = quad.p4().map(|v| v as i16);
            let vx = [x1, x2, x3, x4];
            let vy = [y1, y2, y3, y4];
            if let Some(fill) = fill {
                canvas
                    .filled_polygon(&vx, &vy, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .polygon(&vx, &vy, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a polygon to the current canvas.
    #[inline]
    fn polygon<I>(&mut self, ps: I, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()>
    where
        I: Iterator<Item = PointI2>,
    {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let (vx, vy): (Vec<i16>, Vec<i16>) = ps
                .map(|p| -> (i16, i16) {
                    let [x, y] = p.map(|v| v as i16);
                    (x, y)
                })
                .unzip();
            if let Some(fill) = fill {
                canvas
                    .filled_polygon(&vx, &vy, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .polygon(&vx, &vy, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a ellipse to the current canvas.
    #[inline]
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x, y, width, height] = ellipse.map(|v| v as i16);
            let rw = width / 2;
            let rh = height / 2;
            if let Some(fill) = fill {
                canvas
                    .filled_ellipse(x, y, rw, rh, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .aa_ellipse(x, y, rw, rh, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw an arc to the current canvas.
    #[inline]
    fn arc(
        &mut self,
        p: PointI2,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        self.update_canvas(|canvas: &mut Canvas<_>| -> PixResult<()> {
            let [x, y] = p.map(|v| v as i16);
            let radius = radius as i16;
            let start = start as i16;
            let end = end as i16;
            match mode {
                ArcMode::Default => {
                    if let Some(stroke) = stroke {
                        canvas
                            .arc(x, y, radius, start, end, stroke)
                            .map_err(PixError::Renderer)?;
                    }
                }
                ArcMode::Pie => {
                    if let Some(fill) = fill {
                        canvas
                            .filled_pie(x, y, radius, start, end, fill)
                            .map_err(PixError::Renderer)?;
                    }
                    if let Some(stroke) = stroke {
                        canvas
                            .pie(x, y, radius, start, end, stroke)
                            .map_err(PixError::Renderer)?;
                    }
                }
            }
            Ok(())
        })
    }

    /// Draw an image to the current canvas, optionally rotated about a `center`, flipped or tinted
    #[inline]
    fn image(
        &mut self,
        img: &Image,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()> {
        let window = self
            .windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?;
        let texture =
            WindowCanvas::image_texture_mut(&mut window.image_cache, &window.texture_creator, img)?;
        let [r, g, b, a] = tint.map_or([255; 4], |t| t.channels());
        texture.set_color_mod(r, g, b);
        texture.set_alpha_mod(a);
        texture.set_blend_mode(self.blend_mode);
        texture
            .update(
                None,
                img.as_bytes(),
                img.format().channels() * img.width() as usize,
            )
            .context("failed to update image texture")?;

        let update = |canvas: &mut Canvas<_>| -> PixResult<()> {
            let src = src.map(|r| r.into());
            let dst = dst.map(|r| r.into());
            let result = if angle > 0.0 || center.is_some() || flipped.is_some() {
                let center = center.map(|c| c.into());
                let horizontal = matches!(flipped, Some(Flipped::Horizontal | Flipped::Both));
                let vertical = matches!(flipped, Some(Flipped::Vertical | Flipped::Both));
                canvas.copy_ex(texture, src, dst, angle, center, horizontal, vertical)
            } else {
                canvas.copy(texture, src, dst)
            };
            Ok(result.map_err(PixError::Renderer)?)
        };

        if let Some(texture_id) = self.texture_target {
            if let Some(texture) = window.textures.get(&texture_id) {
                let mut result = Ok(());
                window
                    .canvas
                    .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                        result = update(canvas);
                    })
                    .with_context(|| format!("failed to update texture target {}", texture_id))?;
                result?;
            } else {
                return Err(PixError::InvalidTexture(texture_id).into());
            }
        } else {
            update(&mut window.canvas)?;
        }

        Ok(())
    }

    /// Return the current rendered target pixels as an array of bytes.
    #[inline]
    fn to_bytes(&mut self) -> PixResult<Vec<u8>> {
        if let Some(texture_id) = self.texture_target {
            let window = self.windows.values_mut().find_map(|w| {
                if w.textures.contains_key(&texture_id) {
                    Some(w)
                } else {
                    None
                }
            });
            if let Some(window) = window {
                // We ensured there's a valid texture above
                let texture = window.textures.get(&texture_id).expect("valid texture");
                let mut result = Ok(vec![]);
                window
                    .canvas
                    .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                        result = canvas.read_pixels(None, SdlPixelFormat::RGBA32);
                    })
                    .with_context(|| format!("failed to read texture target {}", texture_id))?;
                Ok(result.map_err(PixError::Renderer)?)
            } else {
                Err(PixError::InvalidTexture(texture_id).into())
            }
        } else {
            Ok(self
                .canvas()?
                .read_pixels(None, SdlPixelFormat::RGBA32)
                .map_err(PixError::Renderer)?)
        }
    }

    /// Connect a controller with the given joystick index to start receiving events.
    fn open_controller(&mut self, controller_id: ControllerId) -> PixResult<()> {
        let joystick_index = *controller_id;
        if self.controller_subsys.is_game_controller(joystick_index) {
            self.controllers
                .insert(controller_id, self.controller_subsys.open(joystick_index)?);
        } else {
            warn!("Joystick {} is not a game controller. Generic joysticks are currently unsupported.", joystick_index);
        }
        Ok(())
    }

    /// Disconnect a controller with the given joystick index to stop receiving events.
    fn close_controller(&mut self, controller_id: ControllerId) {
        self.controllers.remove(&controller_id);
    }
}

impl fmt::Debug for Renderer {
    #[doc(hidden)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renderer")
            .field(
                "audio_device",
                &format_args!(
                    "{{ spec: {:?}, status: {:?}, queue: {:?} }}",
                    self.audio_device.spec(),
                    self.audio_device.status(),
                    self.audio_device.size()
                ),
            )
            .field("title", &self.title)
            .field("fps", &self.fps)
            .field("settings", &self.settings)
            .field("blend_mode", &self.blend_mode)
            .field(
                "current_font",
                &self.font_data.peek(&self.current_font).map(|f| &f.name),
            )
            .field("font_size", &self.font_size)
            .field("font_style", &self.font_style)
            .field("window_target", &self.texture_target)
            .field("texture_target", &self.texture_target)
            .field("windows", &self.windows)
            .field("next_texture_id", &self.next_texture_id)
            .field("font_data", &self.font_data)
            .field("loaded_fonts", &self.loaded_fonts)
            .finish_non_exhaustive()
    }
}

/*
 * Type Conversions
 */

impl ToColor for Color {
    /// Convert [Color] to tuple of `(r, g, b, a)`.
    #[doc(hidden)]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        let [r, g, b, a] = self.channels();
        (r, g, b, a)
    }
}

impl From<Color> for SdlColor {
    /// Convert [Color] to [`SdlColor`].
    #[doc(hidden)]
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.channels();
        Self::RGBA(r, g, b, a)
    }
}

impl From<FontStyle> for SdlFontStyle {
    /// Convert [FontStyle] to [`SdlFontStyle`].
    #[doc(hidden)]
    fn from(style: FontStyle) -> Self {
        Self::from_bits(style.bits()).expect("valid FontStyle")
    }
}

impl From<Rect<i32>> for SdlRect {
    /// Convert [`Rect<i32>`] to [`SdlRect`].
    #[doc(hidden)]
    fn from(rect: Rect<i32>) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as u32,
            rect.height() as u32,
        )
    }
}

impl From<SdlRect> for Rect<i32> {
    /// Convert [`Rect<i32>`] to [`SdlRect`].
    #[doc(hidden)]
    fn from(rect: SdlRect) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as i32,
            rect.height() as i32,
        )
    }
}

impl From<&Rect<i32>> for SdlRect {
    /// Convert &[`Rect<i32>`] to [`SdlRect`].
    #[doc(hidden)]
    fn from(rect: &Rect<i32>) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as u32,
            rect.height() as u32,
        )
    }
}

impl From<PointI2> for SdlPoint {
    /// Convert [`PointI2`] to [`SdlPoint`].
    #[doc(hidden)]
    fn from(p: PointI2) -> Self {
        Self::new(p.x(), p.y())
    }
}

impl From<&PointI2> for SdlPoint {
    /// Convert &[`PointI2`] to [`SdlPoint`].
    #[doc(hidden)]
    fn from(p: &PointI2) -> Self {
        Self::new(p.x(), p.y())
    }
}

impl From<BlendMode> for SdlBlendMode {
    /// Convert [`BlendMode`] to [`SdlBlendMode`].
    #[doc(hidden)]
    fn from(mode: BlendMode) -> Self {
        match mode {
            BlendMode::None => Self::None,
            BlendMode::Blend => Self::Blend,
            BlendMode::Add => Self::Add,
            BlendMode::Mod => Self::Mod,
        }
    }
}

impl From<PixelFormat> for SdlPixelFormat {
    /// Convert [`PixelFormat`] to [`SdlPixelFormat`].
    #[doc(hidden)]
    fn from(format: PixelFormat) -> Self {
        match format {
            PixelFormat::Rgb => Self::RGB24,
            PixelFormat::Rgba => Self::RGBA32,
        }
    }
}
