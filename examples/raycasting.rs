use pix_engine::prelude::*;
use std::cmp::Ordering::Less;

const TITLE: &str = "Raycasting";
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const SCALE: u32 = 1;

const BLOCK_SIZE: u32 = 40;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

struct Edge {
    start: Vector,
    end: Vector,
}

impl Edge {
    pub fn new<V: Into<Vector>>(start: V, end: V) -> Self {
        let start = start.into();
        let end = end.into();
        Self { start, end }
    }
}

struct RayScene {
    cells: Vec<Cell>,
    edges: Vec<Edge>,
    points: Vec<Vector>,
    polygons: Vec<(f64, Vector)>,
    xcells: u32,
    ycells: u32,
    drawing: bool,
    light: Image,
}

impl RayScene {
    fn new() -> Self {
        let xcells = WIDTH / (BLOCK_SIZE * SCALE);
        let ycells = HEIGHT / (BLOCK_SIZE * SCALE);
        let mut cells = Vec::with_capacity((xcells * ycells) as usize);
        for y in 0..ycells {
            for x in 0..xcells {
                cells.push(Cell::new((x, y)));
            }
        }
        Self {
            cells,
            edges: Vec::with_capacity(20),
            points: Vec::with_capacity(40),
            polygons: Vec::new(),
            xcells,
            ycells,
            drawing: false,
            light: Image::new(0, 0),
        }
    }

    fn get_cell_index(&self, x: i32, y: i32) -> usize {
        ((y / BLOCK_SIZE as i32) * self.xcells as i32 + (x / BLOCK_SIZE as i32)) as usize
    }

    fn exists(&self, i: usize) -> bool {
        self.cells.get(i).map(|c| c.exists).unwrap_or(false)
    }

    fn has_edge(&self, i: usize, dir: usize) -> bool {
        self.cells.get(i).map(|c| c.edges[dir].0).unwrap_or(false)
    }
    fn get_edge_index(&mut self, i: usize, dir: usize) -> usize {
        self.cells.get(i).map(|c| c.edges[dir].1).unwrap()
    }
    fn get_edge_mut(&mut self, i: usize) -> &mut Edge {
        self.edges.get_mut(i).unwrap()
    }

