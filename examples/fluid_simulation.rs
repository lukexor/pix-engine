use pix_engine::prelude::*;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
const SCALE: u32 = 1;

const N: usize = (WIDTH / SCALE) as usize;
const NLEN: usize = N - 1;
const N_SCALAR: Scalar = N as Scalar;
const ITER: usize = 2;

const VEL: Scalar = 1.4; // Velocity of fluid
const TIME_INC: Scalar = 0.5; // Amount to step time each draw

const SPACING: usize = 20;
const COUNT: usize = (WIDTH / SCALE) as usize / SPACING + 1;

const DT: Scalar = 0.005;
const DTX: Scalar = DT * (N_SCALAR - 2.0);
const DTY: Scalar = DT * (N_SCALAR - 2.0);
const DIFF: Scalar = 0.00002; // Diffusion
const VISC: Scalar = 0.000000001; // Viscosity

struct Fluid {
    s: Vec<Scalar>,
    density: Vec<Scalar>,
    velx: Vec<Scalar>,
    vely: Vec<Scalar>,
    velx0: Vec<Scalar>,
    vely0: Vec<Scalar>,
}

fn get_idx(x: usize, y: usize) -> usize {
    let x = x.clamp(0, NLEN);
    let y = y.clamp(0, NLEN);
    x + y * N
}

fn diffuse(b: usize, xs: &mut [Scalar], xs0: &[Scalar], amt: Scalar) {
    let a = DT * amt * (N - 2).pow(2) as Scalar;
    linear_solve(b, xs, xs0, a, 1.0 + 6.0 * a);
}

fn project(velx: &mut [Scalar], vely: &mut [Scalar], p: &mut [Scalar], div: &mut [Scalar]) {
    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            div[idx] = -0.5
                * (velx[get_idx(i + 1, j)] - velx[get_idx(i - 1, j)] + vely[get_idx(i, j + 1)]
                    - vely[get_idx(i, j - 1)])
                / N_SCALAR;
            p[idx] = 0.0;
        }
    }
    set_bounds(0, div);
    set_bounds(0, p);
    linear_solve(0, p, div, 1.0, 6.0);

    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            velx[idx] -= 0.5 * (p[get_idx(i + 1, j)] - p[get_idx(i - 1, j)]) * N_SCALAR;
            vely[idx] -= 0.5 * (p[get_idx(i, j + 1)] - p[get_idx(i, j - 1)]) * N_SCALAR;
        }
    }
    set_bounds(1, velx);
    set_bounds(2, vely);
}

fn advect(b: usize, d: &mut [Scalar], d0: &[Scalar], velx: &[Scalar], vely: &[Scalar]) {
    let (mut i0, mut i1, mut j0, mut j1);

    let (mut s0, mut s1, mut t0, mut t1);

    for j in 1..NLEN {
        for i in 1..NLEN {
            let idx = get_idx(i, j);
            let mut x = i as Scalar - (DTX * velx[idx]);
            let mut y = j as Scalar - (DTY * vely[idx]);

            if x < 0.5 {
                x = 0.5;
            }
            if x > N_SCALAR + 0.5 {
                x = N_SCALAR + 0.5;
            }
            i0 = x.floor() as usize;
            i1 = i0 + 1;
            if y < 0.5 {
                y = 0.5;
            }
            if y > N_SCALAR + 0.5 {
                y = N_SCALAR + 0.5;
            }
            j0 = y.floor() as usize;
            j1 = j0 + 1;

            s1 = x - i0 as Scalar;
            s0 = 1.0 - s1;
            t1 = y - j0 as Scalar;
            t0 = 1.0 - t1;

            let pd = d[idx].clamp(0.0, 500.0);
            d[idx] = s0 * (t0 * d0[get_idx(i0, j0)] + t1 * d0[get_idx(i0, j1)])
                + s1 * (t0 * d0[get_idx(i1, j0)] + t1 * d0[get_idx(i1, j1)]);
            d[idx] = d[idx].clamp(pd - 150.0, 500.0);
        }
    }
    set_bounds(b, d);
}

