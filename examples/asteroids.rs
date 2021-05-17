use pix_engine::prelude::*;

const TITLE: &str = "Asteroids";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SHIP_SCALE: f64 = 4.0;
const ASTEROID_SIZE: u32 = 64;
const MIN_ASTEROID_SIZE: u32 = 16;
const SHIP_THRUST: f64 = 150.0;
const MAX_ASTEROID_SPEED: f64 = 50.0;
const SHATTERED_ASTEROID_SPEED: f64 = 80.0;
const BULLET_SPEED: f64 = 200.0;
const ASTEROID_SAFE_RADIUS: f64 = 80.0; // So asteroids don't spawn near player

struct Asteroids {
    asteroids: Vec<SpaceObj>,
    bullets: Vec<SpaceObj>,
    ship: SpaceObj,
    level: usize,
    lives: u32,
    score: i32,
    ship_model: Vec<(f64, f64)>,
    asteroid_model: Vec<(f64, f64)>,
    paused: bool,
    gameover: bool,
    origin: Vector,
    max_pos: Vector,
}

#[derive(Default)]
struct SpaceObj {
    size: u32,
    pos: Vector,
    vel: Vector,
    angle: f64,
    destroyed: bool,
}

impl SpaceObj {
    fn new<V>(size: u32, pos: V, vel: V, angle: f64) -> Self
    where
        V: Into<Vector>,
    {
        Self {
            size,
            pos: pos.into(),
            vel: vel.into(),
            angle,
            destroyed: false,
        }
    }
    fn rand_asteroid(ship: &SpaceObj, s: &PixState) -> Self {
        let (ship_x, ship_y) = ship.pos.into();
        let mut pos = vector!(random!(s.width()), random!(s.height()));
        if circle!(ship_x, ship_y, ASTEROID_SAFE_RADIUS).contains(pos) {
            pos -= ship.pos
        }

        let vel = vector!(random!(-MAX_ASTEROID_SPEED, MAX_ASTEROID_SPEED));
        let angle = random!(360.0);
        Self::new(ASTEROID_SIZE, pos, vel, angle)
    }
}

impl Asteroids {
    fn new() -> Self {
        Self {
            asteroids: Vec::new(),
            bullets: Vec::new(),
            ship: SpaceObj::default(),
            level: 1,
            lives: 4,
            score: 0,
            ship_model: Vec::new(),
            asteroid_model: Vec::new(),
            paused: false,
            gameover: false,
            origin: vector!(),
            max_pos: vector!(),
        }
    }

    fn spawn_new_ship(&mut self, s: &PixState) {
        self.ship.pos = vector!(s.width() / 2, s.height() / 2);
        self.ship.vel.set_mag(0.0);
        self.ship.angle = 0.0;

        let asteroid_count = if !self.asteroids.is_empty() {
            std::cmp::min(self.level + 2, self.asteroids.len())
        } else {
            self.level + 2
        };
        self.asteroids.clear();
        self.bullets.clear();
        for _ in 0..asteroid_count {
            self.asteroids.push(SpaceObj::rand_asteroid(&self.ship, s));
        }
    }

    fn exploded(&mut self, s: &PixState) {
        if self.lives > 0 {
            self.lives -= 1;
            self.score -= 500;
            self.spawn_new_ship(s);
        } else {
            self.gameover = true;
        }
    }

    fn reset(&mut self, s: &PixState) {
        self.paused = false;
        self.spawn_new_ship(s);
        self.level = 1;
        self.lives = 4;
        self.score = 0;
        self.gameover = false;
    }
}

impl AppState for Asteroids {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.max_pos = (s.width(), s.height()).into();
        self.ship_model = vec![(5.0, 0.0), (-2.5, -2.5), (-2.5, 2.5)];
        for i in 0..20 {
            let noise = random!(0.8, 1.2);
            let a = (i as f64 / 20.0) * 2.0 * PI;
            let x = noise * a.sin();
            let y = noise * a.cos();
            self.asteroid_model.push((x, y));
        }
        self.spawn_new_ship(s);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear();

        let width = s.width() as i32;
        let height = s.height() as i32;
        if self.paused {
            return Ok(());
        } else if self.gameover {
            let x = width / 2 - 80;
            let y = height / 2 - 24;
            s.fill(WHITE);
            s.text_size(32);
            s.text((x, y), "GAME OVER")?;
            s.text_size(16);
            s.text((x - 100, y + 24), "PRESS SPACE TO RESTART")?;
            return Ok(());
        }

