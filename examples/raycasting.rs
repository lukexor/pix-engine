use pix_engine::prelude::*;
use std::cmp::Ordering::Less;

const BLOCK_SIZE: u32 = 32;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;
const SCALE: f32 = 2.0;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    let mut engine = PixEngine::create("Window Title", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("valid engine");
    let mut app = RayScene::new();
    engine.run(&mut app).expect("ran successfully");
}

struct Edge {
    start: Vector,
    end: Vector,
}

impl Edge {
    pub fn points(&self) -> impl Iterator<Item = Vector> {
        let s = std::iter::once(self.start);
        let e = std::iter::once(self.end);
        s.chain(e)
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
}

impl RayScene {
    fn new() -> Self {
        let xcells = WIDTH / (BLOCK_SIZE as f32 * SCALE) as u32;
        let ycells = HEIGHT / (BLOCK_SIZE as f32 * SCALE) as u32;
        let mut cells = Vec::with_capacity((xcells * ycells) as usize);
        for y in 0..ycells {
            for x in 0..xcells {
                cells.push(Cell::new((x as i32, y as i32)));
            }
        }
        Self {
            cells,
            edges: Vec::new(),
            points: Vec::new(),
            polygons: Vec::new(),
            xcells: xcells as u32,
            ycells: ycells as u32,
            drawing: false,
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
    fn convert_edges<R: Into<Rect>>(&mut self, s: R, block_size: u32, pitch: i32) {
        let s = s.into();
        // Reset edges state, keeping only the window boundaries
        self.edges.truncate(4);
        for c in self.cells.iter_mut() {
            c.reset();
        }
        let width = s.w;
        let height = s.h;
        for x in 0..width {
            for y in 0..height {
                let x_off = x + s.x;
                let y_off = y + s.y;
                let i = (y_off * pitch + x_off) as usize; // This
                let n = ((y_off - 1) * pitch + x_off) as usize; // Northern neighbor
                let s = ((y_off + 1) * pitch + x_off) as usize; // Sourthern neighbor
                let w = (y_off * pitch + (x_off - 1)) as usize; // Western neighbor
                let e = (y_off * pitch + (x_off + 1)) as usize; // Eastern neighbor

                let x_off = x_off as i32;
                let y_off = y_off as i32;
                let block_size = block_size as i32;
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

        self.points = self.edges.iter().flat_map(|e| e.points()).collect();
        self.points
            .sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Less));
        self.points.dedup();
    }

    fn calc_visibility_polygons(&mut self, o: Vector) {
        self.polygons.clear();
        for p in self.points.iter() {
            // Cast three rays - one at and one off to each side
            for offset in -1..2 {
                let angle = offset as f64 / 10_000.0;
                // TODO This sub could be better served by a translate to o
                let mut r = p.copy() - o;
                r.rotate(angle);
                if let Some(intersect) = self.cast_ray(o, r) {
                    self.polygons.push((r.heading(), intersect));
                }
            }
        }

        // Could fail with NaN or Infinity
        self.polygons
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Less));
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
}

impl Stateful for RayScene {
    fn on_start(&mut self, s: &mut State) -> PixResult<bool> {
        s.set_scale(SCALE, SCALE)?;
        s.show_frame_rate(true);

        let w = (self.xcells * BLOCK_SIZE) as i32 - 1;
        let h = (self.ycells * BLOCK_SIZE) as i32 - 1;
        // Random few cells
        for _ in 0..2 {
            let i = self.get_cell_index(random!(w - 1), random!(h - 1));
            self.cells[i].exists = !self.cells[i].exists;
        }

        // Top
        self.edges.push(Edge {
            start: (0, 0).into(),
            end: (w, 0).into(),
        });
        // Right
        self.edges.push(Edge {
            start: (w, 0).into(),
            end: (w, h).into(),
        });
        // Bottom
        self.edges.push(Edge {
            start: (0, h).into(),
            end: (w, h).into(),
        });
        // Left
        self.edges.push(Edge {
            start: (0, 0).into(),
            end: (0, h).into(),
        });

        self.convert_edges(
            (0, 0, self.xcells, self.ycells),
            BLOCK_SIZE,
            self.xcells as i32,
        );
        Ok(true)
    }

    fn on_update(&mut self, s: &mut State) -> PixResult<bool> {
        s.background(DARK_GRAY);
        let (mx, my) = s.mouse_pos();
        self.calc_visibility_polygons((mx, my).into());

        if !self.polygons.is_empty() {
            s.fill(YELLOW);
            s.stroke(YELLOW);
            for i in 0..self.polygons.len() - 1 {
                let p1 = self.polygons[i].1;
                let p2 = self.polygons[i + 1].1;
                s.triangle(
                    mx,
                    my,
                    p1.x.round() as i32,
                    p1.y.round() as i32,
                    p2.x.round() as i32,
                    p2.y.round() as i32,
                )?;
            }
            // Draw last triangle, connecting back to first point.
            let p1 = self.polygons.last().unwrap().1;
            let p2 = self.polygons[0].1;
            s.triangle(
                mx,
                my,
                p1.x.round() as i32,
                p1.y.round() as i32,
                p2.x.round() as i32,
                p2.y.round() as i32,
            )?;
        }

        // s.fill(BLUE);
        // s.no_stroke();
        // for cell in self.cells.iter().filter(|c| c.exists) {
        //     s.square(
        //         cell.pos.x.round() as i32,
        //         cell.pos.y.round() as i32,
        //         BLOCK_SIZE + 1,
        //     )?;
        // }

        for e in self.edges.iter() {
            s.stroke(FIRE_BRICK);
            s.line(
                e.start.x.round() as i32,
                e.start.y.round() as i32,
                e.end.x.round() as i32,
                e.end.y.round() as i32,
            )?;
        }

        Ok(true)
    }

    fn on_mouse_pressed(&mut self, s: &mut State, btn: MouseButton) {
        if btn == MouseButton::Left {
            let (mx, my) = s.mouse_pos();
            let i = self.get_cell_index(mx, my);
            self.cells[i].exists = !self.cells[i].exists;
            self.drawing = self.cells[i].exists;
            self.convert_edges(
                (0, 0, self.xcells, self.ycells),
                BLOCK_SIZE,
                self.xcells as i32,
            );
        }
    }

    fn on_mouse_dragged(&mut self, s: &mut State) {
        if s.mouse_buttons().contains(&MouseButton::Left) {
            let (mx, my) = s.mouse_pos();
            let (px, py) = s.pmouse_pos();
            if (mx, my) != (px, py) {
                let i = self.get_cell_index(mx, my);
                self.cells[i].exists = self.drawing;
            }
            self.convert_edges(
                (0, 0, self.xcells, self.ycells),
                BLOCK_SIZE,
                self.xcells as i32,
            );
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
