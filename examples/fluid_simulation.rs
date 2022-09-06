use pix_engine::prelude::*;
use rayon::prelude::*;

const WIDTH: u32 = 350;
const HEIGHT: u32 = 150;

const N: usize = WIDTH as usize;
const NLEN: usize = N - 1;
const NHEIGHT: usize = HEIGHT as usize;
const N_SCALAR: f64 = N as f64;

const XVEL: f64 = 1.8; // Velocity of fluid

const SPACING: usize = 20;
const COUNT: usize = N / SPACING + 1;

const DT: f64 = 0.004; // Delta time modifier
const DIFF: f64 = 0.000018; // Diffusion
const VISC: f64 = 0.00000001; // Viscosity

struct Fluid {
    s: Vec<f64>,
    density: Vec<f64>,
    velx: Vec<f64>,
    vely: Vec<f64>,
    velx0: Vec<f64>,
    vely0: Vec<f64>,
    tmp: Vec<f64>,
}

fn get_idx(x: usize, y: usize) -> usize {
    let x = x.clamp(0, NLEN);
    let y = y.clamp(0, NLEN);
    x + y * N
}

fn get_xy(idx: usize) -> (usize, usize) {
    (idx % N, idx / N)
}

fn diffuse(b: usize, xs: &mut [f64], xs0: &[f64], amt: f64, tmp: &mut [f64]) {
    let a = DT * amt * (N - 2).pow(2) as f64;
    linear_solve(b, xs, xs0, a, 1.0 + 6.0 * a, tmp);
}

fn project(velx: &mut [f64], vely: &mut [f64], p: &mut [f64], div: &mut [f64], tmp: &mut [f64]) {
    let c = 1.0 / 6.0;
    div.par_iter_mut()
        .zip(tmp.par_iter_mut())
        .enumerate()
        .for_each(|(i, (div, tmp))| {
            let (x, y) = get_xy(i);
            if (1..NLEN).contains(&x) && (1..NHEIGHT).contains(&y) {
                *div = -0.5
                    * (velx[get_idx(x + 1, y)] - velx[get_idx(x - 1, y)] + vely[get_idx(x, y + 1)]
                        - vely[get_idx(x, y - 1)])
                    / N_SCALAR;
                *tmp = *div * c;
            }
        });
    p.swap_with_slice(tmp);
    set_bounds(0, p);

    velx.par_iter_mut()
        .zip(vely.par_iter_mut())
        .enumerate()
        .for_each(|(i, (velx, vely))| {
            let (x, y) = get_xy(i);
            if (1..NLEN).contains(&x) && (1..NHEIGHT).contains(&y) {
                *velx -= 0.5 * (p[get_idx(x + 1, y)] - p[get_idx(x - 1, y)]) * N_SCALAR;
                *vely -= 0.5 * (p[get_idx(x, y + 1)] - p[get_idx(x, y - 1)]) * N_SCALAR;
            }
        });
}

fn advect(b: usize, d: &mut [f64], d0: &[f64], velx: &[f64], vely: &[f64]) {
    d.par_iter_mut().enumerate().for_each(|(i, d)| {
        let (x, y) = get_xy(i);
        if (1..NLEN).contains(&x) && (1..NHEIGHT).contains(&y) {
            let mut x = x as f64 - (DT * N_SCALAR * velx[i]);
            let mut y = y as f64 - (DT * N_SCALAR * vely[i]);

            if x < 0.5 {
                x = 0.5;
            }
            if x > N_SCALAR + 0.5 {
                x = N_SCALAR + 0.5;
            }
            let i0 = x.floor() as usize;
            let i1 = i0 + 1;
            if y < 0.5 {
                y = 0.5;
            }
            if y > N_SCALAR + 0.5 {
                y = N_SCALAR + 0.5;
            }
            let j0 = y.floor() as usize;
            let j1 = j0 + 1;

            let s1 = x - i0 as f64;
            let s0 = 1.0 - s1;
            let t1 = y - j0 as f64;
            let t0 = 1.0 - t1;

            let pd = d.clamp(0.0, 500.0);
            *d = s0 * (t0 * d0[get_idx(i0, j0)] + t1 * d0[get_idx(i0, j1)])
                + s1 * (t0 * d0[get_idx(i1, j0)] + t1 * d0[get_idx(i1, j1)]);
            *d = d.clamp(pd - 150.0, 500.0);
        }
    });
    set_bounds(b, d);
}

#[allow(clippy::many_single_char_names)]
fn linear_solve(b: usize, xs: &mut [f64], xs0: &[f64], a: f64, c: f64, tmp: &mut [f64]) {
    let c_recip = c.recip();
    tmp.par_iter_mut().enumerate().for_each(|(i, tmp)| {
        let (x, y) = get_xy(i);
        if (1..NLEN).contains(&x) && (1..NHEIGHT).contains(&y) {
            *tmp = (xs0[i]
                + a * (xs[get_idx(x + 1, y)]
                    + xs[get_idx(x - 1, y)]
                    + xs[get_idx(x, y + 1)]
                    + xs[get_idx(x, y - 1)]))
                * c_recip;
        }
    });
    xs.swap_with_slice(tmp);
    set_bounds(b, xs);
}

