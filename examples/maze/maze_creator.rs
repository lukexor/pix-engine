use crate::{
    cell::{Cell, Direction},
    maze::Maze,
};
use pix_engine::prelude::*;
use rand::prelude::IteratorRandom;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MazeCreator {
    current: Option<Cell>,
    visited: HashSet<usize>,
    stack: Vec<Cell>,
    completed: bool,
}

impl MazeCreator {
    pub fn new(maze: &Maze) -> Self {
        let current = maze.random_cell();
        Self {
            current: Some(current),
            visited: HashSet::new(),
            stack: vec![current],
            completed: false,
        }
    }

    pub fn step(&mut self, maze: &mut Maze) -> PixResult<()> {
        if let Some(current) = self.current {
            self.visited.insert(current.id());
            let next = self.get_random_neighbor(&current, maze);
            if let Some((direction, next)) = next {
                if let Some(cell) = maze.get_cell_mut(current.id()) {
                    cell.remove_wall(direction);
                }
                if let Some(cell) = maze.get_cell_mut(next.id()) {
                    cell.remove_wall(direction.opposite());
                }
                self.stack.push(next);
                self.current = Some(next);
            } else {
                self.current = self.stack.pop()
            }
        } else {
            self.completed = true;
        }
        Ok(())
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn draw(&self, s: &mut PixState, maze: &Maze) -> PixResult<()> {
        for cell in maze.cells().iter() {
            let color = match self.stack.last() {
                Some(current) if current.id() == cell.id() => color!(0, 155, 0),
                _ => {
                    if self.visited.contains(&cell.id()) {
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

    fn get_random_neighbor(&self, cell: &Cell, maze: &Maze) -> Option<(Direction, Cell)> {
        let mut rng = rand::thread_rng();
        cell.walls()
            .iter()
            .enumerate()
            .filter(|(_, n)| **n)
            .filter_map(|(i, _)| maze.get_neighbor(&cell, i))
            .filter(|(_, neighbor)| !self.visited.contains(&neighbor.id()))
            .choose(&mut rng)
    }
}
