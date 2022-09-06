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
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
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
    /// Return the dimensions of given text for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the current font, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     let text = "Some text";
    ///     let (w, h) = s.size_of(text)?;
    ///     // Draw a box behind the text
    ///     s.rect(rect![s.cursor_pos() - 10, w as i32 + 20, h as i32 + 20]);
    ///     s.text(text)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn size_of<S: AsRef<str>>(&self, text: S) -> Result<(u32, u32)> {
        self.renderer
            .size_of(text.as_ref(), self.settings.wrap_width)
    }

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
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text("Text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text<S>(&mut self, text: S) -> Result<(u32, u32)>
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
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.heading("Heading")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn heading<S>(&mut self, text: S) -> Result<(u32, u32)>
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
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.monospace("Monospace")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn monospace<S>(&mut self, text: S) -> Result<(u32, u32)>
    where
        S: AsRef<str>,
    {
        let s = self;
        s.push();
        s.renderer.font_family(&s.theme.fonts.monospace)?;
        s.renderer.font_size(s.theme.font_size + 2)?;
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
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
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
    ) -> Result<(u32, u32)>
    where
        S: AsRef<str>,
        A: Into<Option<f64>>,
        C: Into<Option<Point<i32>>>,
        F: Into<Option<Flipped>>,
    {
        let text = text.as_ref();
        let angle = angle.into();
        let center = center.into();
        let flipped = flipped.into();

        let s = &self.settings;
        let fill = s.fill.unwrap_or(Color::TRANSPARENT);

        let rect = {
            let stroke_size = match (s.stroke, s.stroke_weight) {
                (Some(stroke), weight) if weight > 0 => {
                    Some(self.render_text(text, stroke, weight, angle, center, flipped)?)
                }
                _ => None,
            };
            let text_size = self.render_text(text, fill, 0, angle, center, flipped)?;
            stroke_size.unwrap_or(text_size)
        };
        // EXPL: Add some bottom/right padding
        let rect = rect.offset_size([3, 3]);
        self.advance_cursor(rect.size());

        Ok((rect.width() as u32, rect.height() as u32))
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
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.bullet("Bulleted text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn bullet<S>(&mut self, text: S) -> Result<(u32, u32)>
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
        s.circle([pos.x() + ipad.x(), pos.y() + font_size / 2, r])?;
        s.pop();

        s.set_cursor_pos([pos.x() + ipad.x() + 2 * r + 2 * ipad.x(), pos.y()]);
        let (w, h) = s.text_transformed(text, 0.0, None, None)?;

        Ok((w + r as u32, h))
    }

    /// Draw a text menu to the current canvas which returns true when clicked.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn menu<S>(&mut self, text: S) -> Result<bool>
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
            s.stroke(None);
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(bg);
        } else {
            s.fill(None);
        }
        s.rect(hover)?;

        // Text
        s.stroke(None);
        s.fill(fg);
        s.set_cursor_pos([hover.x() + fpad.x(), hover.y() + fpad.y()]);
        s.text_transformed(text, 0.0, None, None)?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_focus(id);
        s.advance_cursor(hover.size());
        Ok(!s.ui.disabled && s.ui.was_clicked(id))
    }

    /// Draw a collapsing text tree to the current canvas which returns true when the bullet is not
    /// collapsed.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn collapsing_tree<S, F>(&mut self, text: S, f: F) -> Result<bool>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> Result<()>,
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
            s.stroke(None);
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(bg);
        } else {
            s.fill(None);
        }
        s.rect(hover)?;

        // Arrow
        s.stroke(None);
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
        s.ui.handle_focus(id);

        s.advance_cursor([hover.width(), ipad.y() / 2]);

        if expanded {
            let (indent_width, _) = s.text_size("    ")?;
            s.ui.set_column_offset(indent_width);
            f(s)?;
            s.ui.reset_column_offset();
        }

        Ok(expanded)
    }

    /// Draw a collapsing header to the current canvas which returns true when the tree is not
    /// collapsed.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn collapsing_header<S, F>(&mut self, text: S, f: F) -> Result<bool>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> Result<()>,
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
            s.stroke(None);
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        s.fill(bg);
        s.rect(hover)?;

        // Arrow
        s.stroke(None);
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
        s.ui.handle_focus(id);

        s.advance_cursor([hover.width(), ipad.y() / 2]);

        if expanded {
            f(s)?;
        }

        Ok(expanded)
    }
}

impl PixState {
    #[inline]
    fn render_text(
        &mut self,
        text: &str,
        color: Color,
        outline: u16,
        angle: Option<f64>,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
    ) -> Result<Rect<i32>> {
        let s = &self.settings;
        let wrap_width = s.wrap_width;
        let angle_mode = s.angle_mode;
        let colors = self.theme.colors;
        let ipad = self.theme.spacing.item_pad;

        let mut pos = self.cursor_pos();
        if s.rect_mode == RectMode::Center {
            let (width, height) = self.size_of(text)?;
            pos.offset([-(clamp_size(width) / 2), -(clamp_size(height) / 2)]);
        };
        if outline == 0 && s.stroke_weight > 0 {
            pos += i32::from(s.stroke_weight);
        }

        self.push();

        let color = if self.ui.disabled {
            color.blended(colors.background, 0.38)
        } else {
            color
        };
        let wrap_width = if wrap_width.is_none() && text.contains('\n') {
            text.lines()
                .map(|line| {
                    let (line_width, _) = self.renderer.size_of(line, None).unwrap_or_default();
                    line_width
                })
                .max()
                .map(|width| width + (pos.x() + ipad.x()) as u32)
        } else {
            wrap_width
        };
        let rect = if matches!(angle, Some(angle) if angle != 0.0) {
            let angle = if angle_mode == AngleMode::Radians {
                angle.map(f64::to_degrees)
            } else {
                angle
            };
            let (width, height) = self.renderer.size_of(text, wrap_width)?;
            let rect = rect![0, 0, clamp_size(width), clamp_size(height)];
            let rect = angle.map_or(rect, |angle| rect.rotated(angle.to_radians(), center));
            self.renderer.text(
                (pos - rect.top_left()).into(),
                text,
                wrap_width,
                angle,
                center,
                flipped,
                Some(color),
                outline,
            )?;
            rect![pos, rect.width() + rect.left(), rect.height() + rect.top()]
        } else {
            let (width, height) = self.renderer.text(
                pos,
                text,
                wrap_width,
                None,
                center,
                flipped,
                Some(color),
                outline,
            )?;
            rect![pos, clamp_size(width), clamp_size(height)]
        };

        self.pop();
        Ok(rect)
    }
}
