use pix_engine::{
    math::{FRAC_PI_2, PI, TAU},
    prelude::*,
};

const SHIP_SCALE: f64 = 4.0;
const ASTEROID_SIZE: u32 = 64;
const MIN_ASTEROID_SIZE: u32 = 16;
const SHIP_THRUST: f64 = 150.0;
const MAX_THRUST: f64 = 600.0;
const MAX_ASTEROID_SPEED: f64 = 50.0;
const SHATTERED_ASTEROID_SPEED: f64 = 100.0;
const BULLET_SPEED: f64 = 200.0;
const ASTEROID_SAFE_RADIUS: f64 = 80.0; // So asteroids don't spawn near player

const ORIGIN: Point<f64> = point!(0.0, 0.0);
const SHIP_MODEL: [Point<f64>; 3] = [point!(5.0, 0.0), point!(-2.5, -2.5), point!(-2.5, 2.5)];

struct SpaceObj {
    size: u32,
    pos: Point<f64>,
    start_pos: Point<f64>,
    vel: Vector<f64>,
    angle: f64,
    destroyed: bool,
}

impl SpaceObj {
    fn new(pos: Point<f64>, vel: Vector<f64>, size: u32, angle: f64) -> Self {
        Self {
            pos,
            start_pos: pos,
            vel,
            size,
            angle,
            destroyed: false,
        }
    }

    fn new_asteroid(ship: &SpaceObj, mut pos: Point<f64>) -> Self {
        if Ellipse::circle_with_position(ship.pos, ASTEROID_SAFE_RADIUS).contains(pos) {
            pos.offset(-ship.pos)
        }
        let mut vel = Vector::random();
        vel.set_mag(MAX_ASTEROID_SPEED);
        let angle = random!(360.0);
        Self::new(pos, vel, ASTEROID_SIZE, angle)
    }

    fn contains(&self, p: Point<f64>) -> bool {
        Ellipse::from(self).contains(p.round().as_::<i32>())
    }
}

impl From<SpaceObj> for Ellipse {
    fn from(obj: SpaceObj) -> Self {
        Self::circle_with_position(obj.pos.round().as_::<i32>(), obj.size as i32)
    }
}

impl From<&SpaceObj> for Ellipse {
    fn from(obj: &SpaceObj) -> Self {
        Self::circle_with_position(obj.pos.round().as_::<i32>(), obj.size as i32)
    }
}

