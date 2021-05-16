use pix_engine::prelude::*;

const TITLE: &str = "Flocking Simulation";
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;

const BIRD_COUNT: usize = 100;
const BIRD_MODEL: [(f64, f64); 13] = [
    (1.5, 0.0),
    (0.75, -0.25),
    (0.25, -1.5),
    (-1.0, -2.25),
    (-0.25, -1.5),
    (-0.45, -0.35),
    (-2.0, -0.2),
    (-2.0, 0.2),
    (-0.45, 0.35),
    (-0.25, 1.5),
    (-1.0, 2.25),
    (0.25, 1.5),
    (0.75, 0.25),
];
const BIRD_SCALE: f64 = 8.0;

#[derive(PartialEq)]
struct Boid {
    pos: Vector,
    vel: Vector,
    acc: Vector,
    percep_radius: f64,
    max_acc: f64,
    max_vel: f64,
}

impl Boid {
    fn new() -> Self {
        let mut vel = Vector::random_2d();
        vel.set_mag(random!(2.0, 4.0));
        Self {
            pos: vector!(random!(WIDTH), random!(HEIGHT)),
            vel,
            acc: vector!(),
            percep_radius: random!(25.0, 50.0),
            max_acc: random!(0.2, 0.4),
            max_vel: random!(3.0, 6.0),
        }
    }

    fn update(&mut self) {
        self.pos += self.vel;
        self.pos.wrap_2d(WIDTH as f64, HEIGHT as f64);
        self.vel += self.acc;
        if self.vel.mag() < 2.0 {
            self.vel.set_mag(2.0);
        }
        self.vel.limit(self.max_vel);
    }

    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.stroke(SKY_BLUE);
        s.wireframe(&BIRD_MODEL, self.pos, self.vel.heading(), BIRD_SCALE)?;
        Ok(())
    }
}

struct BoidAdjustment {
    alignment: Vector,
    cohesion: Vector,
    separation: Vector,
    total: f64,
}

impl BoidAdjustment {
    fn new() -> Self {
        Self {
            alignment: vector!(),
            cohesion: vector!(),
            separation: vector!(),
            total: 0.0,
        }
    }
}

struct App {
    flock: Vec<Boid>,
}

impl App {
    fn new() -> Self {
        let mut flock = Vec::with_capacity(BIRD_COUNT);
        for _ in 0..BIRD_COUNT {
            flock.push(Boid::new());
        }
        Self { flock }
    }

    fn reset(&mut self) {
        self.flock.clear();
        for _ in 0..BIRD_COUNT {
            self.flock.push(Boid::new());
        }
    }

    fn get_adjustment(&self) -> Vec<Option<Vector>> {
        self.flock
            .iter()
            .map(|boid| {
                let mut adjustment =
                    self.flock
                        .iter()
                        .fold(BoidAdjustment::new(), |mut adjustment, other| {
                            let d = boid.pos.dist(other.pos);
                            if boid != other && d < boid.percep_radius {
                                adjustment.alignment += other.vel;
                                adjustment.cohesion += other.pos;
                                adjustment.separation += boid.pos - other.pos;
                                if d > 0.0 {
                                    adjustment.separation /= d;
                                }
                                adjustment.total += 1.0;
                            }
                            adjustment
                        });
                if adjustment.total > 0.0 {
                    if adjustment.total.is_infinite() || adjustment.total.is_nan() {
                        eprintln!("{}", adjustment.total);
                    }
                    adjustment.alignment /= adjustment.total;
                    adjustment.alignment.set_mag(boid.max_vel);
                    adjustment.alignment -= boid.vel;
                    adjustment.alignment.limit(boid.max_acc);

                    adjustment.cohesion /= adjustment.total;
                    adjustment.cohesion -= boid.pos;
                    adjustment.cohesion.set_mag(boid.max_vel);
                    adjustment.cohesion -= boid.vel;
                    adjustment.cohesion.limit(boid.max_acc);

                    adjustment.separation /= adjustment.total;
                    adjustment.separation.set_mag(boid.max_vel);
                    adjustment.separation -= boid.vel;
                    adjustment.separation.limit(boid.max_acc * 1.5);
                    Some(adjustment.alignment + adjustment.cohesion + adjustment.separation)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl AppState for App {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(51);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear();
        let adjustment = self.get_adjustment();
        for (i, boid) in self.flock.iter_mut().enumerate() {
            boid.acc.set_mag(0.0);
            if let Some(adjustment) = adjustment[i] {
                boid.acc += adjustment;
            }
            boid.update();
            boid.draw(s)?;
        }
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, key: Keycode) {
        if key == Keycode::R {
            self.reset();
        }
    }
}

pub fn main() {
    let mut engine = PixEngine::create(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .build()
        .expect("valid engine");

    let mut app = App::new();

    engine.run(&mut app).expect("ran successfully");
}
