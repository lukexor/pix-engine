use pix_engine::prelude::*;

struct Shapes {
    rotation: f64,
    scale: f64,
    scale_latch: bool,
}

impl AppState for Shapes {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(50);
        s.stroke(Color::BLACK);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        s.stroke(Color::CYAN);
        for x in (10..210).step_by(3) {
            s.point([x, x])?;
        }

        s.stroke(Color::RED);
        s.line([[10, 200], [300, 200]])?;

        s.stroke(Color::LIGHT_YELLOW);
        s.line([[185, 20], [110, 10]])?;
        s.line([[190, 90], [115, 80]])?;
        s.stroke(Color::STEEL_BLUE);
        s.bezier([[185, 20], [110, 10], [190, 90], [115, 80]])?;

        s.stroke(Color::YELLOW);
        s.fill(Color::DARK_BLUE);
        s.triangle([[230, 20], [250, 180], [400, 155]])?;

        s.stroke(Color::GAINSBORO);
        s.fill(Color::DARK_ORANGE);
        s.square([20, 220, 100])?;

        s.stroke(Color::REBECCA_PURPLE);
        s.fill(Color::LIGHT_SEA_GREEN);
        s.rounded_rect([150, 220, 150, 100], 16)?;

        s.stroke(Color::CORNFLOWER_BLUE);
        s.fill(None);
        s.quad([[360, 30], [380, 120], [500, 160], [520, 40]])?;

        s.stroke(None);
        s.fill(Color::CORAL);
        s.polygon([[320, 230], [320, 320], [460, 360], [520, 240], [400, 300]])?;

        s.stroke(Color::CRIMSON);
        s.fill(None);
        let delta = s.delta_time().as_secs_f64();
        self.rotation += delta;
        if self.scale_latch {
            self.scale -= 50.0 * delta;
        } else {
            self.scale += 50.0 * delta;
        }
        if self.scale >= 120.0 || self.scale <= 20.0 {
            self.scale_latch = !self.scale_latch;
        }
        s.wireframe(
            [[1.0, 1.0], [0.0, 0.0], [0.0, 1.0], [1.0, 0.0]],
            [650, 160],
            self.rotation,
            self.scale,
        )?;

        s.ellipse_mode(EllipseMode::Center);
        s.fill(Color::SEA_GREEN);
        s.ellipse([70, 370, 120, 60])?;

        s.ellipse_mode(EllipseMode::Corner);
        s.fill(Color::SANDY_BROWN);
        s.circle([160, 340, 50])?;

        s.arc_mode(ArcMode::Default);
        s.stroke(Color::DARK_VIOLET);
        s.arc([350, 400], 50, 0, 200)?;

        s.arc_mode(ArcMode::Pie);
        s.stroke(Color::LIGHT_GOLDENROD_YELLOW);
        s.fill(Color::OLIVE_DRAB);
        s.arc([480, 420], 50, 100, 300)?;

        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("Shapes")
        .target_frame_rate(60)
        .build()?;
    let mut app = Shapes {
        rotation: 0.0,
        scale: 50.0,
        scale_latch: false,
    };
    engine.run(&mut app)
}
