use super::tile::{Tile, Direction};

pub fn generate_initial_tiles(level: usize) -> Vec<Tile> {
    match level {
        1 => vec![
            Tile {
                cells: vec![(0, 0)],
                arrows: vec![Direction::East],
                rotation: 0,
                reversed: false,
            }
        ],
        2 => vec![
            Tile {
                cells: vec![(0, 0), (0, 1)],
                arrows: vec![Direction::South, Direction::East],
                rotation: 0,
                reversed: false,
            }
        ],
        _ => {
            // For larger levels, create a snake-like path
            let mut tiles = Vec::new();
            let mid = level / 2;
            tiles.push(Tile {
                cells: vec![(0, mid), (1, mid)],
                arrows: vec![Direction::East, Direction::East],
                rotation: 0,
                reversed: false,
            });
            tiles
        }
    }
}
