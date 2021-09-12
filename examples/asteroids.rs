use lazy_static::lazy_static;
use pix_engine::prelude::*;

const SHIP_SCALE: Scalar = 4.0;
const ASTEROID_SIZE: u32 = 64;
const MIN_ASTEROID_SIZE: u32 = 16;
const SHIP_THRUST: Scalar = 150.0;
const MAX_THRUST: Scalar = 600.0;
const MAX_ASTEROID_SPEED: Scalar = 50.0;
const SHATTERED_ASTEROID_SPEED: Scalar = 80.0;
const BULLET_SPEED: Scalar = 200.0;
const ASTEROID_SAFE_RADIUS: Scalar = 80.0; // So asteroids don't spawn near player

const ORIGIN: Point = point!(0.0, 0.0, 0.0);
const SHIP_MODEL: [Vector; 3] = [
    vector!(5.0, 0.0, 0.0),
    vector!(-2.5, -2.5, 0.0),
    vector!(-2.5, 2.5, 0.0),
];

lazy_static! {
    static ref ASTEROID_MODEL: Vec<Vector> = {
        let mut model = Vec::with_capacity(20);
        for i in 0..20 {
            let noise = random!(0.8, 1.2);
            let a = (i as Scalar / 20.0) * 2.0 * PI;
            model.push(vector!(noise * a.sin(), noise * a.cos()));
        }
        model
    };
}

struct SpaceObj {
    size: u32,
    pos: Point,
    start_pos: Point,
    vel: Vector,
    angle: Scalar,
    destroyed: bool,
}

impl SpaceObj {
    fn new(pos: Point, vel: Vector, size: u32, angle: Scalar) -> Self {
        Self {
            pos,
            start_pos: pos,
            vel,
            size,
            angle,
            destroyed: false,
        }
    }

    fn new_asteroid(ship: &SpaceObj, mut pos: Point) -> Self {
        if Circle::with_position(ship.pos, ASTEROID_SAFE_RADIUS).contains_point(pos) {
            pos -= ship.pos
        }
        let vel = vector!(random!(-MAX_ASTEROID_SPEED, MAX_ASTEROID_SPEED));
        let angle = random!(360.0);
        Self::new(pos, vel, ASTEROID_SIZE, angle)
    }

    fn contains_point(&self, p: Point) -> bool {
        Circle::from(self).contains_point(p)
    }
}

impl From<SpaceObj> for Circle {
    fn from(obj: SpaceObj) -> Self {
        Self::with_position(obj.pos, obj.size as Scalar)
    }
}

impl From<&SpaceObj> for Circle {
    fn from(obj: &SpaceObj) -> Self {
        Self::with_position(obj.pos, obj.size as Scalar)
    }
}

struct Asteroids {
    asteroids: Vec<SpaceObj>,
    broken_asteroids: Vec<SpaceObj>,
    bullets: Vec<SpaceObj>,
    ship: SpaceObj,
    level: usize,
    lives: i32,
    score: i32,
    paused: bool,
    gameover: bool,
    width: u32,
    height: u32,
}

impl Asteroids {
    fn new(width: u32, height: u32) -> Self {
        Self {
            asteroids: Vec::new(),
            broken_asteroids: Vec::new(),
            bullets: Vec::new(),
            ship: SpaceObj::new(
                point!(width as Scalar / 2.0, height as Scalar / 2.0),
                vector!(),
                4,
                0.0,
            ),
            level: 1,
            lives: 4,
            score: 0,
            paused: false,
            gameover: false,
            width,
            height,
        }
    }

    fn spawn_new_ship(&mut self) {
        self.ship.pos = self.ship.start_pos;
        self.ship.vel.set_mag(0.0);
        self.ship.angle = 0.0;
        self.bullets.clear();
    }

    fn spawn_asteroids(&mut self) {
        self.asteroids.clear();
        let asteroid_count = if !self.asteroids.is_empty() {
            std::cmp::min(self.level + 2, self.asteroids.len())
        } else {
            self.level + 2
        };
        let (w, h) = (self.width as Scalar, self.height as Scalar);
        for _ in 0..asteroid_count {
            let pos = point!(random!(w), random!(h));
            self.asteroids.push(SpaceObj::new_asteroid(&self.ship, pos));
        }
    }

