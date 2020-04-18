use pix_engine::prelude::*;

struct Light {
    pos: Vector,
    color: Color,
    rays: Vec<Ray>,
}

impl Light {
    fn new() -> Self {
        Self {
            pos: Vector::new(0),
            color: Color::random_rgba(),
            rays: (0..360)
                .into_iter()
                .map(|angle| Ray::new((angle as f64).to_radians()))
                .collect(),
        }
    }
    fn pos<P: Into<Vector>>(&mut self, pos: P) {
        self.pos = pos.into();
    }
    fn update(&mut self, boundaries: &[Line], s: &mut State) {
        self.pos = s.mouse_pos().into();
        for ray in self.rays.iter_mut() {
            ray.pos(self.pos);
            let mut closest = None;
            let mut closest_dist = INFINITY;
            for b in boundaries.iter() {
                if let Some(point) = ray.cast(b) {
                    let dist = self.pos.dist(point);
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest = Some(point);
                    }
                }
            }
            if let Some(point) = closest {
                ray.look_at(point);
            }
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

    fn cast(&mut self, b: &Line) -> Option<Vector> {
        // Formula: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
        let (x1, y1): (f64, f64) = b.start.into();
        let (x2, y2): (f64, f64) = b.end.into();
        let (x3, y3): (f64, f64) = self.pos.into();
        let (x4, y4): (f64, f64) = (self.pos + self.looking).into();

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
        Ok(s.draw_line(self.pos, self.pos + self.looking)?)
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
            light: Light::new(),
        }
    }
}

impl PixApp for App {
    fn on_start(&mut self, s: &mut State) -> Result<bool> {
        let w = s.width() as i32;
        let h = s.height() as i32;

        self.boundaries.push(Line::new((-1, -1), (w, -1))); // Top
        self.boundaries.push(Line::new((w, -1), (w, h))); // Right
        self.boundaries.push(Line::new((-1, h), (w, h))); // Bottom
        self.boundaries.push(Line::new((-1, -1), (-1, h))); // Left

        for _ in 0..10 {
            let (x1, y1) = (random(w), random(h));
            let (x2, y2) = (random(w), random(h));
            self.boundaries.push(Line::new((x1, y1), (x2, y2)));
        }

        let light_x = s.width() / 2;
        let light_y = s.height() / 2;
        self.light.pos((light_x as f64, light_y as f64));

        Ok(true)
    }

    fn on_update(&mut self, s: &mut State) -> Result<bool> {
        s.background(51);
        self.light.update(&self.boundaries, s);
        for b in self.boundaries.iter_mut() {
            s.stroke(255);
            s.stroke_weight(2);
            b.draw(s)?;
        }
        self.light.draw(s)?;
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