    #[allow(clippy::many_single_char_names)]
    fn convert_edges_to_poly_map(&mut self) {
        let s = Rect::new(0, 0, self.xcells, self.ycells);
        let pitch = self.xcells as i32;
        let block_size = BLOCK_SIZE as i32;
        // Reset edges state, keeping only the window boundaries
        self.edges.truncate(4);
        for c in self.cells.iter_mut() {
            c.reset();
        }
        let width = s.w as i32;
        let height = s.h as i32;
        for x in 0..width {
            for y in 0..height {
                let x_off = x + s.x;
                let y_off = y + s.y;
                let i = (y_off * pitch + x_off) as usize; // This
                let n = ((y_off - 1) * pitch + x_off) as usize; // Northern neighbor
                let s = ((y_off + 1) * pitch + x_off) as usize; // Sourthern neighbor
                let w = (y_off * pitch + (x_off - 1)) as usize; // Western neighbor
                let e = (y_off * pitch + (x_off + 1)) as usize; // Eastern neighbor

                // Cell exists, check for edges
                if self.exists(i) {
                    // No western neighbor, so needs an edge
                    if x > 0 && !self.exists(w) {
                        // Can extend down from northern neighbors WEST edge
                        if self.has_edge(n, WEST) {
                            let edge_id = self.get_edge_index(n, WEST);
                            self.get_edge_mut(edge_id).end.y += block_size as f64;
                            self.cells[i].edges[WEST] = (true, edge_id);
                        } else {
                            // Create WEST edge extending downward
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as f64);
                            let edge = Edge { start, end };
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[WEST] = (true, edge_id);
                        }
                    }
                    // No eastern neighbor, so needs an edge
                    if x < width && !self.exists(e) {
                        // Can extend down from northern neighbors EAST edge
                        if self.has_edge(n, EAST) {
                            let edge_id = self.get_edge_index(n, EAST);
                            self.get_edge_mut(edge_id).end.y += block_size as f64;
                            self.cells[i].edges[EAST] = (true, edge_id);
                        } else {
                            // Create EAST edge extending downward
                            let start =
                                vector!(x_off * block_size + block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as f64);
                            let edge = Edge { start, end };
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[EAST] = (true, edge_id);
                        }
                    }
                    // No northern neighbor, so needs an edge
                    if y > 0 && !self.exists(n) {
                        // Can extend from western neighbors NORTH edge
                        if self.has_edge(w, NORTH) {
                            let edge_id = self.get_edge_index(w, NORTH);
                            self.get_edge_mut(edge_id).end.x += block_size as f64;
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        } else {
                            // Create NORTH edge extending right
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x + block_size as f64, start.y);
                            let edge = Edge { start, end };
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        }
                    }
                    // No southern neighbor, so needs an edge
                    if y < height && !self.exists(s) {
                        // Can extend from western neighbors SOUTH edge
                        if self.has_edge(w, SOUTH) {
                            let edge_id = self.get_edge_index(w, SOUTH);
                            self.get_edge_mut(edge_id).end.x += block_size as f64;
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        } else {
                            // Create SOUTH edge extending right
                            let start =
                                vector!(x_off * block_size, y_off * block_size + block_size);
                            let end = vector!(start.x + block_size as f64, start.y);
                            let edge = Edge { start, end };
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        }
                    }
                }
            }
        }

        self.points.clear();
        for edge in &self.edges {
            self.points.push(edge.start);
            self.points.push(edge.end);
        }
        self.points
            .sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap_or(Less));
        self.points.dedup();
    }

    fn calc_visibility_polygons(&mut self, o: Vector) {
        self.polygons.clear();
        for p in self.points.iter() {
            // Cast three rays - one at and one off to each side
            for offset in -1..2 {
                let angle = offset as f64 / 10_000.0;
                let mut r = p.copy() - o;
                r.rotate(angle);
                if let Some(intersect) = self.cast_ray(o, r) {
                    self.polygons.push((r.heading(), intersect));
                }
            }
        }

        // Could fail with NaN or Infinity
        self.polygons
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Less));
        self.polygons
            .dedup_by(|a, b| (a.1.x - b.1.x).abs() <= 0.1 && (a.1.y - b.1.y).abs() <= 0.1);
    }

    fn cast_ray(&self, o: Vector, r: Vector) -> Option<Vector> {
        let mut intersect = None;
        let mut closest_param = INFINITY;
        for e in self.edges.iter() {
            if let Some((point, param)) = Self::intersection(o, r + o, e.start, e.end) {
                if intersect.is_none() || param < closest_param {
                    intersect = Some(point);
                    closest_param = param;
                }
            }
        }
        intersect
    }

    fn intersection(rs: Vector, re: Vector, es: Vector, ee: Vector) -> Option<(Vector, f64)> {
        // Ray parametric
        // RAY in parametric: Vector + Delta*T1
        let r_px = rs.x;
        let r_py = rs.y;
        let r_dx = re.x - rs.x;
        let r_dy = re.y - rs.y;

        // SEGMENT in parametric: Vector + Delta*T2
        let s_px = es.x;
        let s_py = es.y;
        let s_dx = ee.x - es.x;
        let s_dy = ee.y - es.y;

        // Are they parallel? If so, no intersect
        let r_mag = (r_dx * r_dx + r_dy * r_dy).sqrt();
        let s_mag = (s_dx * s_dx + s_dy * s_dy).sqrt();
        if (r_dx / r_mag - s_dx / s_mag).abs() < std::f64::EPSILON
            && (r_dy / r_mag - s_dy / s_mag).abs() < std::f64::EPSILON
        {
            return None;
        }

        // SOLVE FOR T1 & T2
        // r_px+r_dx*T1 = s_px+s_dx*T2 && r_py+r_dy*T1 = s_py+s_dy*T2
        // ==> T1 = (s_px+s_dx*T2-r_px)/r_dx = (s_py+s_dy*T2-r_py)/r_dy
        // ==> s_px*r_dy + s_dx*T2*r_dy - r_px*r_dy = s_py*r_dx + s_dy*T2*r_dx - r_py*r_dx
        // ==> T2 = (r_dx*(s_py-r_py) + r_dy*(r_px-s_px))/(s_dx*r_dy - s_dy*r_dx)
        let t2 = (r_dx * (s_py - r_py) + r_dy * (r_px - s_px)) / (s_dx * r_dy - s_dy * r_dx);
        let t1 = (s_px + s_dx * t2 - r_px) / r_dx;

        // Must be within parametic whatevers for RAY/SEGMENT
        if t1 < 0.0 || t1.is_infinite() || t1.is_nan() || t2 < 0.0 || t2 > 1.0 {
            return None;
        }

        // Return the POINT OF INTERSECTION
        Some((Vector::new_2d(r_px + r_dx * t1, r_py + r_dy * t1), t1))
    }

    fn draw_visibility_polygons(&mut self, s: &mut PixState) -> PixResult<bool> {
        let mouse_pos = s.mouse_pos();
        let (x, y) = mouse_pos.into();
        if x <= 0 || x > s.width() as i32 || y <= 0 || y > s.height() as i32 {
            return Ok(true);
        }

        self.calc_visibility_polygons((x, y).into());

        if !self.polygons.is_empty() {
            s.fill(WHITE);
            s.stroke(WHITE);
            for i in 0..self.polygons.len() - 1 {
                let p1: Point = self.polygons[i].1.into();
                let p2: Point = self.polygons[i + 1].1.into();
                s.triangle((mouse_pos, p1, p2))?;
            }
            // Draw last triangle, connecting back to first point.
            let p1: Point = self.polygons.last().unwrap().1.into();
            let p2: Point = self.polygons[0].1.into();
            s.triangle((mouse_pos, p1, p2))?;
        }

        s.fill(RED);
        s.no_stroke();
        s.circle(x, y, 2)?;
        Ok(true)
    }
}