    fn exploded(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
            self.score -= 500;
            self.spawn_new_ship();
            self.spawn_asteroids();
        } else {
            self.gameover = true;
        }
    }

    fn reset(&mut self) {
        self.paused = false;
        self.level = 1;
        self.lives = 4;
        self.score = 0;
        self.gameover = false;
        self.spawn_new_ship();
        self.spawn_asteroids();
    }

    fn handle_controls(&mut self, s: &mut PixState) {
        let elapsed = s.delta_time();
        // Steer
        if s.key_down(Key::Left) {
            self.ship.angle -= 5.0 * elapsed;
        } else if s.keys().contains(&Key::Right) {
            self.ship.angle += 5.0 * elapsed;
        }
        // Thrust
        if s.key_down(Key::Up) {
            self.ship.vel += Vector::from_angle(self.ship.angle, SHIP_THRUST * elapsed);
            self.ship.vel.limit(MAX_THRUST);
        }
    }

    fn draw_asteroids(&mut self, s: &mut PixState) -> PixResult<()> {
        let (w, h) = (self.width as Scalar, self.height as Scalar);
        let elapsed = s.delta_time();
        // Draw asteroids
        for a in self.asteroids.iter_mut() {
            // Ship collision
            if a.contains_point(self.ship.pos) {
                self.exploded();
                return Ok(());
            }

            a.pos += a.vel * elapsed;
            a.pos.wrap_2d(w, h, a.size as Scalar);
            a.angle += 0.5 * elapsed; // Give some twirl
            s.fill(BLACK);
            s.stroke(YELLOW);
            s.wireframe(&ASTEROID_MODEL, a.pos, a.angle, a.size as Scalar)?;
        }
        Ok(())
    }

    fn draw_bullets(&mut self, s: &mut PixState) -> PixResult<()> {
        let (w, h) = (self.width as Scalar, self.height as Scalar);
        let elapsed = s.delta_time();
        // Update bullet and check collisions
        for b in self.bullets.iter_mut() {
            b.pos += b.vel * elapsed;
            b.angle -= 1.0 * elapsed;

            for a in self.asteroids.iter_mut() {
                if a.contains_point(b.pos) {
                    // Asteroid hit
                    b.destroyed = true; // Removes bullet

                    if a.size > MIN_ASTEROID_SIZE {
                        // Break into two
                        let a1 = random!(TAU);
                        let a2 = random!(TAU);
                        let speed = random!(0.0, SHATTERED_ASTEROID_SPEED);
                        self.broken_asteroids.push(SpaceObj::new(
                            a.pos,
                            Vector::from_angle(a1, speed),
                            a.size >> 1,
                            a1,
                        ));
                        self.broken_asteroids.push(SpaceObj::new(
                            a.pos,
                            Vector::from_angle(a2, speed),
                            a.size >> 1,
                            a2,
                        ));
                    }
                    a.destroyed = true; // Remove asteroid
                    self.score += 100;
                }
            }
        }
        self.asteroids.append(&mut self.broken_asteroids);
        // Remove offscreen/destroyed bullets
        self.bullets
            .retain(|b| !b.destroyed && b.pos >= ORIGIN && b.pos < point!(w, h));
        // Remove destroyed asteroids
        self.asteroids.retain(|a| !a.destroyed);

        // Draw bullets
        s.fill(BLACK);
        s.stroke(WHITE);
        for b in self.bullets.iter() {
            s.circle(b)?;
        }

        Ok(())
    }

    fn draw_ship(&mut self, s: &mut PixState) -> PixResult<()> {
        let (w, h) = (self.width as Scalar, self.height as Scalar);
        let elapsed = s.delta_time();
        self.ship.pos += self.ship.vel * elapsed;
        self.ship.pos.wrap_2d(w, h, self.ship.size as Scalar);
        s.wireframe(&SHIP_MODEL, self.ship.pos, self.ship.angle, SHIP_SCALE)
    }

    fn draw_gameover(&mut self, s: &mut PixState) -> PixResult<()> {
        let x = self.width as i32 / 2 - 150;
        let y = self.height as i32 / 2 - 100;
        s.fill(WHITE);
        s.font_size(32)?;
        s.text([x, y], "GAME OVER")?;
        s.font_size(16)?;
        s.text([x - 30, y + 50], "PRESS SPACE TO RESTART")?;
        Ok(())
    }

    fn draw_score(&mut self, s: &mut PixState) -> PixResult<()> {
        // Draw Level, Lives, & Score
        s.font_size(16)?;
        s.fill(WHITE);
        s.text(
            [4, 4],
            &format!("LEVEL: {}  SCORE: {}", self.level, self.score),
        )?;

        s.fill(BLACK);
        s.stroke(WHITE);
        for i in 0..self.lives {
            s.wireframe(
                &SHIP_MODEL,
                [12.0 + (i as Scalar * 14.0), 36.0],
                -FRAC_PI_2,
                2.0,
            )?;
        }

        // Check win condition
        if self.asteroids.is_empty() {
            let (w, h) = (self.width as Scalar, self.height as Scalar);
            self.level += 1;
            self.score += 1000;
            self.bullets.clear();
            for _ in 0..(self.level + 2) {
                let pos = point!(random!(w), random!(h));
                self.asteroids.push(SpaceObj::new_asteroid(&self.ship, pos));
            }
        }
        Ok(())
    }
}

impl AppState for Asteroids {
    fn on_start(&mut self, _s: &mut PixState) -> PixResult<()> {
        self.spawn_new_ship();
        self.spawn_asteroids();
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        if self.gameover {
            return self.draw_gameover(s);
        }
        self.handle_controls(s);
        self.draw_asteroids(s)?;
        self.draw_bullets(s)?;
        self.draw_ship(s)?;
        self.draw_score(s)?;
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        match event.key {
            Key::Escape => {
                if s.running() {
                    s.no_run()
                } else {
                    s.run()
                }
            }
            Key::R => self.reset(),
            Key::Space if self.gameover => self.reset(),
            _ => (),
        }
        Ok(())
    }

    fn on_key_released(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        match event.key {
            Key::Space if !self.gameover => {
                self.bullets.push(SpaceObj::new(
                    self.ship.pos,
                    Vector::from_angle(self.ship.angle, self.ship.vel.mag() + BULLET_SPEED),
                    1,
                    0.0,
                ));
            }
            _ => (),
        }
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let width = 800;
    let height = 600;
    let mut engine = PixEngine::builder()
        .with_dimensions(width, height)
        .with_title("Asteroids")
        .with_frame_rate()
        .position_centered()
        .build();
    let mut app = Asteroids::new(width, height);
    engine.run(&mut app)
}
