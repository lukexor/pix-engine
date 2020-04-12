use crate::{
    image::{Image, ImageRef},
    pixel::{ColorType, Pixel},
    renderer::Renderer,
    state::{AlphaMode, StateData},
    PixEngineResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl StateData {
    // Thanks to https://github.com/OneLoneCoder/olcPixelGameEngine for this!
    pub fn construct_font() -> Image {
        let mut data = String::new();
        data.push_str("?Q`0001oOch0o01o@F40o0<AGD4090LAGD<090@A7ch0?00O7Q`0600>00000000");
        data.push_str("O000000nOT0063Qo4d8>?7a14Gno94AA4gno94AaOT0>o3`oO400o7QN00000400");
        data.push_str("Of80001oOg<7O7moBGT7O7lABET024@aBEd714AiOdl717a_=TH013Q>00000000");
        data.push_str("720D000V?V5oB3Q_HdUoE7a9@DdDE4A9@DmoE4A;Hg]oM4Aj8S4D84@`00000000");
        data.push_str("OaPT1000Oa`^13P1@AI[?g`1@A=[OdAoHgljA4Ao?WlBA7l1710007l100000000");
        data.push_str("ObM6000oOfMV?3QoBDD`O7a0BDDH@5A0BDD<@5A0BGeVO5ao@CQR?5Po00000000");
        data.push_str("Oc``000?Ogij70PO2D]??0Ph2DUM@7i`2DTg@7lh2GUj?0TO0C1870T?00000000");
        data.push_str("70<4001o?P<7?1QoHg43O;`h@GT0@:@LB@d0>:@hN@L0@?aoN@<0O7ao0000?000");
        data.push_str("OcH0001SOglLA7mg24TnK7ln24US>0PL24U140PnOgl0>7QgOcH0K71S0000A000");
        data.push_str("00H00000@Dm1S007@DUSg00?OdTnH7YhOfTL<7Yh@Cl0700?@Ah0300700000000");
        data.push_str("<008001QL00ZA41a@6HnI<1i@FHLM81M@@0LG81?O`0nC?Y7?`0ZA7Y300080000");
        data.push_str("O`082000Oh0827mo6>Hn?Wmo?6HnMb11MP08@C11H`08@FP0@@0004@000000000");
        data.push_str("00P00001Oab00003OcKP0006@6=PMgl<@440MglH@000000`@000001P00000000");
        data.push_str("Ob@8@@00Ob@8@Ga13R@8Mga172@8?PAo3R@827QoOb@820@0O`0007`0000007P0");
        data.push_str("O`000P08Od400g`<3V=P0G`673IP0`@3>1`00P@6O`P00g`<O`000GP800000000");
        data.push_str("?P9PL020O`<`N3R0@E4HC7b0@ET<ATB0@@l6C4B0O`H3N7b0?P01L3R000000020");

        let mut font = Image::new(128, 48);
        let (mut px, mut py) = (0, 0);
        let bytes = data.as_bytes();
        for b in (0..1024).step_by(4) {
            let sym1 = u32::from(bytes[b]) - 48;
            let sym2 = u32::from(bytes[b + 1]) - 48;
            let sym3 = u32::from(bytes[b + 2]) - 48;
            let sym4 = u32::from(bytes[b + 3]) - 48;
            let r = sym1 << 18 | sym2 << 12 | sym3 << 6 | sym4;
            for i in 0..24 {
                let k = if r & (1 << i) > 0 { 255 } else { 0 };
                font.put_pixel(px, py, Pixel::rgba(k, k, k, k));
                py += 1;
                if py == 48 {
                    px += 1;
                    py = 0;
                }
            }
        }
        font
    }

    // Get/Set ==============================================================

    /// Returns a reference to the active draw target
    ///
    /// panics if draw_target is not cleared properly and references an invalid
    /// target
    pub fn get_draw_target(&self) -> ImageRef {
        match &self.draw_target {
            Some(target) => target.clone(),
            None => self.default_draw_target.clone(),
        }
    }
    /// Specify which image should be the target for draw functions
    ///
    /// Note you must call clear_draw_target when finished. otherwise
    /// get_draw_target will likely panic
    pub fn set_draw_target(&mut self, target: ImageRef) {
        self.draw_target = Some(target);
    }
    pub fn get_draw_target_dims(&self) -> (u32, u32) {
        let target = self.get_draw_target();
        let target = target.borrow();
        (target.width(), target.height())
    }
    pub fn clear_draw_target(&mut self) {
        self.draw_target = None;
    }
    pub fn get_alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    pub fn set_alpha_mode(&mut self, mode: AlphaMode) {
        self.alpha_mode = mode;
    }
    pub fn set_alpha_blend(&mut self, blend: f32) {
        self.blend_factor = if blend < 0.0 {
            0.0
        } else if blend > 1.0 {
            1.0
        } else {
            blend
        };
    }
    // Enables or disables screen-space coordinate wrapping
    pub fn enable_coord_wrapping(&mut self, val: bool) {
        self.coord_wrapping = val;
    }
    // Gets the Pixel color for draw target
    pub fn get_draw_color(&mut self) -> Pixel {
        self.draw_color
    }
    // Sets the Pixel color for draw target
    pub fn set_draw_color(&mut self, p: Pixel) -> PixEngineResult<()> {
        self.draw_color = p;
        self.renderer.set_draw_color(p)
    }
    // Resets color for draw target
    pub fn reset_draw_color(&mut self) {
        self.draw_color = self.default_draw_color;
    }
    // Gets the scale factor for draw target
    pub fn get_draw_scale(&mut self) -> u32 {
        self.draw_scale
    }
    // Sets the scale factor for draw target
    pub fn set_draw_scale(&mut self, scale: u32) {
        self.draw_scale = scale;
    }
    // Utility functions =========================================================

    // Wraps (x, y) coordinates around screen width/height into (ox, oy)
    pub fn wrap_coords(&self, x: f32, y: f32, ox: &mut f32, oy: &mut f32) {
        *ox = if x < 0.0 {
            x + self.screen_width as f32
        } else if x >= self.screen_width as f32 {
            x - self.screen_width as f32
        } else {
            x
        };
        *oy = if y < 0.0 {
            y + self.screen_height as f32
        } else if y >= self.screen_height as f32 {
            y - self.screen_height as f32
        } else {
            y
        };
    }

    // Draw functions =========================================================

    // Fills entire draw target to Pixel
    pub fn fill(&mut self, p: Pixel) -> PixEngineResult<()> {
        let (width, height) = self.get_draw_target_dims();
        self.renderer.set_draw_color(p)?;
        self.fill_rect(Rect::new(0, 0, width, height))
    }

    // Clears entire draw target to current draw color
    pub fn clear(&mut self) -> PixEngineResult<()> {
        self.renderer.clear()
    }

    // Draws a single pixel to the draw target
    fn draw_point_i32(&mut self, x: i32, y: i32) {
        self.draw_point(x as u32, y as u32);
    }

    #[allow(clippy::many_single_char_names)]
    pub fn draw_point(&mut self, mut x: u32, mut y: u32) {
        if self.coord_wrapping {
            let (mut ox, mut oy) = (0.0, 0.0);
            self.wrap_coords(x as f32, y as f32, &mut ox, &mut oy);
            x = ox as u32;
            y = oy as u32;
        }
        // These local assignments get around the borrow checker when target is assigned
        let alpha_mode = self.alpha_mode;
        let blend_factor = self.blend_factor;

        let point = Point::new(x, y);
        let _ = match alpha_mode {
            AlphaMode::Normal => self.renderer.draw_point(point),
            AlphaMode::Mask => {
                if self.get_draw_color().a() == 255 {
                    self.renderer.draw_point(point)
                } else {
                    Ok(())
                }
            }
            AlphaMode::Blend => {
                Ok(())
                // let current_p = target.get_pixel(x, y);
                // let a = (f32::from(p.a()) / 255.0) * blend_factor;
                // let c = 1.0 - a;
                // let r = a * f32::from(p.r()) + c * f32::from(current_p.r());
                // let g = a * f32::from(p.g()) + c * f32::from(current_p.g());
                // let b = a * f32::from(p.b()) + c * f32::from(current_p.b());
                // let blended = Pixel::rgb(r as u8, g as u8, b as u8);
                // self.renderer.set_draw_color(blended);
                // self.renderer.draw_point(point)
            }
        };
    }

    // Draws a line from (x1, y1) to (x2, y2)
    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.draw_line_pattern(x1, y1, x2, y2, 0xFFFF_FFFF);
    }
    pub fn draw_line_i32(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.draw_line(x1 as u32, y1 as u32, x2 as u32, y2 as u32)
    }

    // Draws a line pattern from (x1, y1) to (x2, y2)
    pub fn draw_line_pattern(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, mut pattern: u32) {
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let mut x2 = x2 as i32;
        let mut y2 = y2 as i32;
        let dx = x2 - x1;
        let dy = y2 - y1;

        let mut rol = || {
            pattern = (pattern << 1) | (pattern >> 31);
            pattern & 1 > 0
        };

        if dx == 0 {
            // Vertical
            if y2 < y1 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                if rol() {
                    self.draw_point_i32(x1, y);
                }
            }
        } else if dy == 0 {
            // Horizontal
            if x2 < x1 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for x in x1..=x2 {
                if rol() {
                    self.draw_point_i32(x, y1);
                }
            }
        } else {
            // Diagonal
            let dx1 = dx.abs();
            let dy1 = dy.abs();
            let (mut x, mut y, xe, ye);
            let mut px = 2 * dy1 - dx1;
            let mut py = 2 * dx1 - dy1;
            if dy1 <= dx1 {
                if dx >= 0 {
                    x = x1;
                    y = y1;
                    xe = x2;
                } else {
                    x = x2;
                    y = y2;
                    xe = x1;
                }
                if rol() {
                    self.draw_point_i32(x, y);
                }
                while x < xe {
                    x += 1;
                    if px < 0 {
                        px += 2 * dy1;
                    } else {
                        if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                            y += 1;
                        } else {
                            y -= 1;
                        }
                        px += 2 * (dy1 - dx1);
                    }
                    if rol() {
                        self.draw_point_i32(x, y);
                    }
                }
            } else {
                if dy >= 0 {
                    x = x1;
                    y = y1;
                    ye = y2;
                } else {
                    x = x2;
                    y = y2;
                    ye = y1;
                }
                if rol() {
                    self.draw_point_i32(x, y);
                }
                while y < ye {
                    y += 1;
                    if py < 0 {
                        py += 2 * dx1;
                    } else {
                        if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                            x += 1;
                        } else {
                            x -= 1;
                        }
                        py += 2 * (dx1 - dy1);
                    }
                    if rol() {
                        self.draw_point_i32(x, y);
                    }
                }
            }
        }
    }

    // Draws a circle centered at (x, y) with radius r
    pub fn draw_circle(&mut self, x: u32, y: u32, r: u32) {
        self.draw_partial_circle(x, y, r, 0xFF);
    }

    // Draws a partial circle centered at (x, y) with radius r, partially masked
    #[allow(clippy::many_single_char_names)]
    pub fn draw_partial_circle(&mut self, x: u32, y: u32, r: u32, mask: u8) {
        let x = x as i32;
        let y = y as i32;
        let mut x0 = 0;
        let mut y0 = r as i32;
        let mut d = 3 - 2 * r as i32;
        if r == 0 {
            return;
        }

        while y0 >= x0 {
            if mask & 0x01 > 0 {
                self.draw_point_i32(x + x0, y - y0);
            }
            if mask & 0x02 > 0 {
                self.draw_point_i32(x + y0, y - x0);
            }
            if mask & 0x04 > 0 {
                self.draw_point_i32(x + y0, y + x0);
            }
            if mask & 0x08 > 0 {
                self.draw_point_i32(x + x0, y + y0);
            }
            if mask & 0x10 > 0 {
                self.draw_point_i32(x - x0, y + y0);
            }
            if mask & 0x20 > 0 {
                self.draw_point_i32(x - y0, y + x0);
            }
            if mask & 0x40 > 0 {
                self.draw_point_i32(x - y0, y - x0);
            }
            if mask & 0x80 > 0 {
                self.draw_point_i32(x - x0, y - y0);
            }
            x0 += 1;
            if d < 0 {
                d += 4 * x0 + 6;
            } else {
                y0 -= 1;
                d += 4 * (x0 - y0) + 10;
            }
        }
    }

    // Draws a filled circle centered at (x, y) with radius r
    #[allow(clippy::many_single_char_names)]
    pub fn fill_circle(&mut self, x: u32, y: u32, r: u32) {
        let x = x as i32;
        let y = y as i32;
        let mut x0 = 0;
        let mut y0 = r as i32;
        let mut d = 3 - 2 * r as i32;
        if r == 0 {
            return;
        }

        let mut draw_hline = |sx, ex, ny| {
            for i in sx..ex {
                self.draw_point_i32(i, ny);
            }
        };

        while y0 >= x0 {
            draw_hline(x - x0, x + x0, y - y0);
            draw_hline(x - y0, x + y0, y - x0);
            draw_hline(x - x0, x + x0, y + y0);
            draw_hline(x - y0, x + y0, y + x0);
            x0 += 1;
            if d < 0 {
                d += 4 * x0 + 6;
            } else {
                y0 -= 1;
                d += 4 * (x0 - y0) + 10;
            }
        }
    }

    pub fn draw_elipse(&mut self) {
        // TODO
    }

    pub fn fill_elipse(&mut self) {
        // TODO
    }

    // Draws a rectangle at (x, y) to (x + w, y + h)
    #[allow(clippy::many_single_char_names)]
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.draw_line(x, y, x + w, y); // Top
        self.draw_line(x + w, y, x + w, y + h); // Right
        self.draw_line(x + w, y + h, x, y + h); // Bottom
        self.draw_line(x, y + h, x, y); // Left
    }

    // Draws a filled rectangle at (x, y) to (x + w, y + h)
    #[allow(clippy::many_single_char_names)]
    pub fn fill_rect(&mut self, rect: Rect) -> PixEngineResult<()> {
        self.renderer.fill_rect(rect)
    }

    // Draws a triangle between points (x1, y1), (x2, y2), and (x3, y3)
    #[allow(clippy::too_many_arguments)]
    pub fn draw_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
        self.draw_line(x1, y1, x2, y2);
        self.draw_line(x2, y2, x3, y3);
        self.draw_line(x3, y3, x1, y1);
    }

    // Draws a filled triangle between points (x1, y1), (x2, y2), and (x3, y3)
    // https://www.avrfreaks.net/sites/default/files/triangles.c
    // Original Author: Adafruit Industries
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let mut x2 = x2 as i32;
        let mut y2 = y2 as i32;
        let mut x3 = x3 as i32;
        let mut y3 = y3 as i32;
        // Sort coords by y (y3 >= y1 >= y0)
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
            std::mem::swap(&mut x1, &mut x2);
        }
        if y2 > y3 {
            std::mem::swap(&mut y3, &mut y2);
            std::mem::swap(&mut x3, &mut x2);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
            std::mem::swap(&mut x1, &mut x2);
        }

        if y1 == y3 {
            // All on same line
            let mut a = x1;
            let mut b = x1;
            if x2 < a {
                a = x2;
            } else if x2 > b {
                b = x2;
            }
            if x3 < a {
                a = x3;
            } else if x3 > b {
                b = x3;
            }
            self.draw_line_i32(a, y1, b, y1); // Horizontal line
        } else {
            let dx12 = x2 - x1;
            let dy12 = y2 - y1;
            let dx13 = x3 - x1;
            let dy13 = y3 - y1;
            let dx23 = x3 - x2;
            let dy23 = y3 - y2;
            let mut sa = 0;
            let mut sb = 0;

            let last = if y2 == y3 { y2 } else { y2 - 1 };

            for y in y1..=last {
                let a = x1 + sa / dy12;
                let b = x1 + sb / dy13;
                sa += dx12;
                sb += dx13;
                self.draw_line_i32(a, y, b, y);
            }

            sa = dx23 * (last - y2);
            sb = dx13 * (last - y1);
            for y in last..=y3 {
                let a = x2 + sa / dy23;
                let b = x1 + sb / dy13;
                sa += dx23;
                sb += dx13;
                self.draw_line_i32(a, y, b, y);
            }
        }
    }

    // Draws an entire image at location (x, y)
    pub fn draw_image(&mut self, x: u32, y: u32, image: &Image) {
        if self.draw_scale > 1 {
            for oy in 0..image.height() {
                for ox in 0..image.width() {
                    for ys in 0..self.draw_scale {
                        for xs in 0..self.draw_scale {
                            self.set_draw_color(image.get_pixel(ox, oy));
                            self.draw_point(
                                x + (ox * self.draw_scale) + xs,
                                y + (oy * self.draw_scale) + ys,
                            );
                        }
                    }
                }
            }
        } else {
            for oy in 0..image.height() {
                for ox in 0..image.width() {
                    self.set_draw_color(image.get_pixel(ox, oy));
                    self.draw_point(x + ox, y + oy);
                }
            }
        }
    }

    // Draws part of a image at location (x, y) where the drawn area
    // is (ox, oy) to (ox + w, oy + h)
    #[allow(clippy::too_many_arguments)]
    pub fn draw_partial_image(
        &mut self,
        x: u32,
        y: u32,
        ox: u32,
        oy: u32,
        w: u32,
        h: u32,
        image: &Image,
    ) {
        if self.draw_scale > 1 {
            for oy1 in 0..h {
                for ox1 in 0..w {
                    for ys in 0..self.draw_scale {
                        for xs in 0..self.draw_scale {
                            self.set_draw_color(image.get_pixel(ox1 + ox, oy1 + oy));
                            self.draw_point(
                                x + (ox1 * self.draw_scale) + xs,
                                y + (oy1 * self.draw_scale) + ys,
                            );
                        }
                    }
                }
            }
        } else {
            for oy1 in 0..h {
                for ox1 in 0..w {
                    self.set_draw_color(image.get_pixel(ox1 + ox, oy1 + oy));
                    self.draw_point(x + ox1, y + oy1);
                }
            }
        }
    }

    // Draws a single line of text at (x, y)
    pub fn draw_string(&mut self, x: u32, y: u32, text: &str) {
        let mut sx = 0;
        let mut sy = 0;
        for c in text.chars() {
            if c == '\n' {
                sx = 0;
                sy += 8 * self.draw_scale;
            } else {
                let ox = (c as u32 - 32) % 16;
                let oy = (c as u32 - 32) / 16;
                if self.draw_scale > 1 {
                    for oy1 in 0..8 {
                        for ox1 in 0..8 {
                            if self.font.get_pixel(ox1 + ox * 8, oy1 + oy * 8).a() > 0 {
                                for ys in 0..self.draw_scale {
                                    for xs in 0..self.draw_scale {
                                        self.draw_point(
                                            x + sx + (ox1 * self.draw_scale) + xs,
                                            y + sy + (oy1 * self.draw_scale) + ys,
                                        );
                                    }
                                }
                            }
                        }
                    }
                } else {
                    for oy1 in 0..8 {
                        for ox1 in 0..8 {
                            if self.font.get_pixel(ox1 + ox * 8, oy1 + oy * 8).a() > 0 {
                                self.draw_point(x + sx + ox1, y + sy + oy1);
                            }
                        }
                    }
                }
                sx += 8 * self.draw_scale;
            }
        }
    }

    // Draws a wireframe model based on a set of vertices
    pub fn draw_wireframe(
        &mut self,
        model_coords: &[(f32, f32)],
        x: f32,
        y: f32,
        angle: f32,
        scale: f32,
    ) {
        let verts = model_coords.len();
        let mut transformed_coords = vec![(0.0, 0.0); verts];

        // [ 0.0, -5.0]
        // [-2.5,  2.5]
        // [ 2.5,  2.5]
        //
        // n = 0, m = 0 -> (0, 0) -> (-5.0)
        // n = 0, m = 1 -> (0, 1) ->

        // Rotate
        for i in 0..verts {
            transformed_coords[i].0 =
                model_coords[i].0 * angle.cos() - model_coords[i].1 * angle.sin();
            transformed_coords[i].1 =
                model_coords[i].0 * angle.sin() + model_coords[i].1 * angle.cos();
        }

        // Scale
        for coord in transformed_coords.iter_mut() {
            coord.0 *= scale;
            coord.1 *= scale;
        }

        // Translate
        for coord in transformed_coords.iter_mut() {
            coord.0 += x;
            coord.1 += y;
        }

        // Draw
        for i in 0..=verts {
            let j = i + 1;
            self.draw_line(
                transformed_coords[i % verts].0 as u32,
                transformed_coords[i % verts].1 as u32,
                transformed_coords[j % verts].0 as u32,
                transformed_coords[j % verts].1 as u32,
            );
        }
    }

    pub fn create_texture(
        &mut self,
        name: &'static str,
        color_type: ColorType,
        src: Rect,
        dst: Rect,
    ) -> PixEngineResult<()> {
        self.create_window_texture(self.main_window_id, name, color_type, src, dst)
    }

    pub fn create_window_texture(
        &mut self,
        window_id: u32,
        name: &'static str,
        color_type: ColorType,
        src: Rect,
        dst: Rect,
    ) -> PixEngineResult<()> {
        self.renderer
            .create_texture(window_id, name, color_type, src, dst)
    }

    pub fn copy_draw_target(&mut self, name: &str) -> PixEngineResult<()> {
        self.copy_window_draw_target(self.main_window_id, name)
    }

    pub fn copy_window_draw_target(&mut self, window_id: u32, name: &str) -> PixEngineResult<()> {
        // TODO add size check for draw_target to texture dimensions
        let target = self.get_draw_target();
        let target = target.borrow();
        let renderer = &mut self.renderer;
        let pixels = target.bytes();
        renderer.copy_texture(window_id, name, &pixels)?;
        Ok(())
    }

    pub fn copy_image(&mut self, image: &Image) -> PixEngineResult<()> {
        let main_screen = format!("screen{}", self.main_window_id);
        self.renderer
            .copy_texture(self.main_window_id, &main_screen, image.bytes())
    }

    pub fn copy_texture(&mut self, name: &str, bytes: &[u8]) -> PixEngineResult<()> {
        self.copy_window_texture(self.main_window_id, name, bytes)
    }

    pub fn copy_window_texture(
        &mut self,
        window_id: u32,
        name: &str,
        bytes: &[u8],
    ) -> PixEngineResult<()> {
        self.renderer.copy_texture(window_id, name, bytes)
    }

    pub fn open_window(
        &mut self,
        title: &'static str,
        width: u32,
        height: u32,
    ) -> PixEngineResult<u32> {
        self.renderer.open_window(title, width, height)
    }

    pub fn close_window(&mut self, window_id: u32) {
        self.renderer.close_window(window_id);
    }

    pub fn set_viewport(&mut self, rect: Option<Rect>) -> PixEngineResult<()> {
        self.renderer.set_viewport(rect)
    }
}