fn linear_solve(b: usize, xs: &mut [Scalar], xs0: &[Scalar], a: Scalar, c: Scalar) {
    let c_recip = c.recip();
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

fn set_bounds(b: usize, xs: &mut [Scalar]) {
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
        let count = N * N;
        Self {
            s: vec![0.0; count],
            density: vec![0.0; count],
            velx: vec![0.0; count],
            vely: vec![0.0; count],
            velx0: vec![0.0; count],
            vely0: vec![0.0; count],
        }
    }

    fn step(&mut self) {
        diffuse(1, &mut self.velx0, &self.velx, VISC);
        diffuse(2, &mut self.vely0, &self.vely, VISC);

        project(
            &mut self.velx0,
            &mut self.vely0,
            &mut self.velx,
            &mut self.vely,
        );

        advect(1, &mut self.velx, &self.velx0, &self.velx0, &self.vely0);
        advect(2, &mut self.vely, &self.vely0, &self.velx0, &self.vely0);

        project(
            &mut self.velx,
            &mut self.vely,
            &mut self.velx0,
            &mut self.vely0,
        );

        diffuse(0, &mut self.s, &self.density, DIFF);
        advect(0, &mut self.density, &self.s, &self.velx, &self.vely);
    }

    #[allow(clippy::many_single_char_names)]
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.step();
        let scale = SCALE as i32;
        for i in 1..NLEN {
            for j in 1..NLEN {
                let x = i as i32 * scale;
                let y = j as i32 * scale;
                let idx = get_idx(i, j);

                // Draw density
                let d = self.density[idx];
                let m = d / 100.0;
                let f = m * d;
                if f > 10.0 {
                    s.fill(rgb!(
                        (f / 2.0).floor() as u8,
                        (f / 6.0).floor() as u8,
                        (f / 16.0).floor() as u8,
                    ));
                    s.square([x, y, scale])?;
                }
            }
        }
        Ok(())
    }

    fn add_density(&mut self, idx: usize, amount: Scalar) {
        self.density[idx] += amount;
        let velx = random!(-VEL, VEL);
        self.add_velocity(idx, velx, -0.06);
    }

    fn add_velocity(&mut self, idx: usize, amount_x: Scalar, amount_y: Scalar) {
        self.velx[idx] += amount_x;
        self.vely[idx] += amount_y;
    }
}

struct App {
    fluid: Fluid,
    t: Scalar,
    xs: [Scalar; COUNT],
    ys: [Scalar; COUNT],
}

impl App {
    fn new() -> Self {
        Self {
            fluid: Fluid::new(),
            t: 0.0,
            xs: [0.0; COUNT],
            ys: [0.0; COUNT],
        }
    }

    fn flame_on(&mut self, _s: &mut PixState) -> PixResult<()> {
        for k in 0..COUNT {
            for i in -9..=8 {
                for j in -8..=2 {
                    let idx = get_idx(
                        (self.xs[k] + i as Scalar).floor() as usize,
                        (self.ys[k] + j as Scalar).floor() as usize,
                    );
                    self.fluid.add_density(idx, random!(20.0));
                }
            }
        }
        Ok(())
    }

    fn drag(&mut self, s: &mut PixState) -> PixResult<()> {
        let m = s.mouse_pos();
        let r = 4.0;
        let scale = SCALE as i32;
        for i in 0..628 {
            let (sin, cos) = (i as Scalar * 0.01).sin_cos();
            let idx = get_idx(
                ((m.x() / scale) as Scalar + r * cos).floor() as usize,
                ((m.y() / scale) as Scalar + r * sin).floor() as usize,
            );
            self.fluid.add_density(idx, random!(500.0));
        }
        Ok(())
    }
}

impl AppState for App {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK)?;
        s.rect_mode(DrawMode::Center);
        s.no_stroke();

        for i in 0..COUNT {
            self.xs[i] = (i * SPACING) as Scalar;
            self.ys[i] = N as Scalar;
        }

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        if s.mouse_down(Mouse::Left) {
            self.drag(s)?;
        }
        self.flame_on(s)?;
        self.t += TIME_INC;
        self.fluid.on_update(s)?;
        Ok(())
    }

    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        self.drag(s)
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Fluid Simulation")
        .with_frame_rate()
        .scale(2.0, 2.0)
        .position_centered()
        .vsync_enabled()
        .build();
    let mut app = App::new();
    engine.run(&mut app)
}
