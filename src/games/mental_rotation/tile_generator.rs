use super::tile::{Tile, Direction};

pub fn generate_initial_tiles(grid_size: usize) -> Vec<Tile> {
    let mut tiles = Vec::new();
    
    match grid_size {
        1 => {
            // Level 1: Single 1x1 cell with east-pointing arrow
            tiles.push(Tile {
                cells: vec![(0, 0)],
                arrows: vec![Direction::East],
                rotation: 0,
                reversed: false,
            });
        },
        2 => {
            // Level 2: 2x2 grid with path requiring rotation
            tiles.push(Tile {
                cells: vec![(0, 0), (0, 1)],
                arrows: vec![Direction::South, Direction::East],
                rotation: 0,
                reversed: false,
            });
            tiles.push(Tile {
                cells: vec![(1, 1)],
                arrows: vec![Direction::East],
                rotation: 0,
                reversed: false,
            });
        },
        3 => {
            // Level 3: 3x3 grid with slightly more complex path
            tiles.push(Tile {
                cells: vec![(0, 1), (1, 1)],
                arrows: vec![Direction::East, Direction::East],
                rotation: 0,
                reversed: false,
            });
            tiles.push(Tile {
                cells: vec![(2, 1)],
                arrows: vec![Direction::East],
                rotation: 0,
                reversed: false,
            });
        },
        // Add more levels as needed
        _ => ()
    }
    
    tiles
}
