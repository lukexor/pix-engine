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

const WIDTH: u32 = 801;
const HEIGHT: u32 = 601;
const SIZE: u32 = 20;
const FOOTER: u32 = 50;
const COLS: u32 = WIDTH / SIZE;
const ROWS: u32 = (HEIGHT - FOOTER) / SIZE;

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
    cols: u32,
    rows: u32,
    maze: Maze,
    creator: MazeCreator,
    solver: AStarSolver,
    timer: Timer,
}

impl MazeApp {
    fn new(cols: u32, rows: u32) -> Self {
        let maze = Maze::new(cols, rows);
        let creator = MazeCreator::new(&maze);
        let solver = AStarSolver::new(&maze);
        Self {
            mode: MazeMode::Idle,
            cols,
            rows,
            maze,
            creator,
            solver,
            timer: Timer::new(),
        }
    }

    fn start_create_maze(&mut self) {
        self.mode = MazeMode::Creating;
        self.maze = Maze::new(self.cols, self.rows);
        self.creator = MazeCreator::new(&self.maze);
        self.timer.start();
    }

    fn create_maze(&mut self) -> Result<()> {
        self.maze = Maze::new(self.cols, self.rows);
        self.creator = MazeCreator::new(&self.maze);
        self.timer.start();
        while !self.creator.completed() {
            self.step_create_maze()?;
        }
        Ok(())
    }

    fn step_create_maze(&mut self) -> Result<()> {
        self.creator.step(&mut self.maze)?;
        if self.creator.completed() {
            self.timer.stop();
            self.mode = MazeMode::Unsolved;
        }
        Ok(())
    }

    fn start_solve_maze(&mut self, algorithm: Algorithm) -> Result<()> {
        if let MazeMode::Idle | MazeMode::Creating = self.mode {
            self.create_maze()?;
        }
        self.mode = MazeMode::Solving(algorithm);
        self.solver = AStarSolver::new(&self.maze);
        self.timer.start();
        Ok(())
    }

    fn solve_maze(&mut self) -> Result<()> {
        if let MazeMode::Idle | MazeMode::Creating = self.mode {
            self.create_maze()?;
        }
        self.solver = AStarSolver::new(&self.maze);
        self.timer.start();
        while !self.solver.completed() {
            self.step_solve_astar();
        }
        Ok(())
    }

    fn step_solve_astar(&mut self) {
        self.solver.step(&self.maze);
        if self.solver.completed() {
            self.timer.stop();
            self.mode = MazeMode::Solved;
        }
    }

    fn draw(&mut self, s: &mut PixState) -> Result<()> {
        match self.mode {
            MazeMode::Idle => self.maze.draw(s)?,
            MazeMode::Creating | MazeMode::Unsolved => {
                self.creator.draw(s, &self.maze)?;
            }
            MazeMode::Solving(_) | MazeMode::Solved => {
                self.solver.draw(s, &self.maze)?;
            }
        }
        let h = HEIGHT as i32;
        s.set_cursor_pos([10, h - 45]);
        s.fill(Color::WHITE);
        s.stroke(None);
        if s.button("Create")? {
            self.start_create_maze();
        }
        s.same_line(None);
        if s.button(">>##1")? {
            self.create_maze()?;
        }
        s.same_line(None);
        if s.button("Solve A*")? {
            self.start_solve_maze(Algorithm::AStar)?;
        }
        s.same_line(None);
        if s.button(">>##2")? {
            self.solve_maze()?;
        }
        s.push();
        s.fill(s.theme().colors.secondary);
        let rate = s.target_frame_rate().unwrap_or(60);
        s.same_line([10, -5]);
        s.text(format!(
            "Target FPS: {}\nElapsed: {:.3}",
            rate,
            self.timer.elapsed()
        ))?;
        s.pop();
        s.set_cursor_pos([s.width()? as i32 - 30, s.height()? as i32 - 40]);
        s.help_marker(
            "V: Toggle VSync\n\
            Up Arrow: Increase target frame rate.\n\
            Down Arrow: Decrease target frame rate.",
        )?;
        Ok(())
    }
}

impl AppState for MazeApp {
    fn on_update(&mut self, s: &mut PixState) -> Result<()> {
        s.clear()?;

        self.draw(s)?;
        match self.mode {
            MazeMode::Creating => self.step_create_maze()?,
            MazeMode::Solving(Algorithm::AStar) => self.step_solve_astar(),
            _ => (),
        }
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
        let frame_rate = s.target_frame_rate().unwrap_or(60);
        match event.key {
            Key::Up if frame_rate >= 60 => {
                s.frame_rate(None);
            }
            Key::Up if frame_rate < 60 => {
                s.frame_rate(frame_rate + 10);
            }
            Key::Down if frame_rate > 10 => {
                s.frame_rate(frame_rate - 10);
            }
            _ => (),
        }
        Ok(false)
    }
}

pub fn main() -> Result<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Maze Generation & A* Search")
        .with_frame_rate()
        .target_frame_rate(60)
        .build()?;
    let mut app = MazeApp::new(COLS, ROWS);
    engine.run(&mut app)
}
