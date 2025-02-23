use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    pub cells: Vec<(usize, usize)>,
    pub arrows: Vec<Direction>,
    pub rotation: u16,  // Changed from u8 to u16
    pub reversed: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn reversed(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}

impl Tile {
    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
        
        // Create a new Vec to store rotated positions
        let size = self.cells.len();
        let mut new_cells = Vec::with_capacity(size);
        
        // Rotate each cell's position
        for (x, y) in &self.cells {
            new_cells.push((*y, size - 1 - *x));
        }
        
        self.cells = new_cells;
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
        for arrow in &mut self.arrows {
            *arrow = arrow.reversed();
        }
    }
}
