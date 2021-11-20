use pix_engine::{prelude::*, shape::PointF2, vector::VectorF2};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;

const BOID_COUNT: usize = 500;
const BOID_MODEL: [PointF2; 13] = [
    point!(1.5, 0.0),
    point!(0.75, -0.25),
    point!(0.25, -1.5),
    point!(-1.0, -2.25),
    point!(-0.25, -1.5),
    point!(-0.45, -0.35),
    point!(-2.0, -0.2),
    point!(-2.0, 0.2),
    point!(-0.45, 0.35),
    point!(-0.25, 1.5),
    point!(-1.0, 2.25),
    point!(0.25, 1.5),
    point!(0.75, 0.25),
];
const BOID_SIZE: Scalar = 3.0;

#[derive(PartialEq)]
struct Boid {
    pos: PointF2,
    vel: VectorF2,
    acc: VectorF2,
    max_acc: Scalar,
    max_vel: Scalar,
}

impl Boid {
    fn new() -> Self {
        Self {
            pos: point!(random!(WIDTH as Scalar), random!(HEIGHT as Scalar)),
            vel: vector!(random!(-1.0, 1.0), random!(-1.0, 1.0)),
            acc: vector!(),
            max_acc: 0.1,
            max_vel: 3.0,
        }
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.vel.limit(self.max_vel);
        if self.vel.mag() < 2.0 {
            self.vel.set_mag(2.0);
        }
        self.pos += self.vel;
        self.pos
            .wrap([WIDTH as Scalar, HEIGHT as Scalar], BOID_SIZE);
        self.acc *= 0.0;
    }

    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.stroke(Color::SKY_BLUE);
        s.fill(Color::SKY_BLUE);
        s.wireframe(
            BOID_MODEL,
            self.pos.round().as_::<i32>(),
            self.vel.heading(),
            BOID_SIZE,
        )?;
        Ok(())
    }
}

struct BoidAdjustment {
    align: Vec<VectorF2>,
    cohesion: Vec<VectorF2>,
    sep: Vec<VectorF2>,
}

impl BoidAdjustment {
    fn new() -> Self {
        Self {
            align: Vec::with_capacity(BOID_COUNT),
            cohesion: Vec::with_capacity(BOID_COUNT),
            sep: Vec::with_capacity(BOID_COUNT),
        }
    }
}

struct App {
    flock: Vec<Boid>,
}

impl App {
    fn new() -> Self {
        let mut flock = Vec::with_capacity(BOID_COUNT);
        for _ in 0..BOID_COUNT {
            flock.push(Boid::new());
        }
        Self { flock }
    }

    fn reset(&mut self) {
        self.flock.clear();
        for _ in 0..BOID_COUNT {
            self.flock.push(Boid::new());
        }
    }

    fn get_adjustment(&self) -> Vec<VectorF2> {
        let align_dist = 50.0;
        let cohesion_dist = 50.0;
        let sep_dist = 25.0;
        self.flock
            .iter()
            .map(|boid| {
                let adj = self
                    .flock
                    .iter()
                    .fold(BoidAdjustment::new(), |mut adj, other| {
                        let d = boid.pos.dist(other.pos);
                        if d > 0.0 {
                            if d < align_dist {
                                adj.align.push(other.vel);
                            }
                            if d < cohesion_dist {
                                adj.cohesion.push(other.pos.into());
                            }
                            if d < sep_dist {
                                let mut sep = boid.pos - other.pos;
                                sep.normalize();
                                sep /= d;
                                adj.sep.push(sep);
                            }
                        }
                        adj
                    });

                let mut sum = vector!();

                if !adj.sep.is_empty() {
                    let mut sep = adj.sep.iter().sum::<VectorF2>() / adj.sep.len() as Scalar;
                    if sep.mag_sq() > 0.0 {
                        sep.normalize();
                        sep *= boid.max_vel;
                        sep -= boid.vel;
                        sep.limit(boid.max_acc);
                        sum += sep * 1.5;
                    }
                }

                if !adj.align.is_empty() {
                    let mut align = adj.align.iter().sum::<VectorF2>() / adj.align.len() as Scalar;
                    align.normalize();
                    align *= boid.max_vel;
                    align -= boid.vel;
                    align.limit(boid.max_acc);
                    sum += align;
                }

                if !adj.cohesion.is_empty() {
                    let mut cohesion =
                        adj.cohesion.iter().sum::<VectorF2>() / adj.cohesion.len() as Scalar;
                    cohesion -= Vector::from(boid.pos);
                    cohesion.normalize();
                    cohesion *= boid.max_vel;
                    cohesion -= boid.vel;
                    cohesion.limit(boid.max_acc);
                    sum += cohesion;
                }

                sum
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
        let adjustment = self.get_adjustment();
        for (i, boid) in self.flock.iter_mut().enumerate() {
            boid.acc += adjustment[i];
            boid.update();
            boid.draw(s)?;
        }
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        if event.key == Key::R {
            self.reset();
        }
        Ok(false)
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Flocking")
        .with_frame_rate()
        .vsync_enabled()
        .build()?;
    let mut app = App::new();
    engine.run(&mut app)
}
