use astar_solver::AStarSolver;
use maze::Maze;
use maze_creator::MazeCreator;
use pix_engine::prelude::*;
use timer::Timer;

mod astar_solver;
mod cell;
mod maze;
mod maze_creator;
mod timer;

const WIDTH: Primitive = 801;
const HEIGHT: Primitive = 601;
const SIZE: Primitive = 20;
const SCALE: f32 = 1.0;
const FOOTER: Primitive = 50;
const COLS: Primitive = WIDTH / SIZE;
const ROWS: Primitive = (HEIGHT - FOOTER) / SIZE;

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
struct MazeApp {
    mode: MazeMode,
    cols: Primitive,
    rows: Primitive,
    size: Primitive,
    maze: Maze,
    creator: MazeCreator,
    solver: AStarSolver,
    timer: Timer,
}

impl MazeApp {
    fn new(cols: Primitive, rows: Primitive, size: Primitive) -> Self {
        let maze = Maze::new(cols, rows, size);
        let creator = MazeCreator::new(&maze);
        let solver = AStarSolver::new(&maze);
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
        if let MazeMode::Idle | MazeMode::Creating = self.mode {
            self.create_maze();
        }
        self.mode = MazeMode::Solving(algorithm);
        self.solver = AStarSolver::new(&self.maze);
        self.timer.start();
    }

    fn solve_maze(&mut self) {
        if let MazeMode::Idle | MazeMode::Creating = self.mode {
            self.create_maze();
        }
        self.solver = AStarSolver::new(&self.maze);
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
        self.solver.step(&self.maze);
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
        if s.button([345, HEIGHT - 50, 40, 40], ">>")? {
            self.solve_maze();
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
