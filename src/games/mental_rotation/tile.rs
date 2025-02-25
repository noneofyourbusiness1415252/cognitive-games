use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    pub cells: Vec<(usize, usize)>,
    pub arrows: Vec<Direction>,
    pub rotation: i32,  // Degrees
    pub reversed: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
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
    pub fn reversed(self) -> Self {
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

    pub fn rotate_90(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West, 
            Direction::West => Direction::North,
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    pub fn angle(self) -> f64 {
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
        // Remove coordinate transformation - let CSS handle visual rotation
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
        // Remove arrow reversal since we handle it in get_effective_direction
    }

    pub fn get_effective_direction(&self) -> Direction {
        // Convert rotation to number of 90-degree turns
        let turns = ((self.rotation % 360) / 90) as usize;
        
        // Apply rotations
        let mut dir = self.arrows[0];
        for _ in 0..turns {
            dir = dir.rotate_90();
        }
        
        // Apply reversal if needed
        if self.reversed {
            dir.reversed()
        } else {
            dir
        }
    }
}
