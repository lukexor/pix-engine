use std::{thread, time::Duration};

use pix_engine::prelude::*;

const TITLE: &str = "Maze";
const WIDTH: u32 = 300;
const HEIGHT: u32 = 240;
const SIZE: i32 = 3;
const SCALE: f32 = 4.0;

const PATH_N: i32 = 0x01;
const PATH_E: i32 = 0x02;
const PATH_S: i32 = 0x04;
const PATH_W: i32 = 0x08;
const VISITED: i32 = 0x10;

struct Maze {
    width: i32,
    height: i32,
    maze: Vec<i32>,
    neighbors: Vec<i32>,
    size: usize,
    visited: usize,
    stack: Vec<(i32, i32)>,
}

impl Maze {
    fn new() -> Self {
        let width: i32 = WIDTH as i32 / (SIZE + 1);
        let height: i32 = HEIGHT as i32 / (SIZE + 1);
        let size = (width * height) as usize;
        let mut maze = vec![0; size];
        let mut stack = Vec::with_capacity(size);

        let start_x = random!(width);
        let start_y = random!(height);
        stack.push((start_x, start_y));
        maze[(start_y * width + start_x) as usize] = VISITED;

        Self {
            width,
            height,
            maze,
            neighbors: Vec::with_capacity(4),
            size,
            visited: 1,
            stack,
        }
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn offset(&self, x: i32, y: i32) -> usize {
        match self.stack.last() {
            Some(top) => ((top.1 + y) * self.width + top.0 + x) as usize,
            None => 0,
        }
    }

    fn has_neighbors(&mut self) -> bool {
        self.neighbors.clear();
        if let Some(top) = self.stack.last() {
            // println!("top: {:?}", top);
            let north = self.offset(0, -1);
            let east = self.offset(1, 0);
            let south = self.offset(0, 1);
            let west = self.offset(-1, 0);
            if top.1 > 0 && (self.maze[north] & VISITED) == 0 {
                // println!("north: {:?}", self.maze[north]);
                self.neighbors.push(0);
            }
            if top.0 < self.width - 1 && (self.maze[east] & VISITED) == 0 {
                // println!("east: {:?}", self.maze[east]);
                self.neighbors.push(1);
            }
            if top.1 < self.height - 1 && (self.maze[south] & VISITED) == 0 {
                // println!("south: {:?}", self.maze[south]);
                self.neighbors.push(2);
            }
            if top.0 > 0 && (self.maze[west] & VISITED) == 0 {
                // println!("west: {:?}", self.maze[west]);
                self.neighbors.push(3);
            }
        }
        // println!("has: {}", !self.neighbors.is_empty());
        return !self.neighbors.is_empty();
    }

    fn visit_neighbor(&mut self) {
        let (x, y) = match self.stack.last() {
            Some(top) => (top.0, top.1),
            None => (0, 0),
        };
        let current = self.offset(0, 0);
        let next = self.neighbors[random!(self.neighbors.len())];
        // println!("visiting: {}", next);
        match next {
            0 => {
                let north = self.offset(0, -1);
                self.maze[north] |= VISITED | PATH_S;
                self.maze[current] |= PATH_N;
                self.stack.push((x, y - 1));
            }
            // East
            1 => {
                let east = self.offset(1, 0);
                self.maze[east] |= VISITED | PATH_W;
                self.maze[current] |= PATH_E;
                self.stack.push((x + 1, y));
            }
            // South
            2 => {
                let south = self.offset(0, 1);
                self.maze[south] |= VISITED | PATH_N;
                self.maze[current] |= PATH_S;
                self.stack.push((x, y + 1));
            }
            // West
            3 => {
                let west = self.offset(-1, 0);
                self.maze[west] |= VISITED | PATH_E;
                self.maze[current] |= PATH_W;
                self.stack.push((x - 1, y));
            }
            _ => unreachable!("more than max neighbors"),
        }
        self.visited += 1;
    }
}

impl AppState for Maze {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        thread::sleep(Duration::from_millis(10));
        if self.visited < self.size {
            if self.has_neighbors() {
                self.visit_neighbor();
            } else {
                self.stack.pop();
            }
        }

        s.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.maze[self.idx(x, y)];
                if cell & VISITED > 0 {
                    s.fill(WHITE);
                } else {
                    s.fill(BLUE);
                }
                s.square(x * (SIZE + 1), y * (SIZE + 1), SIZE as u32)?;
                if cell & PATH_S > 0 {
                    s.rect(x * (SIZE + 1), y * (SIZE + 1) + SIZE, SIZE as u32, 1)?;
                }
                if cell & PATH_E > 0 {
                    s.rect(x * (SIZE + 1) + SIZE, y * (SIZE + 1), 1, SIZE as u32)?;
                }
            }
        }

        if let Some(top) = self.stack.last() {
            s.fill(GREEN);
            s.square(top.0 * (SIZE + 1), top.1 * (SIZE + 1), SIZE as u32)?;
        }
        Ok(())
    }
}

pub fn main() {
    let mut engine = PixEngine::create(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .scale(SCALE, SCALE)
        .position_centered()
        .build()
        .expect("valid engine");

    let mut app = Maze::new();

    engine.run(&mut app).expect("ran successfully");
}