impl AppState for RayScene {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK);
        s.scale(SCALE as f32, SCALE as f32)?;
        s.cursor(false);
        s.show_frame_rate(true);

        let w = (self.xcells * BLOCK_SIZE) as i32 - 1;
        let h = (self.ycells * BLOCK_SIZE) as i32 - 1;

        // Random scattered cells to start with
        for _ in 0..50 {
            let i = self.get_cell_index(random!(w - 1), random!(h - 1));
            self.cells[i].exists = !self.cells[i].exists;
        }

        // Screen Edges
        self.edges.push(Edge::new((0, 0), (w, 0))); // Top
        self.edges.push(Edge::new((w, 0), (w, h))); // Right
        self.edges.push(Edge::new((0, h), (w, h))); // Bottom
        self.edges.push(Edge::new((0, 0), (0, h))); // Left

        self.convert_edges_to_poly_map();

        self.light = Image::load("static/light.png")?;

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear();

        let (x, y) = s.mouse_pos().into();

        let (cx, cw) = if x - 254 < 0 {
            (0, (x + 255) as u32)
        } else {
            (x - 254, 511)
        };
        let (cy, ch) = if y - 254 < 0 {
            (0, (y + 255) as u32)
        } else {
            (y - 254, 511)
        };
        s.clip((cx, cy, cw, ch));

        self.draw_visibility_polygons(s)?;

        s.fill(BLUE);
        s.stroke(BLUE);
        for cell in self.cells.iter().filter(|c| c.exists) {
            s.square(cell.pos.x as i32, cell.pos.y as i32, BLOCK_SIZE + 1)?;
        }

        s.image(x - 255, y - 255, &self.light)?;

        Ok(())
    }

    fn on_mouse_pressed(&mut self, s: &mut PixState, btn: MouseButton) {
        if btn == MouseButton::Left {
            let (mx, my) = s.mouse_pos().into();
            if mx > 0 && mx <= s.width() as i32 && my > 0 && my <= s.height() as i32 {
                let i = self.get_cell_index(mx, my);
                self.cells[i].exists = !self.cells[i].exists;
                self.drawing = self.cells[i].exists;
                self.convert_edges_to_poly_map();
            }
        }
    }

    fn on_mouse_dragged(&mut self, s: &mut PixState) {
        if s.mouse_buttons().contains(&MouseButton::Left) {
            let (mx, my) = s.mouse_pos().into();
            let (px, py) = s.pmouse_pos().into();
            if mx > 0 && mx <= s.width() as i32 && my > 0 && my <= s.height() as i32 {
                if (mx, my) != (px, py) {
                    let i = self.get_cell_index(mx, my);
                    self.cells[i].exists = self.drawing;
                }
                self.convert_edges_to_poly_map();
            }
        }
    }
}

#[derive(Debug)]
struct Cell {
    pos: Vector,
    edges: [(bool, usize); 4],
    exists: bool,
}

// 0,0 -> 0,0,16,16
// 1,0 -> 16,0,32,16
impl Cell {
    pub fn new<P: Into<Vector>>(pos: P) -> Self {
        Self {
            pos: pos.into() * BLOCK_SIZE as f64,
            edges: [(false, 0); 4],
            exists: false,
        }
    }

    fn reset(&mut self) {
        self.edges = [(false, 0); 4];
    }
}

fn main() {
    let mut engine = PixEngine::create(WIDTH, HEIGHT)
        .with_title(TITLE)
        .position_centered()
        .icon("static/light.png")
        .resizable()
        .build()
        .expect("valid engine");
    let mut app = RayScene::new();
    engine.run(&mut app).expect("ran successfully");
}