fn set_bounds(b: usize, xs: &mut [f64]) {
    for i in 1..NLEN {
        let (_, y) = get_xy(i);
        if y > NHEIGHT {
            break;
        }
        // Top and bottom
        if b == 2 {
            xs[get_idx(i, 0)] = -xs[get_idx(i, 1)];
            xs[get_idx(i, N - 1)] = -xs[get_idx(i, N - 2)];
        } else {
            xs[get_idx(i, 0)] = xs[get_idx(i, 1)];
            xs[get_idx(i, N - 1)] = xs[get_idx(i, N - 2)];
        }
        // left and right
        if b == 1 {
            xs[get_idx(0, i)] = -xs[get_idx(1, i)];
            xs[get_idx(N - 1, i)] = -xs[get_idx(N - 2, i)];
        } else {
            xs[get_idx(0, i)] = xs[get_idx(1, i)];
            xs[get_idx(N - 1, i)] = xs[get_idx(N - 2, i)];
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
            tmp: vec![0.0; count],
        }
    }

    fn step(&mut self) {
        diffuse(1, &mut self.velx0, &self.velx, VISC, &mut self.tmp);
        diffuse(2, &mut self.vely0, &self.vely, VISC, &mut self.tmp);

        project(
            &mut self.velx0,
            &mut self.vely0,
            &mut self.velx,
            &mut self.vely,
            &mut self.tmp,
        );

        advect(1, &mut self.velx, &self.velx0, &self.velx0, &self.vely0);
        advect(2, &mut self.vely, &self.vely0, &self.velx0, &self.vely0);

        project(
            &mut self.velx,
            &mut self.vely,
            &mut self.velx0,
            &mut self.vely0,
            &mut self.tmp,
        );

        diffuse(0, &mut self.s, &self.density, DIFF, &mut self.tmp);
        advect(0, &mut self.density, &self.s, &self.velx, &self.vely);
    }

    #[allow(clippy::many_single_char_names)]
    fn on_update(&mut self, s: &mut PixState) -> Result<()> {
        self.step();
        for i in 0..N * NHEIGHT {
            let (x, y) = get_xy(i);
            // Draw density
            let d = self.density[i];
            let m = d / 100.0;
            let f = m * d;
            if f > 10.0 {
                s.fill(rgb!(
                    (f / 2.0).floor() as u8,
                    (f / 6.0).floor() as u8,
                    (f / 16.0).floor() as u8,
                ));
                s.square([x as i32, y as i32, 1])?;
            }
        }
        Ok(())
    }

    fn add_density(&mut self, idx: usize, amount: f64) {
        self.density[idx] += amount;
        let velx = random!(-XVEL, XVEL);
        let vely = random!(-0.03, -0.01);
        self.add_velocity(idx, velx, vely);
    }

    fn add_velocity(&mut self, idx: usize, amount_x: f64, amount_y: f64) {
        self.velx[idx] += amount_x;
        self.vely[idx] += amount_y;
    }
}

struct App {
    fluid: Fluid,
    sincos: Vec<(f64, f64)>,
    xs: [f64; COUNT],
    ys: [f64; COUNT],
}

impl App {
    fn new() -> Self {
        let mut sincos = Vec::with_capacity(628);
        for i in 0..628 {
            sincos.push((i as f64 * 0.01).sin_cos());
        }
        Self {
            fluid: Fluid::new(),
            sincos,
            xs: [0.0; COUNT],
            ys: [0.0; COUNT],
        }
    }

    fn flame_on(&mut self) -> Result<()> {
        for k in 0..COUNT {
            let xmin = random!(-10, -5);
            let xmax = random!(5, 10);
            for i in xmin..xmax {
                let ymin = random!(-16, 0);
                for j in ymin..0 {
                    let idx = get_idx(
                        (self.xs[k] + i as f64).floor() as usize,
                        (self.ys[k] + j as f64).floor() as usize,
                    );
                    self.fluid.add_density(idx, random!(10.0, 40.0));
                    let velx = random!(-XVEL / 2.0, XVEL / 2.0);
                    let vely = random!(-0.05, 0.01);
                    self.fluid.add_velocity(idx, velx, vely);
                }
            }
        }
        Ok(())
    }

    fn drag(&mut self, pos: Point<i32>) -> Result<()> {
        let mx = pos.x() as f64;
        let my = pos.y() as f64;
        for r in 3..10 {
            let r = r as f64;
            for (sin, cos) in self.sincos.iter() {
                let idx = get_idx((mx + r * cos) as usize, (my + r * sin) as usize);
                self.fluid.add_density(idx, random!(2.0, 5.0));
            }
        }
        Ok(())
    }
}

impl PixEngine for App {
    fn on_start(&mut self, s: &mut PixState) -> Result<()> {
        s.background(Color::BLACK);
        s.rect_mode(RectMode::Center);
        s.stroke(None);
        s.cursor(Cursor::hand())?;
        s.clip(rect![0, 0, WIDTH as i32, HEIGHT as i32 - 10])?;

        for i in 0..COUNT {
            self.xs[i] = (i * SPACING) as f64;
            self.ys[i] = HEIGHT as f64;
        }

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> Result<()> {
        s.clear()?;
        if s.mouse_down(Mouse::Left) {
            self.drag(s.mouse_pos())?;
        }
        self.flame_on()?;
        self.fluid.on_update(s)?;
        Ok(())
    }

    fn on_mouse_dragged(
        &mut self,
        _s: &mut PixState,
        pos: Point<i32>,
        _rel_pos: Point<i32>,
    ) -> Result<bool> {
        self.drag(pos)?;
        Ok(false)
    }
}

pub fn main() -> Result<()> {
    println!("Renders more smoothly using `--release` and `--features opengl`.");
    let mut engine = Engine::builder()
        .with_dimensions(2 * WIDTH, 2 * HEIGHT)
        .scale(2.0, 2.0)
        .with_title("Fluid Simulation")
        .with_frame_rate()
        .target_frame_rate(30)
        .build()?;
    let mut app = App::new();
    engine.run(&mut app)
}
