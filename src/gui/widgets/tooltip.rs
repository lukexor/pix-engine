//! Tooltip UI widgets.

use crate::{gui::state::Texture, prelude::*};

impl PixState {
    /// Draw help marker text that, when hovered, displays a help box with text to the current
    /// canvas.
    pub fn help_marker<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let pos = s.cursor_pos();

        // Calculate hover area
        let marker = "(?)";
        let (w, h) = s.size_of(marker)?;
        let hover = rect!(pos, w as i32, h as i32);

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, hover);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        if focused {
            s.push();
            s.stroke(s.highlight_color());
            s.no_fill();
            s.rect(hover)?;
            s.pop();
        }

        // Marker
        s.disable();
        s.text(marker)?;
        if !disabled {
            s.no_disable();
        }

        // Tooltip
        if hovered || focused {
            s.tooltip(text)?;
        }

        s.pop();

        // Process input
        s.ui.handle_input(id);

        Ok(())
    }

    /// Draw tooltip box at the mouse cursor with text to the current canvas.
    pub fn tooltip<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let style = s.theme.style;
        let pad = style.frame_pad;

        let (w, h) = s.size_of(text)?;
        let w = w + 2 * pad.x() as u32;
        let h = h + 2 * pad.y() as u32;

        // Render
        s.ui.push_cursor();
        s.advanced_tooltip(w, h, |s: &mut PixState| {
            s.background(s.primary_color())?;

            s.rect_mode(RectMode::Corner);
            s.push();
            s.stroke(s.muted_color());
            s.no_fill();
            s.rect([0, 0, w - 1, h - 1])?;
            s.pop();

            s.text(text)?;

            Ok(())
        })?;
        s.ui.pop_cursor();

        Ok(())
    }

    /// Draw an advanced tooltip box at the mouse cursor to the current canvas.
    pub fn advanced_tooltip<F>(&mut self, width: u32, height: u32, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let s = self;
        s.ui.push_id(1);
        let id = s.ui.get_id(&[width, height]);
        s.ui.pop_id();

        // Calculate rect
        let mpos = s.mouse_pos();
        let mut rect = rect![mpos.x() + 15, mpos.y() + 15, width as i32, height as i32];

        // Ensure rect stays inside window
        let (win_width, win_height) = s.window_dimensions()?;
        if rect.right() > win_width as i32 {
            rect.set_right(mpos.x() - 10);
        }
        if rect.bottom() > win_height as i32 {
            rect.set_bottom(mpos.y() - 5);
        }

        if !s.ui.textures.contains_key(&id) {
            let texture_id = s.create_texture(width, height, PixelFormat::Rgba)?;
            s.ui.textures
                .insert(id, Texture::new(texture_id, None, Some(rect)));
        }
        let texture_id = {
            // SAFETY: We just checked or inserted a texture.
            let texture = s.ui.textures.get_mut(&id).expect("valid texture target");
            texture.visible = true;
            texture.dst = Some(rect);
            texture.id
        };

        s.ui.set_mouse_offset(rect.top_left());
        s.with_texture(texture_id, |s: &mut PixState| {
            s.set_cursor_pos(s.theme.style.frame_pad);
            f(s)
        })?;
        s.ui.clear_mouse_offset();

        Ok(())
    }
}
