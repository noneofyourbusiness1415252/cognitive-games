use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn from_rotation(base_direction: Direction, rotation: i32, reversed: bool) -> Self {
        let directions = [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ];
        
        let base_index = match base_direction {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
            _ => 0, // For diagonal directions, default to East
        };
        
        // Calculate rotation index
        let rotation_steps = (rotation / 90) as usize % 4;
        let mut new_index = (base_index + rotation_steps) % 4;
        
        // Handle reversal (opposite direction)
        if reversed {
            new_index = (new_index + 2) % 4;
        }
        
        directions[new_index]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    pub cells: Vec<(usize, usize)>,
    pub rotation: i32,  // Degrees
    pub reversed: bool,
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
        // Calculate the effective direction based on rotation and reversal
        // There are multiple combinations that can result in the same effective direction
        
        // Start with East as the base direction
        let base_direction = Direction::East;
        
        // Apply rotation
        let rotated = match self.rotation {
            0 => base_direction,
            90 => base_direction.rotate_90(),
            180 => base_direction.rotate_90().rotate_90(),
            270 => base_direction.rotate_90().rotate_90().rotate_90(),
            _ => base_direction, // Default to East for invalid rotations
        };
        
        // Apply reversal if needed
        if self.reversed {
            rotated.reversed()
        } else {
            rotated
        }
    }
}
