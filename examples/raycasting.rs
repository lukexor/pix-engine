use pix_engine::prelude::*;

struct Light {
    pos: Vector,
    color: Color,
    rays: Vec<Ray>,
}

impl Light {
    pub fn new<P: Into<Vector>>(pos: P) -> Self {
        Self {
            pos: pos.into(),
            color: Color::random_rgba(),
            rays: (0..360)
                .into_iter()
                .map(|angle| Ray::new((angle as f64).to_radians()))
                .collect(),
        }
    }
}

impl Drawable for Light {
    fn draw(&mut self, s: &mut State) -> StateResult<()> {
        s.stroke(self.color);
        for ray in self.rays.iter_mut() {
            ray.draw(s)?;
        }
        Ok(())
    }
}

struct Ray {
    pos: Vector,
    looking: Vector,
}

impl Ray {
    fn new(angle: f64) -> Self {
        Self {
            pos: Vector::new(0),
            looking: Vector::from_angle(angle, 1.0),
        }
    }

    fn pos<P: Into<Vector>>(&mut self, pos: P) {
        self.pos = pos.into();
    }

    fn look_at(&mut self, point: Vector) {
        self.looking = point - self.pos;
    }

    fn cast(&mut self, pos: Vector, boundary: &Line) -> Option<Vector> {
        // Formula: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
        let x1 = boundary.start.x as f64;
        let y1 = boundary.start.y as f64;
        let x2 = boundary.end.x as f64;
        let y2 = boundary.end.y as f64;

        let x3 = pos.x;
        let y3 = pos.y;
        let x4 = pos.x + self.looking.x;
        let y4 = pos.y + self.looking.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator == 0.0 {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            return Some(Vector::new((x1 + t * (x2 - x1), y1 + t * (y2 - y1))));
        }
        None
    }
}

impl Drawable for Ray {
    fn draw(&mut self, s: &mut State) -> StateResult<()> {
        s.stroke_weight(1);
        // TODO Switch to using translate
        // s.translate(self.pos.x, self.pos.y);
        // s.draw_line((0, 0), Point::from(self.looking));
        let looking = self.looking + self.pos;
        Ok(s.draw_line(Point::from(self.pos), Point::from(looking))?)
    }
}

struct App {
    boundaries: Vec<Line>,
    light: Light,
}

impl App {
    fn new() -> Self {
        Self {
            boundaries: Vec::new(),
            light: Light::new(0),
        }
    }
}

impl PixApp for App {
    fn on_start(&mut self, s: &mut State) -> Result<bool> {
        let w = s.width() as i32;
        let h = s.height() as i32;
        self.boundaries.push(Line::new((0, -1), (w, -1))); // Top
        self.boundaries.push(Line::new((w, 0), (w, h))); // Right
        self.boundaries.push(Line::new((0, h), (w, h))); // Bottom
        self.boundaries.push(Line::new((-1, 0), (-1, h))); // Left

        for _ in 0..10 {
            let x1 = random(w);
            let y1 = random(h);
            let x2 = random(w);
            let y2 = random(h);
            self.boundaries.push(Line::new((x1, y1), (x2, y2)));
        }
        self.light = Light::new((s.width() as f64 / 2.0, s.height() as f64 / 2.0));
        Ok(true)
    }

    fn on_update(&mut self, s: &mut State) -> Result<bool> {
        s.background(51);

        self.light.pos = s.mouse_pos().into();

        for ray in self.light.rays.iter_mut() {
            let mut closest = None;
            let mut closest_dist = INFINITY;
            for b in self.boundaries.iter() {
                if let Some(point) = ray.cast(self.light.pos, b) {
                    let dist = self.light.pos.dist(point);
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest = Some(point);
                    }
                }
            }
            if let Some(point) = closest {
                ray.pos(self.light.pos);
                ray.look_at(point);
            }
        }

        for b in self.boundaries.iter_mut() {
            s.stroke(255);
            s.stroke_weight(2);
            b.draw(s)?;
        }
        self.light.draw(s)?;

        Ok(true)
    }

    fn on_stop(&mut self, _s: &mut State) -> Result<bool> {
        Ok(true)
    }
}

fn main() {
    let app = App::new();
    PixEngine::create("Raycasting", app, 1024, 768)
        .build()
        .expect("engine")
        .run()
        .expect("ran");
}
