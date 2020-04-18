// use pix_engine::prelude::*;

// const SHIP_SCALE: f64 = 4.0;
// const ASTEROID_SIZE: u32 = 64;
// const MIN_ASTEROID_SIZE: u32 = 16;
// const SHIP_THRUST: f64 = 150.0;
// const MAX_ASTEROID_SPEED: f64 = 50.0;
// const SHATTERED_ASTEROID_SPEED: f64 = 80.0;
// const BULLET_SPEED: f64 = 200.0;
// const ASTEROID_SAFE_RADIUS: f64 = 80.0; // So asteroids don't spawn near player

// struct App {
//     asteroids: Vec<SpaceObj>,
//     bullets: Vec<SpaceObj>,
//     ship: SpaceObj,
//     level: u32,
//     lives: u32,
//     score: i32,
//     exploded: bool,
//     ship_model: Vec<(f64, f64)>,
//     asteroid_model: Vec<(f64, f64)>,
//     paused: bool,
// }

// #[derive(Default)]
// struct SpaceObj {
//     size: u32,
//     x: f64,
//     y: f64,
//     dx: f64,
//     dy: f64,
//     angle: f64,
//     destroyed: bool,
// }

// impl SpaceObj {
//     fn new(size: u32, x: f64, y: f64, dx: f64, dy: f64, angle: f64) -> Self {
//         Self {
//             size,
//             x,
//             y,
//             dx,
//             dy,
//             angle,
//             destroyed: false,
//         }
//     }
//     fn rand_asteroid(ship: &SpaceObj, s: &State) -> Self {
//         let mut x = rand::random::<f64>() * s.width() as f64;
//         if x > (ship.x - ASTEROID_SAFE_RADIUS) && x < (ship.x + ASTEROID_SAFE_RADIUS) {
//             let diff = ASTEROID_SAFE_RADIUS - (ship.x - x).abs();
//             if ship.x > x {
//                 x -= diff;
//             } else {
//                 x += diff;
//             }
//         }
//         let mut y = rand::random::<f64>() * s.height() as f64;
//         if y > (ship.y - ASTEROID_SAFE_RADIUS) && y < (ship.y + ASTEROID_SAFE_RADIUS) {
//             let diff = ASTEROID_SAFE_RADIUS - (ship.y - y).abs();
//             if ship.y > y {
//                 y -= diff;
//             } else {
//                 y += diff;
//             }
//         }

//         Self {
//             size: ASTEROID_SIZE,
//             x,
//             y,
//             dx: (rand::random::<f64>() - 0.5) * 2.0 * MAX_ASTEROID_SPEED,
//             dy: (rand::random::<f64>() - 0.5) * 2.0 * MAX_ASTEROID_SPEED,
//             angle: rand::random::<f64>() * 360.0,
//             destroyed: false,
//         }
//     }
// }

// impl App {
//     fn new() -> Self {
//         Self {
//             asteroids: Vec::new(),
//             bullets: Vec::new(),
//             ship: SpaceObj::default(),
//             level: 1,
//             lives: 4,
//             score: 0,
//             exploded: false,
//             ship_model: Vec::new(),
//             asteroid_model: Vec::new(),
//             paused: false,
//         }
//     }

//     fn spawn_new_ship(&mut self, s: &State) {
//         self.ship.x = s.width() as f64 / 2.0;
//         self.ship.y = s.height() as f64 / 2.0;
//         self.ship.dx = 0.0;
//         self.ship.dy = 0.0;
//         self.ship.angle = 0.0;

//         let asteroid_count = if !self.asteroids.is_empty() {
//             std::cmp::min(self.level + 2, self.asteroids.len() as u32)
//         } else {
//             self.level + 2
//         };
//         self.asteroids.clear();
//         self.bullets.clear();
//         for _ in 0..asteroid_count {
//             self.asteroids.push(SpaceObj::rand_asteroid(&self.ship, s));
//         }
//     }

//     fn exploded(&mut self, s: &State) {
//         self.lives -= 1;
//         self.score -= 500;
//         self.exploded = false;
//         self.spawn_new_ship(s);
//     }

//     fn reset(&mut self, s: &State) {
//         self.paused = false;
//         self.spawn_new_ship(s);
//         self.level = 1;
//         self.lives = 4;
//         self.score = 0;
//         self.exploded = false;
//     }
// }

// impl PixApp for App {
//     fn on_start(&mut self, s: &mut State) -> Result<bool> {
//         s.enable_coord_wrapping(true);
//         self.ship_model = vec![(0.0, -5.0), (-2.5, 2.5), (2.5, 2.5)];
//         for i in 0..20 {
//             let noise = rand::random::<f64>() * 0.4 + 0.8;
//             let a = (i as f64 / 20.0) * 2.0 * PI;
//             let x = noise * a.sin();
//             let y = noise * a.cos();
//             self.asteroid_model.push((x, y));
//         }
//         self.spawn_new_ship(s);
//         Ok(true)
//     }

//     fn on_update(&mut self, s: &mut State) -> Result<bool> {
//         if s.get_key(Key::Escape).pressed {
//             self.paused = !self.paused;
//         }
//         if s.get_key(Key::R).pressed {
//             self.reset(s);
//         }

//         if self.paused {
//             return Ok(true);
//         }

//         let elapsed = s.delta_time();

//         // Steer
//         if s.get_key(Key::Left).held {
//             self.ship.angle -= 5.0 * elapsed;
//         } else if s.get_key(Key::Right).held {
//             self.ship.angle += 5.0 * elapsed;
//         }

