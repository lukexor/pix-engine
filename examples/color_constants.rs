use pix_engine::prelude::*;

const COLS: u32 = 12;
const ROWS: u32 = 12;
const SIZE: u32 = 60;
const WIDTH: u32 = COLS * SIZE;
const HEIGHT: u32 = ROWS * SIZE;
const COLORS: [(Color, &str); 142] = [
    (Color::ALICE_BLUE, "ALICE_BLUE"),
    (Color::ANTIQUE_WHITE, "ANTIQUE_WHITE"),
    (Color::AQUA, "AQUA"),
    (Color::AQUA_MARINE, "AQUA_MARINE"),
    (Color::AZURE, "AZURE"),
    (Color::BEIGE, "BEIGE"),
    (Color::BISQUE, "BISQUE"),
    (Color::BLACK, "BLACK"),
    (Color::BLANCHE_DALMOND, "BLANCHE_DALMOND"),
    (Color::BLUE, "BLUE"),
    (Color::BLUE_VIOLET, "BLUE_VIOLET"),
    (Color::BROWN, "BROWN"),
    (Color::BURLY_WOOD, "BURLY_WOOD"),
    (Color::CADET_BLUE, "CADET_BLUE"),
    (Color::CHARTREUSE, "CHARTREUSE"),
    (Color::CHOCOLATE, "CHOCOLATE"),
    (Color::CORAL, "CORAL"),
    (Color::CORNFLOWER_BLUE, "CORNFLOWER_BLUE"),
    (Color::CORN_SILK, "CORN_SILK"),
    (Color::CRIMSON, "CRIMSON"),
    (Color::CYAN, "CYAN"),
    (Color::DARK_BLUE, "DARK_BLUE"),
    (Color::DARK_CYAN, "DARK_CYAN"),
    (Color::DARK_GOLDENROD, "DARK_GOLDENROD"),
    (Color::DARK_GRAY, "DARK_GRAY"),
    (Color::DARK_GREEN, "DARK_GREEN"),
    (Color::DARK_KHAKI, "DARK_KHAKI"),
    (Color::DARK_MAGENTA, "DARK_MAGENTA"),
    (Color::DARK_OLIVE_GREEN, "DARK_OLIVE_GREEN"),
    (Color::DARK_ORANGE, "DARK_ORANGE"),
    (Color::DARK_ORCHID, "DARK_ORCHID"),
    (Color::DARK_RED, "DARK_RED"),
    (Color::DARK_SALMON, "DARK_SALMON"),
    (Color::DARK_SEA_GREEN, "DARK_SEA_GREEN"),
    (Color::DARK_SLATE_BLUE, "DARK_SLATE_BLUE"),
    (Color::DARK_SLATE_GRAY, "DARK_SLATE_GRAY"),
    (Color::DARK_TURQUOISE, "DARK_TURQUOISE"),
    (Color::DARK_VIOLET, "DARK_VIOLET"),
    (Color::DEEP_PINK, "DEEP_PINK"),
    (Color::DEEP_SKY_BLUE, "DEEP_SKY_BLUE"),
    (Color::DIM_GRAY, "DIM_GRAY"),
    (Color::DODGER_BLUE, "DODGER_BLUE"),
    (Color::FIRE_BRICK, "FIRE_BRICK"),
    (Color::FLORAL_WHITE, "FLORAL_WHITE"),
    (Color::FOREST_GREEN, "FOREST_GREEN"),
    (Color::FUCHSIA, "FUCHSIA"),
    (Color::GAINSBORO, "GAINSBORO"),
    (Color::GHOST_WHITE, "GHOST_WHITE"),
    (Color::GOLD, "GOLD"),
    (Color::GOLDENROD, "GOLDENROD"),
    (Color::GRAY, "GRAY"),
    (Color::GREEN, "GREEN"),
    (Color::GREEN_YELLOW, "GREEN_YELLOW"),
    (Color::HONEYDEW, "HONEYDEW"),
    (Color::HOTOINK, "HOTOINK"),
    (Color::INDIAN_RED, "INDIAN_RED"),
    (Color::INDIGO, "INDIGO"),
    (Color::IVORY, "IVORY"),
    (Color::KHAKI, "KHAKI"),
    (Color::LAVENDER, "LAVENDER"),
    (Color::LAVENDER_BLUSH, "LAVENDER_BLUSH"),
    (Color::LAWN_GREEN, "LAWN_GREEN"),
    (Color::LEMON_CHIFFON, "LEMON_CHIFFON"),
    (Color::LIGHT_BLUE, "LIGHT_BLUE"),
    (Color::LIGHT_CORAL, "LIGHT_CORAL"),
    (Color::LIGHT_CYAN, "LIGHT_CYAN"),
    (Color::LIGHT_GOLDENROD_YELLOW, "LIGHT_GOLDENROD_YELLOW"),
    (Color::LIGHT_GRAY, "LIGHT_GRAY"),
    (Color::LIGHT_GREEN, "LIGHT_GREEN"),
    (Color::LIGHT_PINK, "LIGHT_PINK"),
    (Color::LIGHT_SALMON, "LIGHT_SALMON"),
    (Color::LIGHT_SEA_GREEN, "LIGHT_SEA_GREEN"),
    (Color::LIGHT_SKY_BLUE, "LIGHT_SKY_BLUE"),
    (Color::LIGHT_SLATE_GRAY, "LIGHT_SLATE_GRAY"),
    (Color::LIGHT_STEEL_BLUE, "LIGHT_STEEL_BLUE"),
    (Color::LIGHT_YELLOW, "LIGHT_YELLOW"),
    (Color::LIME, "LIME"),
    (Color::LIME_GREEN, "LIME_GREEN"),
    (Color::LINEN, "LINEN"),
    (Color::MAGENTA, "MAGENTA"),
    (Color::MAROON, "MAROON"),
    (Color::MEDIUMAQUA_MARINE, "MEDIUMAQUA_MARINE"),
    (Color::MEDIUM_BLUE, "MEDIUM_BLUE"),
    (Color::MEDIUM_ORCHID, "MEDIUM_ORCHID"),
    (Color::MEDIUM_PURPLE, "MEDIUM_PURPLE"),
    (Color::MEDIUM_SEA_GREEN, "MEDIUM_SEA_GREEN"),
    (Color::MEDIUM_SLATE_BLUE, "MEDIUM_SLATE_BLUE"),
    (Color::MEDIUM_SPRING_GREEN, "MEDIUM_SPRING_GREEN"),
    (Color::MEDIUM_TURQUOISE, "MEDIUM_TURQUOISE"),
    (Color::MEDIUM_VIOLET_RED, "MEDIUM_VIOLET_RED"),
    (Color::MIDNIGHT_BLUE, "MIDNIGHT_BLUE"),
    (Color::MINT_CREAM, "MINT_CREAM"),
    (Color::MISTY_ROSE, "MISTY_ROSE"),
    (Color::MOCCASIN, "MOCCASIN"),
    (Color::NAVAJO_WHITE, "NAVAJO_WHITE"),
    (Color::NAVY, "NAVY"),
    (Color::OLD_LACE, "OLD_LACE"),
    (Color::OLIVE, "OLIVE"),
    (Color::OLIVE_DRAB, "OLIVE_DRAB"),
    (Color::ORANGE, "ORANGE"),
    (Color::ORANGE_RED, "ORANGE_RED"),
    (Color::ORCHID, "ORCHID"),
    (Color::PALE_GOLDENROD, "PALE_GOLDENROD"),
    (Color::PALE_GREEN, "PALE_GREEN"),
    (Color::PALE_TURQUOISE, "PALE_TURQUOISE"),
    (Color::PALE_VIOLET_RED, "PALE_VIOLET_RED"),
    (Color::PAPAYA_WHIP, "PAPAYA_WHIP"),
    (Color::PEACH_PUFF, "PEACH_PUFF"),
    (Color::PERU, "PERU"),
    (Color::PINK, "PINK"),
    (Color::PLUM, "PLUM"),
    (Color::POWDER_BLUE, "POWDER_BLUE"),
    (Color::PURPLE, "PURPLE"),
    (Color::REBECCA_PURPLE, "REBECCA_PURPLE"),
    (Color::RED, "RED"),
    (Color::ROSY_BROWN, "ROSY_BROWN"),
    (Color::ROYAL_BLUE, "ROYAL_BLUE"),
    (Color::SADDLE_BROWN, "SADDLE_BROWN"),
    (Color::SALMON, "SALMON"),
    (Color::SANDY_BROWN, "SANDY_BROWN"),
    (Color::SEA_GREEN, "SEA_GREEN"),
    (Color::SEA_SHELL, "SEA_SHELL"),
    (Color::SIENNA, "SIENNA"),
    (Color::SILVER, "SILVER"),
    (Color::SKY_BLUE, "SKY_BLUE"),
    (Color::SLATE_BLUE, "SLATE_BLUE"),
    (Color::SLATE_GRAY, "SLATE_GRAY"),
    (Color::SNOW, "SNOW"),
    (Color::SPRING_GREEN, "SPRING_GREEN"),
    (Color::STEEL_BLUE, "STEEL_BLUE"),
    (Color::TAN, "TAN"),
    (Color::TEAL, "TEAL"),
    (Color::THISTLE, "THISTLE"),
    (Color::TOMATO, "TOMATO"),
    (Color::TRANSPARENT, "TRANSPARENT"),
    (Color::TURQUOISE, "TURQUOISE"),
    (Color::VIOLET, "VIOLET"),
    (Color::WHEAT, "WHEAT"),
    (Color::WHITE, "WHITE"),
    (Color::WHITE_SMOKE, "WHITE_SMOKE"),
    (Color::YELLOW, "YELLOW"),
    (Color::YELLOW_GREEN, "YELLOW_GREEN"),
];

struct ColorConsts {
    cells: Vec<Rect<i32>>,
}

impl ColorConsts {
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(COLORS.len());
        for i in 0..COLORS.len() {
            let cols = COLS as i32;
            let size = SIZE as i32;
            let col = i as i32 % cols;
            let row = i as i32 / cols;
            let x = col * size;
            let y = row * size;
            cells.push(square![x, y, size]);
        }
        Self { cells }
    }
}

impl AppState for ColorConsts {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.stroke(None);
        for (i, &color) in COLORS.iter().enumerate() {
            s.fill(color.0);
            s.square(self.cells[i])?;
        }
        for (i, &color) in COLORS.iter().enumerate() {
            let pos = s.mouse_pos();
            if self.cells[i].contains_point(pos) {
                s.tooltip(color.1)?;
            }
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("SVG Color Constants")
        .build()?;
    let mut app = ColorConsts::new();
    engine.run(&mut app)
}
