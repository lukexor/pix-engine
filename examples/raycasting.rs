use pix_engine::prelude::*;
use std::{borrow::Cow, cmp::Ordering::Less};

const WIDTH: Primitive = 1000;
const HEIGHT: Primitive = 800;
const SCALE: Primitive = 1;

const BLOCK_SIZE: Primitive = 40;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

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
            pos: pos.into() * BLOCK_SIZE as Scalar,
            edges: [(false, 0); 4],
            exists: false,
        }
    }

    fn reset(&mut self) {
        self.edges = [(false, 0); 4];
    }
}

struct RayScene {
    cells: Vec<Cell>,
    edges: Vec<Line>,
    points: Vec<Point>,
    polygons: Vec<(Scalar, Point)>,
    xcells: Primitive,
    ycells: Primitive,
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
                cells.push(Cell::new([x as Scalar, y as Scalar]));
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
            light: Image::default(),
        }
    }

    fn get_cell_index(&self, x: i32, y: i32) -> usize {
        ((y / BLOCK_SIZE) * self.xcells + (x / BLOCK_SIZE)) as usize
    }

    fn exists(&self, i: usize) -> bool {
        self.cells.get(i).map(|c| c.exists).unwrap_or(false)
    }

    fn has_edge(&self, i: usize, dir: usize) -> bool {
        self.cells.get(i).map(|c| c.edges[dir].0).unwrap_or(false)
    }
    fn get_edge_index(&mut self, i: usize, dir: usize) -> PixResult<usize> {
        self.cells
            .get(i)
            .map(|c| c.edges[dir].1)
            .ok_or_else(|| PixError::Other(Cow::from("invalid cell index")))
    }
    fn get_edge_mut(&mut self, i: usize) -> PixResult<&mut Line> {
        self.edges
            .get_mut(i)
            .ok_or_else(|| PixError::Other(Cow::from("invalid edge index")))
    }

    #[allow(clippy::many_single_char_names)]
    fn convert_edges_to_poly_map(&mut self) -> PixResult<()> {
        let rect = Rect::new(0, 0, self.xcells, self.ycells);
        let pitch = self.xcells;
        let block_size = BLOCK_SIZE;
        // Reset edges state, keeping only the window boundaries
        self.edges.truncate(4);
        for c in self.cells.iter_mut() {
            c.reset();
        }
        for x in 0..rect.width {
            for y in 0..rect.height {
                let x_off = x + rect.x;
                let y_off = y + rect.y;
                let i = (y_off * pitch + x_off) as usize; // This
                let n = ((y_off - 1) * pitch + x_off) as usize; // Northern neighbor
                let s = ((y_off + 1) * pitch + x_off) as usize; // Sourthern neighbor
                let w = (y_off * pitch + (x_off - 1)) as usize; // Western neighbor
                let e = (y_off * pitch + (x_off + 1)) as usize; // Eastern neighbor

                // Cell exists, check for edges
                if self.exists(i) {
                    let x_off = x_off as Scalar;
                    let y_off = y_off as Scalar;
                    let block_size = block_size as Scalar;
                    // No western neighbor, so needs an edge
                    if x > 0 && !self.exists(w) {
                        // Can extend down from northern neighbors WEST edge
                        if self.has_edge(n, WEST) {
                            let edge_id = self.get_edge_index(n, WEST)?;
                            self.get_edge_mut(edge_id)?.end.y += block_size as Scalar;
                            self.cells[i].edges[WEST] = (true, edge_id);
                        } else {
                            // Create WEST edge extending downward
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as Scalar);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[WEST] = (true, edge_id);
                        }
                    }
                    // No eastern neighbor, so needs an edge
                    if x < rect.width && !self.exists(e) {
                        // Can extend down from northern neighbors EAST edge
                        if self.has_edge(n, EAST) {
                            let edge_id = self.get_edge_index(n, EAST)?;
                            self.get_edge_mut(edge_id)?.end.y += block_size as Scalar;
                            self.cells[i].edges[EAST] = (true, edge_id);
                        } else {
                            // Create EAST edge extending downward
                            let start =
                                vector!(x_off * block_size + block_size, y_off * block_size);
                            let end = vector!(start.x, start.y + block_size as Scalar);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[EAST] = (true, edge_id);
                        }
                    }
                    // No northern neighbor, so needs an edge
                    if y > 0 && !self.exists(n) {
                        // Can extend from western neighbors NORTH edge
                        if self.has_edge(w, NORTH) {
                            let edge_id = self.get_edge_index(w, NORTH)?;
                            self.get_edge_mut(edge_id)?.end.x += block_size as Scalar;
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        } else {
                            // Create NORTH edge extending right
                            let start = vector!(x_off * block_size, y_off * block_size);
                            let end = vector!(start.x + block_size as Scalar, start.y);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        }
                    }
                    // No southern neighbor, so needs an edge
                    if y < rect.height && !self.exists(s) {
                        // Can extend from western neighbors SOUTH edge
                        if self.has_edge(w, SOUTH) {
                            let edge_id = self.get_edge_index(w, SOUTH)?;
                            self.get_edge_mut(edge_id)?.end.x += block_size as Scalar;
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        } else {
                            // Create SOUTH edge extending right
                            let start =
                                vector!(x_off * block_size, y_off * block_size + block_size);
                            let end = vector!(start.x + block_size as Scalar, start.y);
                            let edge = Line::new(start, end);
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
        Ok(())
    }

    fn calc_visibility_polygons(&mut self, o: Point) {
        self.polygons.clear();
        for &p in self.points.iter() {
            // Cast three rays - one at and one off to each side
            for offset in -1..=1 {
                let angle = offset as Scalar / 10_000.0;
                let r = Vector::rotated(p - o, angle);
                if let Some(intersect) = self.cast_ray(o, r) {
                    self.polygons.push((r.heading(), intersect));
                    continue;
                }
            }
        }

        // Could fail with NaN or Infinity
        self.polygons
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Less));
        self.polygons
            .dedup_by(|a, b| (a.1.x - b.1.x).abs() < 0.5 && (a.1.y - b.1.y).abs() < 0.5);
    }

    fn cast_ray(&self, o: Point, r: Vector) -> Option<Point> {
        let mut intersect = None;
        let mut closest_param = Scalar::INFINITY;
        let ray: Line = Line::new(o, o + r);
        for &e in self.edges.iter() {
            if let Some((point, param)) = ray.intersects_line(e) {
                if intersect.is_none() || param < closest_param {
                    intersect = Some(point);
                    closest_param = param;
                }
            }
        }
        intersect
    }

    fn draw_visibility_polygons(&mut self, s: &mut PixState) -> PixResult<()> {
        let mouse = s.mouse_pos();
        if !rect![0, 0, s.width(), s.height()].contains_point(mouse) {
            return Ok(());
        }

        self.calc_visibility_polygons(mouse.as_());

        s.fill(WHITE);
        s.no_stroke();
        if !self.polygons.is_empty() {
            for i in 0..self.polygons.len() - 1 {
                let p1 = self.polygons[i].1;
                let p2 = self.polygons[i + 1].1;
                s.triangle([mouse.into(), p1, p2])?;
            }
            // Draw last triangle, connecting back to first point.
            // SAFETY: self.polygons has at least one element due to is_empty() check above
            let p1 = self.polygons.last().unwrap().1;
            let p2 = self.polygons[0].1;
            s.triangle([mouse.into(), p1, p2])?;
        }

        s.fill(BLACK);
        s.no_stroke();
        s.circle([mouse.x, mouse.y, 2])?;
        Ok(())
    }
}

impl AppState for RayScene {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK);
        s.scale(SCALE, SCALE)?;
        s.no_cursor();

        let w = self.xcells * BLOCK_SIZE;
        let h = self.ycells * BLOCK_SIZE;

        // Random scattered cells to start with
        for _ in 0..50 {
            let i = self.get_cell_index(random!(w), random!(h));
            self.cells[i].exists = !self.cells[i].exists;
        }

        // Screen Edges
        let w = w as Scalar;
        let h = h as Scalar;
        self.edges.push(Line::new([0.0, 0.0], [w, 0.0])); // Top
        self.edges.push(Line::new([w, 0.0], [w, h])); // Right
        self.edges.push(Line::new([0.0, h], [w, h])); // Bottom
        self.edges.push(Line::new([0.0, 0.0], [0.0, h])); // Left

        self.convert_edges_to_poly_map()?;

        self.light = s.create_image_from_file(DEFAULT_ASSET_DIR.join("light.png"))?;
        s.image_tint(color![255, 255, 153]);

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        let mouse = s.mouse_pos();

        let (cx, cw) = if mouse.x - 254 < 0 {
            (0, mouse.x + 255)
        } else {
            (mouse.x - 254, 511)
        };
        let (cy, ch) = if mouse.y - 254 < 0 {
            (0, mouse.y + 255)
        } else {
            (mouse.y - 254, 511)
        };
        s.clip([cx, cy, cw, ch]);

        self.draw_visibility_polygons(s)?;

        s.fill(BLUE);
        let mut in_cell = None;
        for cell in self.cells.iter().filter(|c| c.exists) {
            let sq = square![cell.pos, BLOCK_SIZE as Scalar];
            if sq.contains_point(mouse.as_::<Scalar>()) {
                in_cell = Some(cell);
                break;
            }
            s.square(sq)?;
        }

        s.blend_mode(BlendMode::Mod);
        s.image([mouse.x - 255, mouse.y - 255], &self.light)?;
        s.blend_mode(BlendMode::None);

        if let Some(cell) = in_cell {
            s.clear();
            s.square([cell.pos.x, cell.pos.y, BLOCK_SIZE as Scalar])?;
            s.fill(YELLOW);
            s.circle([mouse.x, mouse.y, 2])?;
        }

        Ok(())
    }

    fn on_mouse_pressed(&mut self, s: &mut PixState, btn: Mouse, pos: Point<i32>) -> PixResult<()> {
        if btn == Mouse::Left && rect![0, 0, s.width(), s.height()].contains_point(pos) {
            let i = self.get_cell_index(pos.x, pos.y);
            self.cells[i].exists = !self.cells[i].exists;
            self.drawing = self.cells[i].exists;
            self.convert_edges_to_poly_map()?;
        }
        Ok(())
    }

    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        if s.mouse_buttons().contains(&Mouse::Left) {
            let m = s.mouse_pos();
            let pm = s.pmouse_pos();
            if rect![0, 0, s.width(), s.height()].contains_point(m) {
                if m != pm {
                    let i = self.get_cell_index(m.x, m.y);
                    self.cells[i].exists = self.drawing;
                }
                self.convert_edges_to_poly_map()?;
            }
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("2D Raycasting")
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .resizable()
        .icon(DEFAULT_ASSET_DIR.join("light.png"))
        .build();
    let mut app = RayScene::new();
    engine.run(&mut app)
}
