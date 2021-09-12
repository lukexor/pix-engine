use pix_engine::prelude::*;

use crate::cell::{Cell, Direction};

#[derive(Debug, Clone)]
pub struct Maze {
    cols: u32,
    rows: u32,
    size: u32,
    cells: Vec<Cell>,
}

impl Maze {
    pub fn new(cols: u32, rows: u32, size: u32) -> Self {
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

    pub fn idx(&self, col: u32, row: u32) -> Option<usize> {
        if (0..self.cols).contains(&col) && (0..self.rows).contains(&row) {
            Some((col + row * self.cols) as usize)
        } else {
            None
        }
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    pub fn get_neighbor(&self, cell: &Cell, index: usize) -> Option<(Direction, Cell)> {
        use Direction::*;
        match index {
            0 if cell.row() > 0 => self.get(cell.col(), cell.row() - 1).map(|n| (North, n)),
            1 => self.get(cell.col() + 1, cell.row()).map(|n| (East, n)),
            2 => self.get(cell.col(), cell.row() + 1).map(|n| (South, n)),
            3 if cell.col() > 0 => self.get(cell.col() - 1, cell.row()).map(|n| (West, n)),
            _ => None,
        }
    }

    pub fn random_cell(&self) -> Cell {
        self.cells[random!(self.len())]
    }

    pub fn draw(&self, s: &mut PixState) -> PixResult<()> {
        for cell in &self.cells {
            cell.draw(s, 51)?;
        }
        self.draw_border(s)?;
        Ok(())
    }

    pub fn draw_border(&self, s: &mut PixState) -> PixResult<()> {
        s.no_fill();
        s.stroke(WHITE);
        s.rect([0, 0, self.cols * self.size + 1, self.rows * self.size + 1])?;
        Ok(())
    }

    fn get(&self, col: u32, row: u32) -> Option<Cell> {
        self.idx(col, row).map(|idx| self.cells[idx])
    }

    fn len(&self) -> usize {
        self.cells.len()
    }
}