        let elapsed = s.delta_time();

        // Steer
        if s.key_pressed(Key::Left) {
            self.ship.angle -= 5.0 * elapsed;
        } else if s.keys().contains(&Key::Right) {
            self.ship.angle += 5.0 * elapsed;
        }
        // Thrust
        if s.key_pressed(Key::Up) {
            self.ship.vel += Vector::from_angle(self.ship.angle, SHIP_THRUST * elapsed);
        }

        // Draw Level, Lives, & Score
        s.text_size(16);
        s.fill(WHITE);
        s.text(
            (4, 4),
            &format!("LEVEL: {}  SCORE: {}", self.level, self.score),
        )?;

        s.stroke(WHITE);
        for i in 0..self.lives {
            s.wireframe(&self.ship_model, (12 + (i * 14), 36), -HALF_PI, 2.0)?;
        }

        self.ship.pos += self.ship.vel * elapsed;
        self.ship.pos.wrap_2d(s.width() as f64, s.height() as f64);

        // Draw asteroids
        for a in self.asteroids.iter_mut() {
            // Ship collision
            if circle!(a.pos.x, a.pos.y, a.size).contains(self.ship.pos) {
                self.exploded(s);
                return Ok(());
            }

            a.pos += a.vel * elapsed;
            a.pos.wrap_2d(s.width() as f64, s.height() as f64);
            a.angle += 0.5 * elapsed; // Give some twirl
            s.stroke(YELLOW);
            s.wireframe(&self.asteroid_model, a.pos, a.angle, a.size as f64)?;
        }

        let mut new_asteroids = Vec::new();

        // Update bullet and check collisions
        for b in self.bullets.iter_mut() {
            b.pos += b.vel * elapsed;
            b.angle -= 1.0 * elapsed;

            for a in self.asteroids.iter_mut() {
                if circle!(a.pos.x, a.pos.y, a.size).contains(b.pos) {
                    // Asteroid hit
                    b.destroyed = true; // Removes bullet

                    if a.size > MIN_ASTEROID_SIZE {
                        // Break into two
                        let a1 = random!(TWO_PI);
                        let a2 = random!(TWO_PI);
                        new_asteroids.push(SpaceObj::new(
                            a.size >> 1,
                            a.pos,
                            Vector::from_angle(a1, SHATTERED_ASTEROID_SPEED),
                            0.0,
                        ));
                        new_asteroids.push(SpaceObj::new(
                            a.size >> 1,
                            a.pos,
                            Vector::from_angle(a2, SHATTERED_ASTEROID_SPEED),
                            0.0,
                        ));
                    }
                    a.destroyed = true; // Remove asteroid
                    self.score += 100;
                }
            }
        }
        self.asteroids.append(&mut new_asteroids);

        // Remove offscreen/destroyed bullets
        let origin = self.origin;
        let max_pos = self.max_pos;
        self.bullets
            .retain(|b| !b.destroyed && b.pos >= origin && b.pos < max_pos);
        // Remove destroyed asteroids
        self.asteroids.retain(|a| !a.destroyed);

        // Draw bullets
        s.fill(WHITE);
        s.stroke(WHITE);
        for b in self.bullets.iter() {
            s.circle((b.pos, 1))?;
        }

        // Draw ship
        s.wireframe(&self.ship_model, self.ship.pos, self.ship.angle, SHIP_SCALE)?;

        // Win level
        if self.asteroids.is_empty() {
            self.level += 1;
            self.score += 1000;
            self.bullets.clear();
            for _ in 0..(self.level + 2) {
                self.asteroids.push(SpaceObj::rand_asteroid(&self.ship, s));
            }
        }

        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, key: Key) {
        match key {
            Key::Escape => self.paused = !self.paused,
            Key::R => self.reset(s),
            Key::Space if self.gameover => {
                self.reset(s);
            }
            _ => (),
        }
    }

    fn on_key_released(&mut self, _s: &mut PixState, key: Key) {
        match key {
            Key::Space if !self.gameover => {
                self.bullets.push(SpaceObj::new(
                    0,
                    self.ship.pos,
                    Vector::from_angle(self.ship.angle, BULLET_SPEED),
                    0.0,
                ));
            }
            _ => (),
        }
    }
}

pub fn main() {
    let mut engine = PixEngine::create(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .build()
        .expect("valid engine");
    let mut app = Asteroids::new();
    engine.run(&mut app).expect("ran successfully");
}
