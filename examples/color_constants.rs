use pix_engine::prelude::*;

const COLS: u32 = 12;
const ROWS: u32 = 12;
const SIZE: u32 = 60;
const WIDTH: u32 = COLS * SIZE;
const HEIGHT: u32 = ROWS * SIZE;
const COLORS: [(Color, &str); 142] = [
    (ALICE_BLUE, "ALICE_BLUE"),
    (ANTIQUE_WHITE, "ANTIQUE_WHITE"),
    (AQUA, "AQUA"),
    (AQUA_MARINE, "AQUA_MARINE"),
    (AZURE, "AZURE"),
    (BEIGE, "BEIGE"),
    (BISQUE, "BISQUE"),
    (BLACK, "BLACK"),
    (BLANCHE_DALMOND, "BLANCHE_DALMOND"),
    (BLUE, "BLUE"),
    (BLUE_VIOLET, "BLUE_VIOLET"),
    (BROWN, "BROWN"),
    (BURLY_WOOD, "BURLY_WOOD"),
    (CADET_BLUE, "CADET_BLUE"),
    (CHARTREUSE, "CHARTREUSE"),
    (CHOCOLATE, "CHOCOLATE"),
    (CORAL, "CORAL"),
    (CORNFLOWER_BLUE, "CORNFLOWER_BLUE"),
    (CORN_SILK, "CORN_SILK"),
    (CRIMSON, "CRIMSON"),
    (CYAN, "CYAN"),
    (DARK_BLUE, "DARK_BLUE"),
    (DARK_CYAN, "DARK_CYAN"),
    (DARK_GOLDENROD, "DARK_GOLDENROD"),
    (DARK_GRAY, "DARK_GRAY"),
    (DARK_GREEN, "DARK_GREEN"),
    (DARK_KHAKI, "DARK_KHAKI"),
    (DARK_MAGENTA, "DARK_MAGENTA"),
    (DARK_OLIVE_GREEN, "DARK_OLIVE_GREEN"),
    (DARK_ORANGE, "DARK_ORANGE"),
    (DARK_ORCHID, "DARK_ORCHID"),
    (DARK_RED, "DARK_RED"),
    (DARK_SALMON, "DARK_SALMON"),
    (DARK_SEA_GREEN, "DARK_SEA_GREEN"),
    (DARK_SLATE_BLUE, "DARK_SLATE_BLUE"),
    (DARK_SLATE_GRAY, "DARK_SLATE_GRAY"),
    (DARK_TURQUOISE, "DARK_TURQUOISE"),
    (DARK_VIOLET, "DARK_VIOLET"),
    (DEEP_PINK, "DEEP_PINK"),
    (DEEP_SKY_BLUE, "DEEP_SKY_BLUE"),
    (DIM_GRAY, "DIM_GRAY"),
    (DODGER_BLUE, "DODGER_BLUE"),
    (FIRE_BRICK, "FIRE_BRICK"),
    (FLORAL_WHITE, "FLORAL_WHITE"),
    (FOREST_GREEN, "FOREST_GREEN"),
    (FUCHSIA, "FUCHSIA"),
    (GAINSBORO, "GAINSBORO"),
    (GHOST_WHITE, "GHOST_WHITE"),
    (GOLD, "GOLD"),
    (GOLDENROD, "GOLDENROD"),
    (GRAY, "GRAY"),
    (GREEN, "GREEN"),
    (GREEN_YELLOW, "GREEN_YELLOW"),
    (HONEYDEW, "HONEYDEW"),
    (HOTOINK, "HOTOINK"),
    (INDIAN_RED, "INDIAN_RED"),
    (INDIGO, "INDIGO"),
    (IVORY, "IVORY"),
    (KHAKI, "KHAKI"),
    (LAVENDER, "LAVENDER"),
    (LAVENDER_BLUSH, "LAVENDER_BLUSH"),
    (LAWN_GREEN, "LAWN_GREEN"),
    (LEMON_CHIFFON, "LEMON_CHIFFON"),
    (LIGHT_BLUE, "LIGHT_BLUE"),
    (LIGHT_CORAL, "LIGHT_CORAL"),
    (LIGHT_CYAN, "LIGHT_CYAN"),
    (LIGHT_GOLDENROD_YELLOW, "LIGHT_GOLDENROD_YELLOW"),
    (LIGHT_GRAY, "LIGHT_GRAY"),
    (LIGHT_GREEN, "LIGHT_GREEN"),
    (LIGHT_PINK, "LIGHT_PINK"),
    (LIGHT_SALMON, "LIGHT_SALMON"),
    (LIGHT_SEA_GREEN, "LIGHT_SEA_GREEN"),
    (LIGHT_SKY_BLUE, "LIGHT_SKY_BLUE"),
    (LIGHT_SLATE_GRAY, "LIGHT_SLATE_GRAY"),
    (LIGHT_STEEL_BLUE, "LIGHT_STEEL_BLUE"),
    (LIGHT_YELLOW, "LIGHT_YELLOW"),
    (LIME, "LIME"),
    (LIME_GREEN, "LIME_GREEN"),
    (LINEN, "LINEN"),
    (MAGENTA, "MAGENTA"),
    (MAROON, "MAROON"),
    (MEDIUMAQUA_MARINE, "MEDIUMAQUA_MARINE"),
    (MEDIUM_BLUE, "MEDIUM_BLUE"),
    (MEDIUM_ORCHID, "MEDIUM_ORCHID"),
    (MEDIUM_PURPLE, "MEDIUM_PURPLE"),
    (MEDIUM_SEA_GREEN, "MEDIUM_SEA_GREEN"),
    (MEDIUM_SLATE_BLUE, "MEDIUM_SLATE_BLUE"),
    (MEDIUM_SPRING_GREEN, "MEDIUM_SPRING_GREEN"),
    (MEDIUM_TURQUOISE, "MEDIUM_TURQUOISE"),
    (MEDIUM_VIOLET_RED, "MEDIUM_VIOLET_RED"),
    (MIDNIGHT_BLUE, "MIDNIGHT_BLUE"),
    (MINT_CREAM, "MINT_CREAM"),
    (MISTY_ROSE, "MISTY_ROSE"),
    (MOCCASIN, "MOCCASIN"),
    (NAVAJO_WHITE, "NAVAJO_WHITE"),
    (NAVY, "NAVY"),
    (OLD_LACE, "OLD_LACE"),
    (OLIVE, "OLIVE"),
    (OLIVE_DRAB, "OLIVE_DRAB"),
    (ORANGE, "ORANGE"),
    (ORANGE_RED, "ORANGE_RED"),
    (ORCHID, "ORCHID"),
    (PALE_GOLDENROD, "PALE_GOLDENROD"),
    (PALE_GREEN, "PALE_GREEN"),
    (PALE_TURQUOISE, "PALE_TURQUOISE"),
    (PALE_VIOLET_RED, "PALE_VIOLET_RED"),
    (PAPAYA_WHIP, "PAPAYA_WHIP"),
    (PEACH_PUFF, "PEACH_PUFF"),
    (PERU, "PERU"),
    (PINK, "PINK"),
    (PLUM, "PLUM"),
    (POWDER_BLUE, "POWDER_BLUE"),
    (PURPLE, "PURPLE"),
    (REBECCA_PURPLE, "REBECCA_PURPLE"),
    (RED, "RED"),
    (ROSY_BROWN, "ROSY_BROWN"),
    (ROYAL_BLUE, "ROYAL_BLUE"),
    (SADDLE_BROWN, "SADDLE_BROWN"),
    (SALMON, "SALMON"),
    (SANDY_BROWN, "SANDY_BROWN"),
    (SEA_GREEN, "SEA_GREEN"),
    (SEA_SHELL, "SEA_SHELL"),
    (SIENNA, "SIENNA"),
    (SILVER, "SILVER"),
    (SKY_BLUE, "SKY_BLUE"),
    (SLATE_BLUE, "SLATE_BLUE"),
    (SLATE_GRAY, "SLATE_GRAY"),
    (SNOW, "SNOW"),
    (SPRING_GREEN, "SPRING_GREEN"),
    (STEEL_BLUE, "STEEL_BLUE"),
    (TAN, "TAN"),
    (TEAL, "TEAL"),
    (THISTLE, "THISTLE"),
    (TOMATO, "TOMATO"),
    (TRANSPARENT, "TRANSPARENT"),
    (TURQUOISE, "TURQUOISE"),
    (VIOLET, "VIOLET"),
    (WHEAT, "WHEAT"),
    (WHITE, "WHITE"),
    (WHITE_SMOKE, "WHITE_SMOKE"),
    (YELLOW, "YELLOW"),
    (YELLOW_GREEN, "YELLOW_GREEN"),
];

