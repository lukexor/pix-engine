use crate::{cell::Cell, maze::Maze};
use pix_engine::prelude::*;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AStarCell {
    cell: Cell,
    previous: Option<usize>,
    g: Scalar,
    h: Scalar,
    f: Scalar,
}

impl AStarCell {
    fn new(cell: Cell) -> Self {
        Self {
            cell,
            previous: None,
            g: Scalar::MAX,
            h: Scalar::MAX,
            f: Scalar::MAX,
        }
    }

    fn heuristic(&self, cell: &Cell) -> Scalar {
        let a = self.cell.col() - cell.col();
        let b = self.cell.row() - cell.row();
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
        match other.f.partial_cmp(&self.f) {
            Some(o) => o,
            None => std::cmp::Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AStarSolver {
    start: Cell,
    end: Cell,
    current: Option<AStarCell>,
    cells: Vec<AStarCell>,
    heap: BinaryHeap<AStarCell>,
    open_set: HashSet<usize>,
    closed_set: HashSet<usize>,
    path: Vec<AStarCell>,
    path_set: HashSet<usize>,
    completed: bool,
}

impl AStarSolver {
    pub fn new(maze: &Maze) -> Self {
        let end = maze.random_cell();
        let start = maze.random_cell();
        let cells = maze
            .cells
            .iter()
            .map(|cell| AStarCell::new(*cell))
            .collect();
        let mut current = AStarCell::new(start);
        current.g = 0.0;
        current.h = current.heuristic(&end);
        current.f = current.h;

        let mut heap = BinaryHeap::new();
        heap.push(current);

        let mut open_set = HashSet::new();
        open_set.insert(start.id);

        Self {
            start,
            end,
            current: Some(current),
            cells,
            heap,
            open_set,
            closed_set: HashSet::new(),
            path: vec![current],
            path_set: HashSet::new(),
            completed: false,
        }
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn step(&mut self, maze: &Maze) {
        // Because of our custom Ord impl, this is a min-heap
        if let Some(current) = self.heap.pop() {
            self.path.clear();
            self.path_set.clear();

            self.path.push(current);
            let mut previous = current.previous;
            while let Some(cell_id) = previous {
                let cell = self.cells[cell_id];
                self.path.push(cell);
                self.path_set.insert(cell_id);
                previous = cell.previous;
            }

            if current.cell.id == self.end.id {
                self.heap.clear();
                self.open_set.clear();
                self.completed = true;
            } else {
                self.closed_set.insert(current.cell.id);
                current
                    .cell
                    .walls
                    .iter()
                    .enumerate()
                    .filter(|(_, wall)| !*wall) // Filter only valid paths without a wall
                    .for_each(|(i, _)| {
                        if let Some(neighbor) = self.get_neighbor(&maze, &current.cell, i) {
                            if !self.closed_set.contains(&neighbor.cell.id) {
                                let tmp_g = current.g + 1.0;
                                if tmp_g < neighbor.g {
                                    let neighbor = self.update_heuristic(&neighbor, &current);
                                    if !self.open_set.contains(&neighbor.cell.id) {
                                        self.heap.push(neighbor);
                                        self.open_set.insert(neighbor.cell.id);
                                    }
                                }
                            }
                        }
                    });
            }
        } else {
            self.completed = true;
        }
    }

    pub fn draw(&self, s: &mut PixState, maze: &Maze) -> PixResult<()> {
        for cell in &maze.cells {
            if self.path_set.contains(&cell.id) {
                cell.draw(s, [0, 125, 125])?;
            } else if self.closed_set.contains(&cell.id) {
                cell.draw(s, [125, 0, 0])?;
            } else if self.open_set.contains(&cell.id) {
                cell.draw(s, [225, 125, 0])?;
            } else {
                cell.draw(s, 51)?;
            }
        }
        self.start.draw(s, [0, 155, 0])?;
        self.end.draw(s, [255, 255, 0])?;
        Ok(())
    }

    fn get_cell(&self, maze: &Maze, col: Primitive, row: Primitive) -> Option<AStarCell> {
        maze.idx(col, row).map(|idx| self.cells[idx])
    }

    fn get_neighbor(&self, maze: &Maze, cell: &Cell, index: usize) -> Option<AStarCell> {
        match index {
            0 => self.get_cell(maze, cell.col(), cell.row() - 1),
            1 => self.get_cell(maze, cell.col() + 1, cell.row()),
            2 => self.get_cell(maze, cell.col(), cell.row() + 1),
            3 => self.get_cell(maze, cell.col() - 1, cell.row()),
            _ => None,
        }
    }

    fn update_heuristic(&mut self, cell: &AStarCell, current: &AStarCell) -> AStarCell {
        let mut neighbor = &mut self.cells[cell.cell.id];
        neighbor.previous = Some(current.cell.id);
        neighbor.g = current.g + 1.0;
        neighbor.h = neighbor.heuristic(&self.end);
        neighbor.f = neighbor.g + neighbor.h;
        *neighbor
    }
}
