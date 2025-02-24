use super::tile::{Tile, Direction};
use super::grid::GridPos;
use js_sys::Math;

fn random_rotation() -> i32 {
    (Math::floor(Math::random() * 4.0) as i32) * 90
}

fn random_bool() -> bool {
    Math::random() >= 0.5
}

pub fn generate_level(level: usize) -> Vec<Tile> {
    let size = level;
    let mut tiles = Vec::new();
    let mid = size / 2;

    // Handle level 1
    if size == 1 {
        tiles.push(Tile {
            cells: vec![(0, 0)],
            arrows: vec![Direction::East],
            rotation: 270,  // Start pointing down for maximum moves
            reversed: true, // Start reversed for extra move
        });
        return tiles;
    }

    // For level 2+, create non-overlapping tiles
    let mut occupied = vec![vec![false; size]; size];

    // Start tile - always start in wrong orientation
    let start_tile = Tile {
        cells: vec![(0, mid)],
        arrows: vec![Direction::East],
        rotation: 270,  // Start pointing down
        reversed: true, // Start reversed
    };
    occupied[0][mid] = true;
    tiles.push(start_tile);

    // Middle tiles - place in zigzag pattern for maximum rotations
    for x in 1..size-1 {
        let y = if x % 2 == 0 { mid } else { mid.saturating_sub(1) };
        if !occupied[x][y] {
            let tile = Tile {
                cells: vec![(x, y)],
                arrows: vec![Direction::East],
                rotation: if x % 2 == 0 { 90 } else { 270 },
                reversed: x % 2 == 1,
            };
            occupied[x][y] = true;
            tiles.push(tile);
        }
    }

    // End tile
    let end_tile = Tile {
        cells: vec![(size-1, mid)],
        arrows: vec![Direction::East],
        rotation: 90,  // Point down initially
        reversed: true,
    };
    occupied[size-1][mid] = true;
    tiles.push(end_tile);

    tiles
}