struct Asteroids {
    asteroid_model: [Point<f64>; 20],
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
        let mut asteroid_model = [Point::default(); 20];
        for (i, p) in asteroid_model.iter_mut().enumerate() {
            let noise = random!(0.8, 1.2);
            let a = (i as f64 / 20.0) * 2.0 * PI;
            *p = point!(noise * a.sin(), noise * a.cos());
        }
        Self {
            asteroid_model,
            asteroids: Vec::new(),
            broken_asteroids: Vec::new(),
            bullets: Vec::new(),
            ship: SpaceObj::new(
                point!(width as f64 / 2.0, height as f64 / 2.0),
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
        let (w, h) = (self.width as f64, self.height as f64);
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
        let elapsed = s.delta_time().as_secs_f64();
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

    fn draw_asteroids(&mut self, s: &mut PixState) -> Result<()> {
        let (w, h) = (self.width as f64, self.height as f64);
        let elapsed = s.delta_time().as_secs_f64();
        // Draw asteroids
        for a in self.asteroids.iter_mut() {
            // Ship collision
            if a.contains(self.ship.pos) {
                self.exploded();
                return Ok(());
            }

            a.pos += a.vel * elapsed;
            a.pos.wrap([w, h], a.size as f64);
            a.angle += 0.5 * elapsed; // Give some twirl
            s.fill(Color::BLACK);
            s.stroke(Color::YELLOW);
            s.wireframe(
                self.asteroid_model,
                a.pos.round().as_::<i32>(),
                a.angle,
                a.size as f64,
            )?;
        }
        Ok(())
    }

    fn draw_bullets(&mut self, s: &mut PixState) -> Result<()> {
        let (w, h) = (self.width as f64, self.height as f64);
        let elapsed = s.delta_time().as_secs_f64();
        // Update bullet and check collisions
        for b in self.bullets.iter_mut() {
            b.pos += b.vel * elapsed;
            b.angle -= 1.0 * elapsed;

            for a in self.asteroids.iter_mut() {
                if a.contains(b.pos) {
                    // Asteroid hit
                    b.destroyed = true; // Removes bullet

                    if a.size > MIN_ASTEROID_SIZE {
                        // Break into two
                        let a1 = random!(TAU);
                        let a2 = random!(TAU);
                        let speed = random!(MAX_ASTEROID_SPEED, SHATTERED_ASTEROID_SPEED);
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
        s.fill(Color::WHITE);
        s.stroke(None);
        for b in self.bullets.iter() {
            s.circle(Ellipse::from(b))?;
        }

        Ok(())
    }

    fn draw_ship(&mut self, s: &mut PixState) -> Result<()> {
        let (w, h) = (self.width as f64, self.height as f64);
        let elapsed = s.delta_time().as_secs_f64();
        self.ship.pos += self.ship.vel * elapsed;
        self.ship.pos.wrap([w, h], self.ship.size as f64);
        s.fill(Color::BLACK);
        s.stroke(Color::WHITE);
        s.wireframe(
            SHIP_MODEL,
            self.ship.pos.round().as_::<i32>(),
            self.ship.angle,
            SHIP_SCALE,
        )
    }

    fn draw_gameover(&mut self, s: &mut PixState) -> Result<()> {
        let x = self.width as i32 / 2 - 8;
        let y = self.height as i32 / 2 - 100;
        s.fill(Color::WHITE);
        s.stroke(None);
        s.font_size(32)?;
        s.rect_mode(RectMode::Center);
        s.set_cursor_pos([x, y]);
        s.text("GAME OVER")?;
        s.set_cursor_pos([x, y + 50]);
        s.font_size(16)?;
        s.text("PRESS SPACE TO RESTART")?;
        Ok(())
    }

    fn draw_score(&mut self, s: &mut PixState) -> Result<()> {
        // Draw Level, Lives, & Score
        s.font_size(16)?;
        s.fill(Color::WHITE);
        s.stroke(None);
        s.text(format!("LEVEL: {}  SCORE: {}", self.level, self.score))?;

        s.fill(Color::BLACK);
        s.stroke(Color::WHITE);
        for i in 0..self.lives {
            s.wireframe(SHIP_MODEL, [12 + (i * 14), 40], -FRAC_PI_2, 2.0)?;
        }

        // Check win condition
        if self.asteroids.is_empty() {
            let (w, h) = (self.width as f64, self.height as f64);
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

impl PixEngine for Asteroids {
    fn on_start(&mut self, s: &mut PixState) -> Result<()> {
        s.background(Color::BLACK);
        self.spawn_new_ship();
        self.spawn_asteroids();
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> Result<()> {
        s.clear()?;
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

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
        match event.key {
            Key::Escape => {
                let running = s.running();
                s.run(!running);
            }
            Key::R => self.reset(),
            Key::Space if self.gameover => self.reset(),
            _ => (),
        }
        Ok(false)
    }

    fn on_key_released(&mut self, _s: &mut PixState, event: KeyEvent) -> Result<bool> {
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
        Ok(false)
    }
}

pub fn main() -> Result<()> {
    let width = 800;
    let height = 600;
    let mut engine = Engine::builder()
        .with_dimensions(width, height)
        .with_title("Asteroids")
        .with_frame_rate()
        .build()?;
    let mut app = Asteroids::new(width, height);
    engine.run(&mut app)
}
