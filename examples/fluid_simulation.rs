use pix_engine::prelude::*;

const TITLE: &str = "Fluid Simulation";
const WIDTH: u32 = 600 - 10;
const HEIGHT: u32 = 600;

const N: usize = 300;
const NLEN: usize = N - 1;
const NF: f64 = N as f64;
const SCALE: u32 = 2;
const ITER: usize = 4;

const VEL: f64 = 1.0; // Velocity of fluid from perlin noise
const TIME_INC: f64 = 2.02; // Amount to step time for perlin noise each draw

const SPACING: usize = 12;
const COUNT: usize = (WIDTH / SCALE) as usize / SPACING + 1;

struct Fluid {
    dt: f64,
    diff: f64,
    visc: f64,
    s: [f64; N * N],
    density: [f64; N * N],
    velx: [f64; N * N],
    vely: [f64; N * N],
    velx0: [f64; N * N],
    vely0: [f64; N * N],
    flame: Image,
}

fn get_idx(x: usize, y: usize) -> usize {
    let x = x.clamp(0, NLEN);
    let y = y.clamp(0, NLEN);
    return x + y * N;
}

fn diffuse(b: usize, xs: &mut [f64], xs0: &[f64], diff: f64, dt: f64) {
    let a = dt * diff * (N - 2).pow(2) as f64;
    linear_solve(b, xs, xs0, a, 1.0 + 6.0 * a);
}

fn project(velx: &mut [f64], vely: &mut [f64], p: &mut [f64], div: &mut [f64]) {
    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            div[idx] = -0.5
                * (velx[get_idx(i + 1, j)] - velx[get_idx(i - 1, j)] + vely[get_idx(i, j + 1)]
                    - vely[get_idx(i, j - 1)])
                / NF;
            p[idx] = 0.0;
        }
    }
    set_bounds(0, div);
    set_bounds(0, p);
    linear_solve(0, p, div, 1.0, 6.0);

    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            velx[idx] -= 0.5 * (p[get_idx(i + 1, j)] - p[get_idx(i - 1, j)]) * NF;
            vely[idx] -= 0.5 * (p[get_idx(i, j + 1)] - p[get_idx(i, j - 1)]) * NF;
        }
    }
    set_bounds(1, velx);
    set_bounds(2, vely);
}

fn advect(b: usize, d: &mut [f64], d0: &[f64], velx: &[f64], vely: &[f64], dt: f64) {
    let (mut i0, mut i1, mut j0, mut j1);

    let dtx = dt * (NF - 2.0);
    let dty = dt * (NF - 2.0);

    let (mut s0, mut s1, mut t0, mut t1);

    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            let mut x = i as f64 - (dtx * velx[idx]);
            let mut y = j as f64 - (dty * vely[idx]);

            if x < 0.5 {
                x = 0.5;
            }
            if x > NF + 0.5 {
                x = NF + 0.5;
            }
            i0 = x.floor() as usize;
            i1 = i0 + 1;
            if y < 0.5 {
                y = 0.5;
            }
            if y > NF + 0.5 {
                y = NF + 0.5;
            }
            j0 = y.floor() as usize;
            j1 = j0 + 1;

            s1 = x - i0 as f64;
            s0 = 1.0 - s1;
            t1 = y - j0 as f64;
            t0 = 1.0 - t1;

            let mut pd = d[idx];
            // NEW
            if pd > 450.0 {
                pd = 450.0;
            }
            // NEW
            d[idx] = s0 * (t0 * d0[get_idx(i0, j0)] + t1 * d0[get_idx(i0, j1)])
                + s1 * (t0 * d0[get_idx(i1, j0)] + t1 * d0[get_idx(i1, j1)]);
            d[idx] = d[idx].clamp(pd - 150.0, 450.0);
        }
    }
    set_bounds(b, d);
}

fn linear_solve(b: usize, xs: &mut [f64], xs0: &[f64], a: f64, c: f64) {
    let c_recip = 1.0 / c;
    for _ in 0..ITER {
        for j in 1..NLEN {
            for i in 1..NLEN {
                let idx = get_idx(i, j);
                xs[idx] = (xs0[idx]
                    + a * (xs[get_idx(i + 1, j)]
                        + xs[get_idx(i - 1, j)]
                        + xs[get_idx(i, j + 1)]
                        + xs[get_idx(i, j - 1)]))
                    * c_recip;
            }
        }
    }
    set_bounds(b, xs);
}

fn set_bounds(b: usize, xs: &mut [f64]) {
    // Top and bottom
    for i in 1..NLEN {
        if b == 2 {
            xs[get_idx(i, 0)] = -xs[get_idx(i, 1)];
            xs[get_idx(i, N - 1)] = -xs[get_idx(i, N - 2)];
        } else {
            xs[get_idx(i, 0)] = xs[get_idx(i, 1)];
            xs[get_idx(i, N - 1)] = xs[get_idx(i, N - 2)];
        }
    }
    // left and right
    for j in 1..NLEN {
        if b == 1 {
            xs[get_idx(0, j)] = -xs[get_idx(1, j)];
            xs[get_idx(N - 1, j)] = -xs[get_idx(N - 2, j)];
        } else {
            xs[get_idx(0, j)] = xs[get_idx(1, j)];
            xs[get_idx(N - 1, j)] = xs[get_idx(N - 2, j)];
        }
    }

    xs[get_idx(0, 0)] = 0.5 * (xs[get_idx(1, 0)] + xs[get_idx(0, 1)]);
    xs[get_idx(0, NLEN)] = 0.5 * (xs[get_idx(1, NLEN)] + xs[get_idx(0, N - 2)]);
    xs[get_idx(NLEN, 0)] = 0.5 * (xs[get_idx(N - 2, 0)] + xs[get_idx(NLEN, 1)]);
    xs[get_idx(NLEN, NLEN)] = 0.5 * (xs[get_idx(N - 2, NLEN)] + xs[get_idx(NLEN, N - 2)]);
}