//         // Thrust
//         if s.get_key(Key::Up).held {
//             self.ship.dx += self.ship.angle.sin() * SHIP_THRUST * elapsed;
//             self.ship.dy += -self.ship.angle.cos() * SHIP_THRUST * elapsed;
//         }
//         // Shoot a bullet
//         if s.get_key(Key::Space).released {
//             self.bullets.push(SpaceObj::new(
//                 0,
//                 self.ship.x,
//                 self.ship.y,
//                 BULLET_SPEED * self.ship.angle.sin(),
//                 BULLET_SPEED * -self.ship.angle.cos(),
//                 100.0,
//             ));
//         }

//         s.clear();

//         if self.exploded {
//             if self.lives > 0 {
//                 self.exploded(s);
//             } else {
//                 s.set_draw_scale(3);
//                 s.draw_string(
//                     s.width() / 2 - 108,
//                     s.height() / 3 - 24,
//                     "GAME OVER",
//                     pixel::WHITE,
//                 );
//                 s.set_draw_scale(1);
//                 s.draw_string(
//                     s.width() / 2 - 88,
//                     s.height() / 3 + 16,
//                     "PRESS SPACE TO RESTART",
//                     pixel::WHITE,
//                 );
//                 if s.get_key(Key::Space).pressed {
//                     self.reset(s);
//                 }
//             }
//             return Ok(true);
//         }

//         // Draw Level, Lives, & Score
//         s.draw_string(
//             4,
//             4,
//             &format!("LEVEL: {}  SCORE: {}", self.level, self.score),
//             pixel::WHITE,
//         );
//         for i in 0..self.lives {
//             s.draw_wireframe(
//                 &self.ship_model,
//                 12.0 + (i as f64 * 14.0),
//                 36.0,
//                 0.0,
//                 2.0,
//                 pixel::WHITE,
//             );
//         }

//         self.ship.x += self.ship.dx * elapsed;
//         self.ship.y += self.ship.dy * elapsed;

//         // Keep ship in game space
//         s.wrap_coords(self.ship.x, self.ship.y, &mut self.ship.x, &mut self.ship.y);

//         // Draw asteroids
//         for a in self.asteroids.iter_mut() {
//             // Ship collision
//             if s.is_inside_circle(a.x, a.y, a.size as f64, self.ship.x, self.ship.y) {
//                 self.exploded = true;
//             }

//             a.x += a.dx * elapsed;
//             a.y += a.dy * elapsed;
//             a.angle += 0.5 * elapsed; // Give some twirl
//             s.wrap_coords(a.x, a.y, &mut a.x, &mut a.y);
//             s.draw_wireframe(
//                 &self.asteroid_model,
//                 a.x,
//                 a.y,
//                 a.angle,
//                 a.size as f64,
//                 pixel::YELLOW,
//             );
//         }

//         let mut new_asteroids = Vec::new();
//         // Draw bullets
//         for b in self.bullets.iter_mut() {
//             b.x += b.dx * elapsed;
//             b.y += b.dy * elapsed;
//             b.angle -= 1.0 * elapsed;

//             for a in self.asteroids.iter_mut() {
//                 if s.is_inside_circle(a.x, a.y, a.size as f64, b.x, b.y) {
//                     // Asteroid hit
//                     b.destroyed = true; // Removes bullet

//                     if a.size > MIN_ASTEROID_SIZE {
//                         // Break into two
//                         let a1 = rand::random::<f64>() * 2.0 * PI;
//                         let a2 = rand::random::<f64>() * 2.0 * PI;
//                         new_asteroids.push(SpaceObj::new(
//                             a.size >> 1,
//                             a.x,
//                             a.y,
//                             SHATTERED_ASTEROID_SPEED * a1.sin(),
//                             SHATTERED_ASTEROID_SPEED * a1.cos(),
//                             0.0,
//                         ));
//                         new_asteroids.push(SpaceObj::new(
//                             a.size >> 1,
//                             a.x,
//                             a.y,
//                             SHATTERED_ASTEROID_SPEED * a2.sin(),
//                             SHATTERED_ASTEROID_SPEED * a2.cos(),
//                             0.0,
//                         ));
//                     }
//                     a.destroyed = true; // Remove asteroid
//                     self.score += 100;
//                 }
//             }
//         }
//         self.asteroids.append(&mut new_asteroids);

//         // Remove offscreen/destroyed bullets
//         self.bullets.retain(|b| {
//             !b.destroyed
//                 && b.x >= 1.0
//                 && b.x < s.width() as f64
//                 && b.y >= 1.0
//                 && b.y < s.height() as f64
//         });
//         // Remove destroyed asteroids
//         self.asteroids.retain(|a| !a.destroyed);

//         // Draw bullets
//         for b in self.bullets.iter() {
//             s.draw_circle(b.x as u32, b.y as u32, 1, pixel::WHITE);
//         }

//         // Draw ship
//         s.draw_wireframe(
//             &self.ship_model,
//             self.ship.x,
//             self.ship.y,
//             self.ship.angle,
//             SHIP_SCALE,
//             pixel::WHITE,
//         );

//         // Win level
//         if self.asteroids.is_empty() {
//             self.level += 1;
//             self.score += 1000;
//             self.bullets.clear();
//             for _ in 0..(self.level + 2) {
//                 self.asteroids.push(SpaceObj::rand_asteroid(&self.ship, s));
//             }
//         }

//         Ok(true)
//     }
// }

pub fn main() {
    //     let app = App::new();
    //     let mut engine = PixEngine::create("Asteroids", app, 800, 600)
    //         .build()
    //         .expect("valid engine");
    //     if let Err(e) = engine.run() {
    //         eprintln!("Encountered a error: {:?}", e);
    //     }
}
