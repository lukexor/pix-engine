use pix_engine::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}
use Direction::*;

use crate::SIZE;

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
    id: usize,
    col: u32,
    row: u32,
    rect: Rect<i32>,
    walls: [bool; 4],
}

impl Cell {
    pub fn new(id: usize, col: u32, row: u32) -> Self {
        Self {
            id,
            col,
            row,
            rect: square!((col * SIZE) as i32, (row * SIZE) as i32, SIZE as i32),
            walls: [true; 4],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn row(&self) -> u32 {
        self.row
    }

    pub fn walls(&self) -> &[bool] {
        &self.walls
    }

    pub fn remove_wall(&mut self, direction: Direction) {
        self.walls[direction as usize] = false;
    }

    pub fn draw<C: Into<Color>>(&self, s: &mut PixState, color: C) -> PixResult<()> {
        let color = color.into();
        s.fill(color);
        s.no_stroke();
        s.rect(self.rect)?;
        s.no_fill();
        s.stroke(Color::WHITE);
        let top_left = self.rect.top_left();
        let top_right = self.rect.top_right();
        let bottom_left = self.rect.bottom_left();
        let bottom_right = self.rect.bottom_right();
        for (i, _) in self.walls.iter().enumerate().filter(|(_, n)| **n) {
            match i {
                0 => s.line([top_left, top_right])?,
                1 => s.line([top_right, bottom_right])?,
                2 => s.line([bottom_left, bottom_right])?,
                3 => s.line([top_left, bottom_left])?,
                _ => (),
            }
        }
        Ok(())
    }
}
