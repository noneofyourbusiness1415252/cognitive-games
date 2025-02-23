use super::tile::{Tile, Direction};

pub fn generate_initial_tiles(level: usize) -> Vec<Tile> {
    match level {
        1 => vec![Tile {
            cells: vec![(0, 0)],
            arrows: vec![Direction::East], // Keep East since we'll rotate it
            rotation: 90,  // This makes it point South
            reversed: false,
        }],
        // Add more levels later
        _ => vec![Tile {
            cells: vec![(0, 0)],
            arrows: vec![Direction::East],
            rotation: 90,
            reversed: false,
        }],
    }
}
