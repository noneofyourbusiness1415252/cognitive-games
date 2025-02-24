use super::tile::{Tile, Direction};
use super::grid::GridPos;

pub fn generate_level(level: usize) -> Vec<Tile> {
    let size = level;
    let mut tiles = Vec::new();
    let mid = size / 2;

    if size == 1 {
        tiles.push(Tile {
            cells: vec![(0, 0)],
            arrows: vec![Direction::East],
            rotation: 0,
            reversed: false,
        });
        return tiles;
    }

    // For level 2+, create path with proper positioning
    let start = GridPos::new(0, mid);
    let mut pos = start;

    // Create tiles to connect start to end horizontally with some vertical movement
    for x in 0..size {
        let cells = if x < size - 1 {
            vec![(x, mid), (x + 1, mid)]
        } else {
            vec![(x, mid)]
        };

        tiles.push(Tile {
            cells: cells.into_iter().map(|(x, y)| (x, y)).collect(),
            arrows: vec![Direction::East],
            rotation: 0,
            reversed: false,
        });
    }

    tiles
}