struct ColorConsts {
    hovered: Option<usize>,
}

impl AppState for ColorConsts {
    #[allow(clippy::many_single_char_names)]
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.no_stroke();
        for (i, &color) in COLORS.iter().enumerate() {
            let col = i as u32 % COLS;
            let row = i as u32 / COLS;
            let x = col * SIZE;
            let y = row * SIZE;
            s.fill(color.0);
            s.square([x, y, SIZE])?;
        }
        if let Some(idx) = self.hovered {
            if let Some(color) = COLORS.get(idx) {
                s.cursor(&Cursor::hand())?;
                let pos = s.mouse_pos();

                let (w, h) = s.size_of(color.1)?;
                let pad = 4;
                let w = w as i32 + pad * 2;
                let h = h as i32 + pad * 2;
                let mut x = pos.x() + 15;
                let mut y = pos.y() + 25;
                if x + w > WIDTH as i32 {
                    x -= w;
                }
                if y + h > HEIGHT as i32 {
                    y -= h;
                }

                s.fill(BLACK);
                s.rect([x, y, w, h])?;

                s.fill(WHITE);
                s.text([x + pad, y + pad], color.1)?;
            }
        }
        Ok(())
    }

    fn on_mouse_motion(&mut self, _: &mut PixState, pos: PointI2, _: i32, _: i32) -> PixResult<()> {
        let col = pos.x() as u32 / SIZE;
        let row = pos.y() as u32 / SIZE;
        let idx = (row * COLS + col) as usize;
        self.hovered = Some(idx);
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("SVG Color Constants")
        .position_centered()
        .build();
    let mut app = ColorConsts { hovered: None };
    engine.run(&mut app)
}