impl Fluid {
    pub fn new() -> Self {
        Self {
            dt: 0.01,          // Time step
            diff: 0.00001,     // Diffusion
            visc: 0.000000005, // Viscosity
            s: [0.0; N * N],
            density: [0.0; N * N],
            flame: Image::new(0, 0),
            velx: [0.0; N * N],
            vely: [0.0; N * N],
            velx0: [0.0; N * N],
            vely0: [0.0; N * N],
        }
    }

    fn step(&mut self) {
        diffuse(1, &mut self.velx0, &mut self.velx, self.visc, self.dt);
        diffuse(2, &mut self.vely0, &mut self.vely, self.visc, self.dt);

        project(
            &mut self.velx0,
            &mut self.vely0,
            &mut self.velx,
            &mut self.vely,
        );

        advect(
            1,
            &mut self.velx,
            &self.velx0,
            &self.velx0,
            &self.vely0,
            self.dt,
        );
        advect(
            2,
            &mut self.vely,
            &self.vely0,
            &self.velx0,
            &self.vely0,
            self.dt,
        );

        project(
            &mut self.velx,
            &mut self.vely,
            &mut self.velx0,
            &mut self.vely0,
        );

        diffuse(0, &mut self.s, &mut self.density, self.diff, self.dt);
        advect(
            0,
            &mut self.density,
            &self.s,
            &self.velx,
            &self.vely,
            self.dt,
        );
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.step();
        for i in 1..NLEN {
            for j in 1..NLEN {
                let x = i as i32 * SCALE as i32;
                let y = j as i32 * SCALE as i32;
                let idx = get_idx(i, j);

                // Draw density
                let d = self.density[idx];
                let m = d / 100.0;
                let f = m * d;
                if f > 15.0 {
                    s.fill(rgb!(f.floor() as u8, (f / 3.0).floor() as u8, 0, 220));
                    s.square((x, y, SCALE))?;
                }
            }
        }
        Ok(())
    }

    fn add_density(&mut self, idx: usize, amount: f64) {
        self.density[idx] += amount;
        let velx = random!(-1.5 * VEL, 1.5 * VEL);
        let vely = random!(-0.8 * VEL, -0.1 * VEL);
        self.add_velocity(idx, velx, vely);
    }

    fn add_velocity(&mut self, idx: usize, amount_x: f64, amount_y: f64) {
        self.velx[idx] += amount_x;
        self.vely[idx] += amount_y;
    }
}

struct App {
    fluid: Fluid,
    t: f64,
    xs: [f64; COUNT],
    ys: [f64; COUNT],
    base: Rect,
}

impl App {
    fn new() -> Self {
        Self {
            fluid: Fluid::new(),
            t: 0.0,
            xs: [0.0; COUNT],
            ys: [0.0; COUNT],
            base: rect!(0, HEIGHT as i32 - 10, WIDTH * SCALE, 20),
        }
    }

    fn flame_on(&mut self, _s: &mut PixState) -> PixResult<()> {
        for k in 0..COUNT {
            for i in (-9..9).step_by(3) {
                for j in -5..=0 {
                    let idx = get_idx(
                        (self.xs[k] + i as f64).floor() as usize,
                        (self.ys[k] + j as f64).floor() as usize,
                    );
                    self.fluid.add_density(idx, random!(50.0));
                }
            }
        }
        Ok(())
    }

    fn drag(&mut self, s: &mut PixState) -> PixResult<()> {
        let (x, y) = s.mouse_pos().into();
        let r = 3.0;
        for i in 0..628 {
            let (sin, cos) = (i as f64 * 0.01).sin_cos();
            let idx = get_idx(
                ((x / SCALE as i32) as f64 + r * cos).floor() as usize,
                ((y / SCALE as i32) as f64 + r * sin).floor() as usize,
            );
            self.fluid.density[idx] += random!(100.0);
            let velx = random!(-2.0 * VEL, 2.0 * VEL);
            let vely = random!(-0.05 * VEL, -0.01 * VEL);
            self.fluid.add_velocity(idx, velx, vely);
        }
        Ok(())
    }
}

impl AppState for App {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.rect_mode(DrawMode::Center);
        s.no_stroke();
        self.fluid.flame = Image::load("static/flame.png")?;

        for i in 0..COUNT {
            self.xs[i] = (i * SPACING) as f64;
            self.ys[i] = N as f64;
        }

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK);
        s.clear();
        if s.mouse_pressed(Mouse::Left) {
            self.drag(s)?;
        }
        self.flame_on(s)?;
        self.t += TIME_INC;
        self.fluid.on_update(s)?;
        s.fill(DARK_SLATE_GRAY);
        s.rect(self.base)?;
        Ok(())
    }

    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        self.drag(s)
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
