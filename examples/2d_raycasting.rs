use pix_engine::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const SCALE: u32 = 1;

const BLOCK_SIZE: u32 = 40;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

const LIGHT: &[u8] = include_bytes!("light.png");

#[derive(Debug)]
struct Cell {
    pos: Point<i32>,
    edges: [(bool, usize); 4],
    exists: bool,
}

// 0,0 -> 0,0,16,16
// 1,0 -> 16,0,32,16
impl Cell {
    pub fn new<P: Into<Point<i32>>>(pos: P) -> Self {
        Self {
            pos: pos.into() * BLOCK_SIZE as i32,
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
    edges: Vec<Line<i32>>,
    points: Vec<Point<i32>>,
    polygons: Vec<(f64, Point<i32>)>,
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
                cells.push(Cell::new([x as i32, y as i32]));
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

    fn get_cell_index(&self, x: u32, y: u32) -> usize {
        ((y / BLOCK_SIZE) * self.xcells + (x / BLOCK_SIZE)) as usize
    }

    fn exists(&self, i: usize) -> bool {
        self.cells.get(i).map(|c| c.exists).unwrap_or(false)
    }

    fn has_edge(&self, i: usize, dir: usize) -> bool {
        self.cells.get(i).map(|c| c.edges[dir].0).unwrap_or(false)
    }
    fn get_edge_index(&mut self, i: usize, dir: usize) -> usize {
        self.cells[i].edges[dir].1
    }
    fn get_edge_mut(&mut self, i: usize) -> &mut Line<i32> {
        &mut self.edges[i]
    }

    #[allow(clippy::many_single_char_names)]
    fn convert_edges_to_poly_map(&mut self) -> PixResult<()> {
        let rect = Rect::new(0, 0, self.xcells as i32, self.ycells as i32);
        let pitch = self.xcells as i32;
        let block_size = BLOCK_SIZE as i32;
        // Reset edges state, keeping only the window boundaries
        self.edges.truncate(4);
        for c in self.cells.iter_mut() {
            c.reset();
        }
        for x in 0..rect.width() as i32 {
            for y in 0..rect.height() as i32 {
                let x_off = x + rect.x();
                let y_off = y + rect.y();
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
                            let edge = self.get_edge_mut(edge_id);
                            edge.set_end([edge.end().x(), edge.end().y() + block_size]);
                            self.cells[i].edges[WEST] = (true, edge_id);
                        } else {
                            // Create WEST edge extending downward
                            let start = point!(x_off * block_size, y_off * block_size);
                            let end = point!(start.x(), start.y() + block_size);
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[WEST] = (true, edge_id);
                        }
                    }
                    // No eastern neighbor, so needs an edge
                    if x < rect.width() && !self.exists(e) {
                        // Can extend down from northern neighbors EAST edge
                        if self.has_edge(n, EAST) {
                            let edge_id = self.get_edge_index(n, EAST);
                            let edge = self.get_edge_mut(edge_id);
                            edge.set_end([edge.end().x(), edge.end().y() + block_size]);
                            self.cells[i].edges[EAST] = (true, edge_id);
                        } else {
                            // Create EAST edge extending downward
                            let start = point!(x_off * block_size + block_size, y_off * block_size);
                            let end = point!(start.x(), start.y() + block_size);
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
                            let edge_id = self.get_edge_index(w, NORTH);
                            let edge = self.get_edge_mut(edge_id);
                            edge.set_end([edge.end().x() + block_size, edge.end().y()]);
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        } else {
                            // Create NORTH edge extending right
                            let start = point!(x_off * block_size, y_off * block_size);
                            let end = point!(start.x() + block_size, start.y());
                            let edge = Line::new(start, end);
                            let edge_id = self.edges.len();
                            self.edges.push(edge);
                            self.cells[i].edges[NORTH] = (true, edge_id);
                        }
                    }
                    // No southern neighbor, so needs an edge
                    if y < rect.height() && !self.exists(s) {
                        // Can extend from western neighbors SOUTH edge
                        if self.has_edge(w, SOUTH) {
                            let edge_id = self.get_edge_index(w, SOUTH);
                            let edge = self.get_edge_mut(edge_id);
                            edge.set_end([edge.end().x() + block_size, edge.end().y()]);
                            self.cells[i].edges[SOUTH] = (true, edge_id);
                        } else {
                            // Create SOUTH edge extending right
                            let start = point!(x_off * block_size, y_off * block_size + block_size);
                            let end = point!(start.x() + block_size, start.y());
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
            self.points.push(edge.start());
            self.points.push(edge.end());
        }
        self.points.sort_unstable();
        self.points.dedup();
        Ok(())
    }

    #[allow(clippy::many_single_char_names)]
    fn calc_visibility_polygons(&mut self, o: Point<i32>) {
        self.polygons.clear();
        for &p in self.points.iter() {
            let v = (p - o).as_::<f64>();
            // Cast three rays - one at and one off to each side
            for offset in -1..=1 {
                let angle = offset as f64 / 10_000.0;
                let r = Vector::rotated(v, angle);
                if let Some(intersect) = self.cast_ray(o, r) {
                    let [x, y] = intersect.as_array();
                    let intersect = point!(x.round() as i32, y.round() as i32);
                    self.polygons.push((r.heading(), intersect));
                    continue;
                }
            }
        }

        self.polygons
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Less));
        self.polygons
            .dedup_by(|a, b| (a.1.x() - b.1.x()).abs() < 1 && (a.1.y() - b.1.y()).abs() < 1);
    }

    fn cast_ray(&self, o: Point<i32>, r: Vector<f64>) -> Option<Point<f64>> {
        let mut intersect = None;
        let mut closest_param = f64::INFINITY;
        let o = o.as_::<f64>();
        let ray = Line::new(o, o + r);
        for &e in self.edges.iter() {
            if let Some((point, param)) = ray.intersects(e.as_::<f64>()) {
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
        if !rect![0, 0, s.width()? as i32, s.height()? as i32].contains(mouse) {
            return Ok(());
        }

        self.calc_visibility_polygons(mouse);

        s.fill(Color::WHITE);
        s.stroke(None);
        if !self.polygons.is_empty() {
            for i in 0..self.polygons.len() - 1 {
                let p1 = self.polygons[i].1;
                let p2 = self.polygons[i + 1].1;
                s.triangle([mouse, p1, p2])?;
            }
            // Draw last triangle, connecting back to first point.
            let p1 = self.polygons[self.polygons.len() - 1].1;
            let p2 = self.polygons[0].1;
            s.triangle([mouse, p1, p2])?;
        }

        s.fill(Color::BLACK);
        s.stroke(None);
        s.circle([mouse.x(), mouse.y(), 2])?;
        Ok(())
    }
}

impl AppState for RayScene {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(Color::BLACK);
        s.scale(SCALE as f32, SCALE as f32)?;
        s.cursor(None)?;

        let w = self.xcells * BLOCK_SIZE;
        let h = self.ycells * BLOCK_SIZE;

        // Random scattered cells to start with
        for _ in 0..50 {
            let i = self.get_cell_index(random!(w), random!(h));
            self.cells[i].exists = !self.cells[i].exists;
        }

        // Screen Edges
        let w = w as i32;
        let h = h as i32;
        self.edges.push(Line::new([0, 0], [w, 0])); // Top
        self.edges.push(Line::new([w, 0], [w, h])); // Right
        self.edges.push(Line::new([0, h], [w, h])); // Bottom
        self.edges.push(Line::new([0, 0], [0, h])); // Left

        self.convert_edges_to_poly_map()?;

        self.light = Image::from_read(LIGHT)?;
        s.image_tint(color![255, 255, 153]);

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;
        let mouse = s.mouse_pos();

        if !s.mouse_down(Mouse::Left) {
            let (cx, cw) = if mouse.x() - 254 < 0 {
                (0, mouse.x() + 255)
            } else {
                (mouse.x() - 254, 511)
            };
            let (cy, ch) = if mouse.y() - 254 < 0 {
                (0, mouse.y() + 255)
            } else {
                (mouse.y() - 254, 511)
            };
            s.clip(rect![cx, cy, cw, ch])?;

            self.draw_visibility_polygons(s)?;
        } else {
            s.clip(None)?;
        }

        s.fill(Color::BLUE);
        let mut in_cell = None;
        for cell in self.cells.iter().filter(|c| c.exists) {
            let sq = square![cell.pos, BLOCK_SIZE as i32];
            if sq.contains(mouse) {
                in_cell = Some(cell);
            }
            s.square(sq)?;
        }

        if !s.mouse_down(Mouse::Left) {
            s.blend_mode(BlendMode::Mod);
            s.image(&self.light, [mouse.x() - 255, mouse.y() - 255])?;
            s.blend_mode(BlendMode::None);
        }

        if let Some(cell) = in_cell {
            s.square([cell.pos.x(), cell.pos.y(), BLOCK_SIZE as i32])?;
            s.fill(Color::YELLOW);
            s.circle([mouse.x(), mouse.y(), 2])?;
        }

        Ok(())
    }

    fn on_mouse_pressed(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: Point<i32>,
    ) -> PixResult<bool> {
        if btn == Mouse::Left && rect![0, 0, s.width()? as i32, s.height()? as i32].contains(pos) {
            let i = self.get_cell_index(pos.x() as u32, pos.y() as u32);
            self.cells[i].exists = !self.cells[i].exists;
            self.drawing = self.cells[i].exists;
            self.convert_edges_to_poly_map()?;
        }
        Ok(false)
    }

    fn on_mouse_dragged(
        &mut self,
        s: &mut PixState,
        pos: Point<i32>,
        rel_pos: Point<i32>,
    ) -> PixResult<bool> {
        if s.mouse_buttons().contains(&Mouse::Left) {
            let within_window = rect![0, 0, s.width()? as i32, s.height()? as i32].contains(pos);
            if within_window && (rel_pos.x() > 0 || rel_pos.y() > 0) {
                let i = self.get_cell_index(pos.x() as u32, pos.y() as u32);
                self.cells[i].exists = self.drawing;
            }
            self.convert_edges_to_poly_map()?;
        }
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("2D Raycasting")
        .with_frame_rate()
        .icon(Image::from_read(LIGHT)?)
        .build()?;
    let mut app = RayScene::new();
    engine.run(&mut app)
}
