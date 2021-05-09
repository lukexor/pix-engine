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
}

#[derive(Default)]
struct SpaceObj {
    size: u32,
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    angle: f64,
    destroyed: bool,
}

impl SpaceObj {
    fn new(size: u32, x: f64, y: f64, dx: f64, dy: f64, angle: f64) -> Self {
        Self {
            size,
            x,
            y,
            dx,
            dy,
            angle,
            destroyed: false,
        }
    }
    fn rand_asteroid(ship: &SpaceObj, s: &PixState) -> Self {
        let mut x = random!(s.width() as f64);
        if x > (ship.x - ASTEROID_SAFE_RADIUS) && x < (ship.x + ASTEROID_SAFE_RADIUS) {
            let diff = ASTEROID_SAFE_RADIUS - (ship.x - x).abs();
            if ship.x > x {
                x -= diff;
            } else {
                x += diff;
            }
        }
        let mut y = random!(s.height() as f64);
        if y > (ship.y - ASTEROID_SAFE_RADIUS) && y < (ship.y + ASTEROID_SAFE_RADIUS) {
            let diff = ASTEROID_SAFE_RADIUS - (ship.y - y).abs();
            if ship.y > y {
                y -= diff;
            } else {
                y += diff;
            }
        }

        Self {
            size: ASTEROID_SIZE,
            x,
            y,
            dx: random!(-0.5, 0.5) * 2.0 * MAX_ASTEROID_SPEED,
            dy: random!(-0.5, 0.5) * 2.0 * MAX_ASTEROID_SPEED,
            angle: random!(360.0),
            destroyed: false,
        }
    }

    fn wrap_coords(s: &PixState, x: f64, y: f64) -> (f64, f64) {
        let width = s.width() as f64;
        let height = s.height() as f64;
        let ox = if x < 0.0 {
            x + width
        } else if x >= width {
            x - width
        } else {
            x
        };
        let oy = if y < 0.0 {
            y + height
        } else if y >= height {
            y - height
        } else {
            y
        };
        (ox, oy)
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
        }
    }

    fn spawn_new_ship(&mut self, s: &PixState) {
        self.ship.x = (s.width() / 2) as f64;
        self.ship.y = (s.height() / 2) as f64;
        self.ship.dx = 0.0;
        self.ship.dy = 0.0;
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
        s.show_frame_rate(true);

        self.ship_model = vec![(0.0, -5.0), (-2.5, 2.5), (2.5, 2.5)];
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
            s.text("GAME OVER", x, y)?;
            s.text_size(16);
            s.text("PRESS SPACE TO RESTART", x - 100, y + 24)?;
            return Ok(());
        }

        let elapsed = s.delta_time();

        // Steer
        if s.key_pressed(Keycode::Left) {
            self.ship.angle -= 5.0 * elapsed;
        } else if s.keys().contains(&Keycode::Right) {
            self.ship.angle += 5.0 * elapsed;
        }
        // Thrust
        if s.key_pressed(Keycode::Up) {
            self.ship.dx += self.ship.angle.sin() * SHIP_THRUST * elapsed;
            self.ship.dy += -self.ship.angle.cos() * SHIP_THRUST * elapsed;
        }

        // Draw Level, Lives, & Score
        s.text_size(16);
        s.fill(WHITE);
        s.text(
            &format!("LEVEL: {}  SCORE: {}", self.level, self.score),
            4,
            4,
        )?;

        s.stroke(WHITE);
        for i in 0..self.lives {
            s.wireframe(&self.ship_model, 12 + (i as i32 * 14), 36, 0.0, 2.0)?;
        }

        self.ship.x += self.ship.dx * elapsed;
        self.ship.y += self.ship.dy * elapsed;

        // Keep ship in game space
        let ship_pos = SpaceObj::wrap_coords(s, self.ship.x, self.ship.y);
        self.ship.x = ship_pos.0;
        self.ship.y = ship_pos.1;

        // Draw asteroids
        for a in self.asteroids.iter_mut() {
            // Ship collision
            if Circle::new(a.x as i32, a.y as i32, a.size)
                .contains(self.ship.x as i32, self.ship.y as i32)
            {
                self.exploded(s);
                return Ok(());
            }

            a.x += a.dx * elapsed;
            a.y += a.dy * elapsed;
            a.angle += 0.5 * elapsed; // Give some twirl
            let a_pos = SpaceObj::wrap_coords(s, a.x, a.y);
            a.x = a_pos.0;
            a.y = a_pos.1;
            s.stroke(YELLOW);
            s.wireframe(
                &self.asteroid_model,
                a.x as i32,
                a.y as i32,
                a.angle,
                a.size as f64,
            )?;
        }

        let mut new_asteroids = Vec::new();

        // Update bullet and check collisions
        for b in self.bullets.iter_mut() {
            b.x += b.dx * elapsed;
            b.y += b.dy * elapsed;
            b.angle -= 1.0 * elapsed;

            for a in self.asteroids.iter_mut() {
                if Circle::new(a.x as i32, a.y as i32, a.size).contains(b.x as i32, b.y as i32) {
                    // Asteroid hit
                    b.destroyed = true; // Removes bullet

                    if a.size > MIN_ASTEROID_SIZE {
                        // Break into two
                        let a1 = random!(TWO_PI);
                        let a2 = random!(TWO_PI);
                        new_asteroids.push(SpaceObj::new(
                            a.size >> 1,
                            a.x,
                            a.y,
                            SHATTERED_ASTEROID_SPEED * a1.sin(),
                            SHATTERED_ASTEROID_SPEED * a1.cos(),
                            0.0,
                        ));
                        new_asteroids.push(SpaceObj::new(
                            a.size >> 1,
                            a.x,
                            a.y,
                            SHATTERED_ASTEROID_SPEED * a2.sin(),
                            SHATTERED_ASTEROID_SPEED * a2.cos(),
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
        self.bullets.retain(|b| {
            !b.destroyed && b.x >= 1.0 && b.x < width as f64 && b.y >= 1.0 && b.y < height as f64
        });
        // Remove destroyed asteroids
        self.asteroids.retain(|a| !a.destroyed);

        // Draw bullets
        s.fill(WHITE);
        s.stroke(WHITE);
        for b in self.bullets.iter() {
            s.circle(b.x as i32, b.y as i32, 1)?;
        }

        // Draw ship
        s.wireframe(
            &self.ship_model,
            self.ship.x as i32,
            self.ship.y as i32,
            self.ship.angle,
            SHIP_SCALE,
        )?;

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

    fn on_key_pressed(&mut self, s: &mut PixState, key: Keycode) {
        match key {
            Keycode::Escape => self.paused = !self.paused,
            Keycode::R => self.reset(s),
            Keycode::Space if self.gameover => {
                self.reset(s);
            }
            _ => (),
        }
    }

    fn on_key_released(&mut self, _s: &mut PixState, key: Keycode) {
        match key {
            Keycode::Space if !self.gameover => {
                self.bullets.push(SpaceObj::new(
                    0,
                    self.ship.x,
                    self.ship.y,
                    BULLET_SPEED * self.ship.angle.sin(),
                    BULLET_SPEED * -self.ship.angle.cos(),
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
        .position_centered()
        .build()
        .expect("valid engine");
    let mut app = Asteroids::new();
    engine.run(&mut app).expect("ran successfully");
}
