//! Text widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::text`]
//! - [`PixState::text_transformed`]
//! - [`PixState::bullet`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { text_field: String, text_area: String};
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.text("Text")?;
//!     s.angle_mode(AngleMode::Degrees);
//!     let angle = 10.0;
//!     let center = point!(10, 10);
//!     s.text_transformed("Text", angle, center, Flipped::Horizontal)?;
//!     s.bullet("Bulleted text")?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{ops::clamp_size, prelude::*, renderer::Rendering};

impl PixState {
    /// Draw body text to the current canvas.
    ///
    /// Returns the rendered `(width, height)` of the text, including any newlines or text
    /// wrapping.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text<S>(&mut self, text: S) -> PixResult<(u32, u32)>
    where
        S: AsRef<str>,
    {
        self.text_transformed(text, 0.0, None, None)
    }

    /// Draw heading text to the current canvas.
    ///
    /// Returns the rendered `(width, height)` of the text, including any newlines or text
    /// wrapping.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.heading("Heading")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn heading<S>(&mut self, text: S) -> PixResult<(u32, u32)>
    where
        S: AsRef<str>,
    {
        let s = self;
        s.push();
        s.renderer.font_family(&s.theme.fonts.heading)?;
        s.renderer.font_size(s.theme.sizes.heading)?;
        s.renderer.font_style(s.theme.styles.heading);
        let size = s.text_transformed(text, 0.0, None, None);
        s.pop();
        size
    }

    /// Draw monospace text to the current canvas.
    ///
    /// Returns the rendered `(width, height)` of the text, including any newlines or text
    /// wrapping.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.monospace("Monospace")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn monospace<S>(&mut self, text: S) -> PixResult<(u32, u32)>
    where
        S: AsRef<str>,
    {
        let s = self;
        s.push();
        s.renderer.font_family(&s.theme.fonts.monospace)?;
        s.renderer.font_size(s.theme.sizes.monospace)?;
        s.renderer.font_style(s.theme.styles.monospace);
        let size = s.text_transformed(text, 0.0, None, None);
        s.pop();
        size
    }

    /// Draw transformed text to the current canvas, optionally rotated about a `center` by `angle`
    /// or `flipped`. `angle` can be in radians or degrees depending on [`AngleMode`].
    ///
    /// Returns the rendered `(width, height)` of the text, including any newlines or text
    /// wrapping.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.angle_mode(AngleMode::Degrees);
    ///     let angle = 10.0;
    ///     let center = point!(10, 10);
    ///     s.text_transformed("Transformed text", angle, center, Flipped::Horizontal)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_transformed<S, A, C, F>(
        &mut self,
        text: S,
        angle: A,
        center: C,
        flipped: F,
    ) -> PixResult<(u32, u32)>
    where
        S: AsRef<str>,
        A: Into<Option<Scalar>>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let text = text.as_ref();
        let mut angle = angle.into();
        let center = center.into();
        let flipped = flipped.into();

        let s = self;
        let fill = if let Some(fill) = s.settings.fill {
            fill
        } else {
            return Ok((0, 0));
        };
        let stroke = s.settings.stroke;
        let stroke_weight = s.settings.stroke_weight;
        let wrap_width = s.settings.wrap_width;
        let colors = s.theme.colors;

        let disabled = s.ui.disabled;
        let mut pos = s.cursor_pos();
        if let RectMode::Center = s.settings.rect_mode {
            let (width, height) = s.size_of(text)?;
            pos.offset([-(clamp_size(width) / 2), -(clamp_size(height) / 2)]);
        };
        let mut angle_radians = angle;
        if let AngleMode::Radians = s.settings.angle_mode {
            angle = angle.map(|a| a.to_degrees());
        } else {
            angle_radians = angle.map(|a| a.to_radians());
        }

        let mut render_text = |mut color: Color, outline: u8| -> PixResult<(u32, u32)> {
            s.push();

            // Make sure to offset the text if an outline was drawn
            if stroke.is_some() && stroke_weight > 0 && outline == 0 {
                pos += i32::from(stroke_weight);
            }
            if disabled {
                color = color.blended(colors.background, 0.38);
            }

            let (w, h) = if wrap_width.is_some() {
                s.renderer.text(
                    pos,
                    text,
                    wrap_width,
                    angle,
                    center,
                    flipped,
                    Some(color),
                    outline,
                )?
            } else {
                let mut x = pos.x();
                let mut y = pos.y();
                let (mut total_width, mut total_height) = (0, 0);
                for line in text.split('\n') {
                    let (line_width, line_height) = s.renderer.size_of(text, wrap_width)?;
                    let rect = rect![0, 0, clamp_size(line_width), clamp_size(line_height)];
                    let bounding_box =
                        angle_radians.map_or(rect, |angle| rect.rotated(angle, center));
                    x -= bounding_box.x();
                    y -= bounding_box.y();
                    s.renderer.text(
                        point![x, y],
                        line,
                        wrap_width,
                        angle,
                        center,
                        flipped,
                        Some(color),
                        outline,
                    )?;
                    total_width += bounding_box.width() as u32;
                    total_height += bounding_box.height() as u32;
                    y += bounding_box.height();
                }
                (total_width, total_height)
            };
            let rect = rect![pos, clamp_size(w), clamp_size(h)];

            // Only advance the cursor if we're not drawing a text outline
            if outline == 0 {
                s.advance_cursor(rect);
            }

            s.pop();
            Ok((w, h))
        };

        let stroke_size = match stroke {
            Some(stroke) if stroke_weight > 0 => Some(render_text(stroke, stroke_weight)?),
            _ => None,
        };
        let size = render_text(fill, 0)?;

        Ok(stroke_size.unwrap_or(size))
    }

    /// Draw bulleted text to the current canvas.
    ///
    /// Returns the rendered `(width, height)` of the text, including any newlines or text
    /// wrapping.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.bullet("Bulleted text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn bullet<S>(&mut self, text: S) -> PixResult<(u32, u32)>
    where
        S: AsRef<str>,
    {
        let s = self;
        let fpad = s.theme.spacing.frame_pad;
        let font_size = clamp_size(s.theme.sizes.body);
        let pos = s.cursor_pos();

        let r = font_size / 6;

        s.push();
        s.ellipse_mode(EllipseMode::Corner);
        s.circle([pos.x(), pos.y() + font_size / 2, r])?;
        s.pop();

        s.set_cursor_pos([pos.x() + 2 * r + 2 * fpad.x(), pos.y()]);
        let (w, h) = s.text_transformed(text, 0.0, None, None)?;

        Ok((w + r as u32, h))
    }
}
