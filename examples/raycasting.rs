use pix_engine::prelude::*;

struct Boundary {
    start: Point,
    end: Point,
}

impl Boundary {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            start: Point::new((x1, y1)),
            end: Point::new((x2, y2)),
        }
    }
    fn draw(&self, s: &mut State) -> Result<()> {
        s.stroke(255);
        s.stroke_weight(2);
        Ok(s.draw_line(self.start, self.end)?)
    }
}

struct Light {
    pos: Vector,
    color: Color,
    rays: Vec<Ray>,
}

impl Light {
    pub fn new(w: u32, h: u32) -> Self {
        let pos = Vector::new((w as f64, h as f64));
        let mut rays = Vec::with_capacity(360);
        for angle in 0..360 {
            rays.push(Ray::new((angle as f64).to_radians()));
        }
        Self {
            pos,
            color: Color::random_rgba(),
            rays,
        }
    }
    fn draw(&self, s: &mut State) -> Result<()> {
        s.stroke(self.color);
        for ray in self.rays.iter() {
            ray.draw(self.pos, s)?;
        }
        Ok(())
    }
}

struct Ray {
    looking: Vector,
}

impl Ray {
    fn new(angle: f64) -> Self {
        Self {
            looking: Vector::from_angle(angle, 1.0),
        }
    }
    fn draw(&self, pos: Vector, s: &mut State) -> Result<()> {
        s.stroke_weight(1);
        // TODO Switch to using translate
        // s.translate(self.pos.x, self.pos.y);
        // s.draw_line((0, 0), Point::from(self.looking));
        let mut looking = self.looking.copy();
        looking.add(pos);
        Ok(s.draw_line(Point::from(pos), Point::from(looking))?)
    }
    fn look_at(&mut self, pos: Vector, point: Vector) {
        self.looking.x = point.x - pos.x;
        self.looking.y = point.y - pos.y;
    }
    fn cast(&mut self, pos: Vector, boundary: &Boundary) -> Option<Vector> {
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

struct App {
    boundaries: Vec<Boundary>,
    light: Light,
}

impl App {
    fn new() -> Self {
        Self {
            boundaries: Vec::new(),
            light: Light::new(0, 0),
        }
    }
}

impl PixApp for App {
    fn on_start(&mut self, s: &mut State) -> Result<bool> {
        let w = s.width() as i32;
        let h = s.height() as i32;
        self.boundaries.push(Boundary::new(0, -1, w, -1)); // Top
        self.boundaries.push(Boundary::new(w, 0, w, h)); // Right
        self.boundaries.push(Boundary::new(0, h, w, h)); // Bottom
        self.boundaries.push(Boundary::new(-1, 0, -1, h)); // Left

        for _ in 0..10 {
            let x1 = random(w);
            let y1 = random(h);
            let x2 = random(w);
            let y2 = random(h);
            self.boundaries.push(Boundary::new(x1, y1, x2, y2));
        }
        self.light = Light::new(s.width() / 2, s.height() / 2);
        Ok(true)
    }

    fn on_update(&mut self, s: &mut State) -> Result<bool> {
        s.background(51);

        self.light.pos = s.mouse_pos().into();

        for ray in self.light.rays.iter_mut() {
            let mut closest = None;
            let mut closest_dist = constants::INFINITY;
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
                ray.look_at(self.light.pos, point);
            }
        }

        for b in self.boundaries.iter() {
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
