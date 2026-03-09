use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North, South, East, West,
    NorthEast, NorthWest, SouthEast, SouthWest,
}

impl Direction {
    pub fn rotate_cw(self) -> Self {
        match self {
            Self::North => Self::East, Self::East => Self::South,
            Self::South => Self::West, Self::West => Self::North,
            Self::NorthEast => Self::SouthEast, Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest, Self::NorthWest => Self::NorthEast,
        }
    }

    pub fn reversed(self) -> Self {
        match self {
            Self::North => Self::South, Self::South => Self::North,
            Self::East => Self::West, Self::West => Self::East,
            Self::NorthEast => Self::SouthWest, Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast, Self::SouthEast => Self::NorthWest,
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::East => "arrow pointing-right",
            Self::South => "arrow pointing-down",
            Self::West => "arrow pointing-left",
            Self::North => "arrow pointing-up",
            Self::SouthEast => "arrow pointing-southeast",
            Self::SouthWest => "arrow pointing-southwest",
            Self::NorthEast => "arrow pointing-northeast",
            Self::NorthWest => "arrow pointing-northwest",
        }
    }

    pub fn delta(self) -> Option<(i32, i32)> {
        match self {
            Self::East => Some((1, 0)), Self::West => Some((-1, 0)),
            Self::South => Some((0, 1)), Self::North => Some((0, -1)),
            _ => None,
        }
    }

    pub fn components(self) -> Option<(Direction, Direction)> {
        match self {
            Self::NorthEast => Some((Self::North, Self::East)),
            Self::NorthWest => Some((Self::North, Self::West)),
            Self::SouthEast => Some((Self::South, Self::East)),
            Self::SouthWest => Some((Self::South, Self::West)),
            _ => None,
        }
    }
}

pub fn direction_between(from: (usize, usize), to: (usize, usize)) -> Direction {
    match (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32) {
        (1, 0) => Direction::East,  (-1, 0) => Direction::West,
        (0, 1) => Direction::South, (0, -1) => Direction::North,
        _ => Direction::East,
    }
}

pub fn diagonal_for_turn(in_dir: Direction, out_dir: Direction) -> Direction {
    match (in_dir, out_dir) {
        (Direction::East, Direction::South) | (Direction::South, Direction::East) => Direction::SouthEast,
        (Direction::East, Direction::North) | (Direction::North, Direction::East) => Direction::NorthEast,
        (Direction::West, Direction::South) | (Direction::South, Direction::West) => Direction::SouthWest,
        (Direction::West, Direction::North) | (Direction::North, Direction::West) => Direction::NorthWest,
        _ => out_dir,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    pub cells: Vec<(usize, usize)>,
    pub arrows: Vec<Direction>,
    pub is_obstacle: bool,
}

impl Tile {
    /// Compute cells after 90° CW rotation (top-left anchor preserved).
    pub fn rotated_cells(&self) -> Vec<(usize, usize)> {
        if self.cells.is_empty() { return vec![]; }
        let min_x = self.cells.iter().map(|c| c.0).min().unwrap();
        let min_y = self.cells.iter().map(|c| c.1).min().unwrap();
        let max_y = self.cells.iter().map(|c| c.1).max().unwrap();
        let h = max_y - min_y;
        self.cells.iter().map(|&(x, y)| {
            (min_x + h - (y - min_y), min_y + (x - min_x))
        }).collect()
    }

    /// Physically rotate 90° CW: move cells and rotate arrows.
    pub fn rotate_cw(&mut self) {
        self.cells = self.rotated_cells();
        for a in &mut self.arrows { *a = a.rotate_cw(); }
    }

    pub fn reverse_arrows(&mut self) {
        for a in &mut self.arrows { *a = a.reversed(); }
    }
}
