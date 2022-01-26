//! Text widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::text`]
//! - [`PixState::text_transformed`]
//! - [`PixState::bullet`]
//! - [`PixState::collapsing_tree`]
//! - [`PixState::collapsing_header`]
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

use crate::{gui::Direction, ops::clamp_size, prelude::*, renderer::Rendering};

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
        self.text_transformed(text, None, None, None)
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
        s.renderer.font_size(s.theme.font_size + 6)?;
        s.renderer.font_style(s.theme.styles.heading);
        let size = s.text_transformed(text, None, None, None);
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
        s.renderer.font_size(s.theme.font_size)?;
        s.renderer.font_style(s.theme.styles.monospace);
        let size = s.text_transformed(text, None, None, None);
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
        if s.settings.rect_mode == RectMode::Center {
            let (width, height) = s.size_of(text)?;
            pos.offset([-(clamp_size(width) / 2), -(clamp_size(height) / 2)]);
        };
        let mut angle_radians = angle;
        if s.settings.angle_mode == AngleMode::Radians {
            angle = angle.map(Scalar::to_degrees);
        } else {
            angle_radians = angle.map(Scalar::to_radians);
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
                    let (line_width, line_height) = s.renderer.size_of(line, wrap_width)?;
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
                s.advance_cursor(rect.size());
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
        let ipad = s.theme.spacing.item_pad;
        let font_size = clamp_size(s.theme.font_size);
        let pos = s.cursor_pos();

        let r = font_size / 5;

        s.push();
        s.ellipse_mode(EllipseMode::Corner);
        s.circle([pos.x(), pos.y() + font_size / 2, r])?;
        s.pop();

        s.set_cursor_pos([pos.x() + 2 * r + 2 * ipad.x(), pos.y()]);
        let (w, h) = s.text_transformed(text, 0.0, None, None)?;

        Ok((w + r as u32, h))
    }

    /// Draw a text menu to the current canvas which returns true when clicked.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn menu<S>(&mut self, text: S) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let pos = s.cursor_pos();
        let fpad = s.theme.spacing.frame_pad;

        // Calculate hover size
        let (width, height) = s.text_size(text)?;
        let width = s.ui.next_width.take().unwrap_or(width + 2 * fpad.x());

        let hover = rect![pos, width, height + 2 * fpad.y()];
        let hovered = s.ui.try_hover(id, &hover);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Hover/Focused Rect
        let [stroke, bg, fg] = if hovered {
            s.widget_colors(id, ColorType::Secondary)
        } else {
            s.widget_colors(id, ColorType::Background)
        };

        if active || focused {
            s.stroke(stroke);
        } else {
            s.no_stroke();
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(bg);
        } else {
            s.no_fill();
        }
        s.rect(hover)?;

        // Text
        s.no_stroke();
        s.fill(fg);
        s.set_cursor_pos([hover.x() + fpad.x(), hover.y() + fpad.y()]);
        s.text_transformed(text, 0.0, None, None)?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_events(id);
        s.advance_cursor(hover.size());
        Ok(!s.ui.disabled && s.ui.was_clicked(id))
    }

    /// Draw a collapsing text tree to the current canvas which returns true when the bullet is not
    /// collapsed.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn collapsing_tree<S, F>(&mut self, text: S, f: F) -> PixResult<bool>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let font_size = clamp_size(s.theme.font_size);
        let pos = s.cursor_pos();
        let fpad = s.theme.spacing.frame_pad;
        let ipad = s.theme.spacing.item_pad;
        let expanded = s.ui.expanded(id);
        let arrow_width = font_size / 2;

        // Calculate hover size
        let (width, height) = s.text_size(text)?;
        let column_offset = s.ui.column_offset();
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.ui_width().unwrap_or(width));

        let hover = rect![pos, width - column_offset, height + 2 * fpad.y()];
        let hovered = s.ui.try_hover(id, &hover);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        s.push();

        // Hover/Focused Rect
        let [stroke, bg, fg] = if hovered {
            s.widget_colors(id, ColorType::Secondary)
        } else {
            s.widget_colors(id, ColorType::Background)
        };

        if active || focused {
            s.stroke(stroke);
        } else {
            s.no_stroke();
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(bg);
        } else {
            s.no_fill();
        }
        s.rect(hover)?;

        // Arrow
        s.no_stroke();
        s.fill(fg);
        if expanded {
            s.arrow(hover.top_left() + fpad, Direction::Down, 1.0)?;
        } else {
            s.arrow(hover.top_left() + fpad, Direction::Right, 1.0)?;
        }

        // Text
        let bullet_offset = arrow_width + 3 * ipad.x();
        s.set_cursor_pos([hover.x() + bullet_offset, hover.y() + fpad.y()]);
        s.text_transformed(text, 0.0, None, None)?;

        s.pop();

        // Process input
        if (hovered && s.ui.was_clicked(id)) || (focused && s.ui.key_entered() == Some(Key::Return))
        {
            s.ui.set_expanded(id, !expanded);
        }
        s.ui.handle_events(id);

        s.advance_cursor([hover.width(), ipad.y() / 2]);

        if expanded {
            let (indent_width, _) = s.text_size("    ")?;
            s.ui.inc_column_offset(indent_width);
            f(s)?;
            s.ui.dec_column_offset();
        }

        Ok(expanded)
    }

    /// Draw a collapsing header to the current canvas which returns true when the tree is not
    /// collapsed.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn collapsing_header<S, F>(&mut self, text: S, f: F) -> PixResult<bool>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let font_size = clamp_size(s.theme.font_size);
        let pos = s.cursor_pos();
        let fpad = s.theme.spacing.frame_pad;
        let ipad = s.theme.spacing.item_pad;
        let expanded = s.ui.expanded(id);
        let arrow_width = font_size / 2;

        // Calculate hover size
        let (width, height) = s.text_size(text)?;
        let column_offset = s.ui.column_offset();
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.ui_width().unwrap_or(width));

        let hover = rect![pos, width - column_offset, height + 2 * fpad.y()];
        let hovered = s.ui.try_hover(id, &hover);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        s.push();

        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Secondary);
        if active || focused {
            s.stroke(stroke);
        } else {
            s.no_stroke();
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        s.fill(bg);
        s.rect(hover)?;

        // Arrow
        s.no_stroke();
        s.fill(fg);
        if expanded {
            s.arrow(hover.top_left() + fpad, Direction::Down, 1.0)?;
        } else {
            s.arrow(hover.top_left() + fpad, Direction::Right, 1.0)?;
        }

        // Text
        let bullet_offset = arrow_width + 3 * ipad.x();
        s.set_cursor_pos([hover.x() + bullet_offset, hover.y() + fpad.y()]);
        s.text_transformed(text, 0.0, None, None)?;

        s.pop();

        // Process input
        if (hovered && s.ui.was_clicked(id)) || (focused && s.ui.key_entered() == Some(Key::Return))
        {
            s.ui.set_expanded(id, !expanded);
        }
        s.ui.handle_events(id);

        s.advance_cursor([hover.width(), ipad.y() / 2]);

        if expanded {
            let (indent_width, _) = s.text_size("    ")?;
            s.ui.inc_column_offset(indent_width);
            f(s)?;
            s.ui.dec_column_offset();
        }

        Ok(expanded)
    }
}
