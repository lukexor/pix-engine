use pix_engine::prelude::*;
use rand::prelude::IteratorRandom;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

const WIDTH: Primitive = 801;
const HEIGHT: Primitive = 601;
const SIZE: Primitive = 20;
const SCALE: f32 = 1.0;
const FOOTER: Primitive = 50;
const COLS: Primitive = WIDTH / SIZE;
const ROWS: Primitive = (HEIGHT - FOOTER) / SIZE;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell {
    id: usize,
    col: Primitive,
    row: Primitive,
    rect: Rect,
    walls: [bool; 4],
}

impl Cell {
    fn new(id: usize, col: Primitive, row: Primitive) -> Self {
        Self {
            id,
            col,
            row,
            rect: square!(col * SIZE, row * SIZE, SIZE).as_(),
            walls: [true; 4],
        }
    }

    fn col(&self) -> Primitive {
        self.col
    }

    fn row(&self) -> Primitive {
        self.row
    }

    fn remove_wall(&mut self, direction: Direction) {
        self.walls[direction as usize] = false;
    }

    fn draw(&self, s: &mut PixState, color: Color) -> PixResult<()> {
        s.fill(color);
        s.no_stroke();
        s.rect(self.rect)?;
        s.no_fill();
        s.stroke(WHITE);
        let top = self.rect.top();
        let right = self.rect.right();
        let bottom = self.rect.bottom();
        let left = self.rect.left();
        for (i, _) in self.walls.iter().enumerate().filter(|(_, n)| **n) {
            match i {
                0 => s.line([left, top, right, top])?,
                1 => s.line([right, top, right, bottom])?,
                2 => s.line([left, bottom, right, bottom])?,
                3 => s.line([left, top, left, bottom])?,
                _ => (),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Algorithm {
    AStar,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MazeMode {
    Idle,
    Creating,
    Unsolved,
    Solving(Algorithm),
    Solved,
}

#[derive(Debug, Clone)]
struct Maze {
    cols: Primitive,
    rows: Primitive,
    size: Primitive,
    cells: Vec<Cell>,
}

impl Maze {
    fn new(cols: Primitive, rows: Primitive, size: Primitive) -> Self {
        let mut cells = Vec::with_capacity((cols * rows) as usize);
        for row in 0..rows {
            // Ensure cols are added contiguously before rows
            for col in 0..cols {
                cells.push(Cell::new(cells.len(), col, row));
            }
        }
        Self {
            cols,
            rows,
            size,
            cells,
        }
    }

    fn idx(&self, col: Primitive, row: Primitive) -> Option<usize> {
        if (0..self.cols).contains(&col) && (0..self.rows).contains(&row) {
            Some((col + row * self.cols) as usize)
        } else {
            None
        }
    }

    fn get(&self, col: Primitive, row: Primitive) -> Option<Cell> {
        self.idx(col, row).map(|idx| self.cells[idx])
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn random_cell(&self) -> Cell {
        self.cells[random!(self.len())]
    }

    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        for cell in &self.cells {
            cell.draw(s, color!(51))?;
        }
        self.draw_border(s)?;
        Ok(())
    }

    fn draw_border(&self, s: &mut PixState) -> PixResult<()> {
        s.no_fill();
        s.stroke(WHITE);
        s.rect([0, 0, self.cols * self.size + 1, self.rows * self.size + 1])?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Timer {
    start: Option<Instant>,
    end: Option<Instant>,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    fn elapsed(&self) -> f32 {
        match (self.start, self.end) {
            (Some(start), Some(end)) if end > start => (end - start).as_secs_f32(),
            (Some(start), _) => (Instant::now() - start).as_secs_f32(),
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct MazeCreator {
    current: Option<Cell>,
    visited: HashSet<usize>,
    stack: Vec<Cell>,
    completed: bool,
}

impl MazeCreator {
    fn new(maze: &Maze) -> Self {
        let current = maze.random_cell();
        Self {
            current: Some(current),
            visited: HashSet::new(),
            stack: vec![current],
            completed: false,
        }
    }

    fn step(&mut self, maze: &mut Maze) {
        if let Some(current) = self.current {
            self.visited.insert(current.id);
            let next = self.get_random_neighbor(current, maze);
            if let Some((direction, next)) = next {
                maze.cells[current.id].remove_wall(direction);
                maze.cells[next.id].remove_wall(direction.opposite());
                self.stack.push(next);
                self.current = Some(next);
            } else {
                self.current = self.stack.pop()
            }
        } else {
            self.completed = true;
        }
    }

    fn get_random_neighbor(&self, cell: Cell, maze: &Maze) -> Option<(Direction, Cell)> {
        let mut rng = rand::thread_rng();
        cell.walls
            .iter()
            .enumerate()
            .filter(|(_, n)| **n)
            .filter_map(|(i, _)| match i {
                0 => maze.get(cell.col(), cell.row() - 1).map(|n| (North, n)),
                1 => maze.get(cell.col() + 1, cell.row()).map(|n| (East, n)),
                2 => maze.get(cell.col(), cell.row() + 1).map(|n| (South, n)),
                3 => maze.get(cell.col() - 1, cell.row()).map(|n| (West, n)),
                _ => None,
            })
            .filter(|&(_, neighbor)| !self.visited.contains(&neighbor.id))
            .choose(&mut rng)
    }

    fn completed(&self) -> bool {
        self.completed
    }

    fn draw(&self, s: &mut PixState, maze: &Maze) -> PixResult<()> {
        for cell in &maze.cells {
            let color = match self.stack.last() {
                Some(current) if current.id == cell.id => color!(0, 155, 0),
                _ => {
                    if self.visited.contains(&cell.id) {
                        color!(0, 50, 75)
                    } else {
                        color!(51)
                    }
                }
            };
            cell.draw(s, color)?;
        }
        maze.draw_border(s)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MazeSolver {
    start: Cell,
    end: Cell,
    current: Option<AStarCell>,
    cells: Vec<AStarCell>,
    heap: BinaryHeap<AStarCell>,
    open_set: HashSet<usize>,
    closed_set: HashSet<usize>,
    path: Vec<Cell>,
    path_set: HashSet<usize>,
    completed: bool,
}

impl MazeSolver {
    fn new(maze: &Maze) -> Self {
        let end = maze.random_cell();
        let start = maze.random_cell();
        let cells = maze
            .cells
            .iter()
            .map(|cell| AStarCell::new(*cell, &end))
            .collect();

        let mut heap = BinaryHeap::new();
        heap.push(AStarCell::new(start, &end));

        let mut open_set = HashSet::new();
        open_set.insert(start.id);

        Self {
            start,
            end,
            current: Some(AStarCell::new(start, &end)),
            cells,
            heap,
            open_set,
            closed_set: HashSet::new(),
            path: vec![start],
            path_set: HashSet::new(),
            completed: false,
        }
    }

    fn step(&mut self, maze: &Maze) {
        if let Some(current) = self.heap.pop() {
            if current.cell.id == self.end.id {
                self.heap.clear();
                self.completed = true;
            } else {
                self.closed_set.insert(current.cell.id);
                current
                    .cell
                    .walls
                    .iter()
                    // .filter_map(|n| n.map(|n| maze.cells[n]))
                    .for_each(|neighbor| {
                        // if !self.closed_set.contains(&neighbor.id) {
                        //     let tmp_g = current.g + 1.0;
                        //     if tmp_g < neighbor.g {
                        //         neighbor.previous = current;
                        //         neighbor.g = tmp_g;
                        //         neighbor.h = neighbor.heuristic(end);
                        //         neighbor.f = neighbor.g + neighbor.h;
                        //         if !self.open_set.contains(neighbor) {
                        //             self.open_set.push(neighbor)
                        //         }
                        //     }
                        // }
                    });
            }
        } else {
            self.completed = true;
        }
    }

    fn completed(&self) -> bool {
        self.completed
    }

    fn draw(&self, s: &mut PixState, maze: &Maze) -> PixResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MazeApp {
    mode: MazeMode,
    cols: Primitive,
    rows: Primitive,
    size: Primitive,
    maze: Maze,
    creator: MazeCreator,
    solver: MazeSolver,
    timer: Timer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct AStarCell {
    cell: Cell,
    previous: Option<usize>,
    g: Scalar,
    h: Scalar,
    f: Scalar,
}

impl AStarCell {
    fn new(cell: Cell, end: &Cell) -> Self {
        let mut astar_cell = Self {
            cell,
            previous: None,
            g: 0.0,
            h: 0.0,
            f: 0.0,
        };
        astar_cell.h = astar_cell.heuristic(end);
        astar_cell.f = astar_cell.h;
        astar_cell
    }

    fn heuristic(&self, cell: &Cell) -> Scalar {
        let a = self.cell.col - cell.col;
        let b = self.cell.row - cell.row;
        ((a.pow(2) + b.pow(2)) as Scalar).sqrt()
    }
}

impl Eq for AStarCell {}

impl PartialOrd for AStarCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.f.partial_cmp(&other.f) {
            Some(o) => o,
            None => std::cmp::Ordering::Less,
        }
    }
}

impl MazeApp {
    fn new(cols: Primitive, rows: Primitive, size: Primitive) -> Self {
        let maze = Maze::new(cols, rows, size);
        let creator = MazeCreator::new(&maze);
        let solver = MazeSolver::new(&maze);
        Self {
            mode: MazeMode::Idle,
            cols,
            rows,
            size,
            maze,
            creator,
            solver,
            timer: Timer::new(),
        }
    }

    fn start_create_maze(&mut self) {
        self.mode = MazeMode::Creating;
        self.maze = Maze::new(self.cols, self.rows, self.size);
        self.creator = MazeCreator::new(&self.maze);
        self.timer.start();
    }

    fn create_maze(&mut self) {
        self.maze = Maze::new(self.cols, self.rows, self.size);
        self.creator = MazeCreator::new(&self.maze);
        self.timer.start();
        while !self.creator.completed() {
            self.step_create_maze();
        }
    }

    fn start_solve_maze(&mut self, algorithm: Algorithm) {
        self.mode = MazeMode::Solving(algorithm);
        self.solver = MazeSolver::new(&self.maze);
        self.timer.start();
        while !self.solver.completed() {
            self.step_solve_astar();
        }
    }

    fn step_create_maze(&mut self) {
        self.creator.step(&mut self.maze);
        if self.creator.completed() {
            self.timer.stop();
            self.mode = MazeMode::Unsolved;
        }
    }

    fn step_solve_astar(&mut self) {
        self.solver.step(&mut self.maze);
        if self.solver.completed() {
            self.timer.stop();
            self.mode = MazeMode::Solved;
        }
    }

    fn draw(&mut self, s: &mut PixState) -> PixResult<()> {
        match self.mode {
            MazeMode::Idle => self.maze.draw(s)?,
            MazeMode::Creating | MazeMode::Unsolved => {
                self.creator.draw(s, &self.maze)?;
            }
            MazeMode::Solving(_) | MazeMode::Solved => {
                self.solver.draw(s, &self.maze)?;
            }
        }
        // TODO: Refine button interface to have `clicked()`
        if s.button([10, HEIGHT - 50, 125, 40], "Create")? {
            self.start_create_maze();
        }
        if s.button([140, HEIGHT - 50, 40, 40], ">>")? {
            self.create_maze();
        }
        if s.button([200, HEIGHT - 50, 140, 40], "Solve A*")? {
            self.start_solve_maze(Algorithm::AStar);
        }
        s.fill(GREEN);
        s.text(
            [WIDTH - 250, HEIGHT - 40],
            &format!("Elapsed: {:.3}", self.timer.elapsed()),
        )?;
        Ok(())
    }
}

impl AppState for MazeApp {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(51);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.draw(s)?;
        match self.mode {
            MazeMode::Creating => self.step_create_maze(),
            MazeMode::Solving(Algorithm::AStar) => self.step_solve_astar(),
            _ => (),
        }
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        let frame_rate = s.frame_rate();
        match event.key {
            Key::Up if frame_rate >= 60 => {
                s.clear_frame_rate();
            }
            Key::Up if frame_rate < 60 => {
                s.set_frame_rate(frame_rate + 10);
            }
            Key::Down if frame_rate > 10 => {
                s.set_frame_rate(frame_rate - 10);
            }
            _ => (),
        }
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Maze Generation")
        .with_frame_rate()
        .scale(SCALE, SCALE)
        .position_centered()
        .build();
    let mut app = MazeApp::new(COLS, ROWS, SIZE);
    engine.run(&mut app)
}
