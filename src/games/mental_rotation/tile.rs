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

    pub fn rotate_90(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::NorthEast => Direction::SouthEast,
            Direction::NorthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
        }
    }

    pub fn to_angle(&self) -> f64 {
        match self {
            Direction::North => 270.0,
            Direction::South => 90.0,
            Direction::East => 0.0,
            Direction::West => 180.0,
            Direction::NorthEast => 315.0,
            Direction::NorthWest => 225.0,
            Direction::SouthEast => 45.0,
            Direction::SouthWest => 135.0,
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

        // Rotate arrows
        for arrow in &mut self.arrows {
            *arrow = arrow.rotate_90();
        }
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
        for arrow in &mut self.arrows {
            *arrow = arrow.reversed();
        }
    }
}
