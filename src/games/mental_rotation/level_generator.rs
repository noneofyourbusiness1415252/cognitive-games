use super::tile::{Tile, Direction};
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

    // For level 2+, create a path from start to end
    let mut occupied = vec![vec![false; size]; size];

    // Create a path from start to end
    let mut path = Vec::new();
    let mut current = (0, mid);
    path.push(current);
    occupied[current.1][current.0] = true;

    // Generate direct path between start and end
    while current.0 < size - 1 {
        current = (current.0 + 1, current.1);
        path.push(current);
        occupied[current.1][current.0] = true;
    }

    // Create tiles along the path
    for &(x, y) in &path {
        let mut arrows = vec![Direction::East]; // Default direction
        
        // Determine arrow direction based on position in path
        if x < size - 1 {
            // If not the last position, point to the next position
            arrows = vec![Direction::East];
        } else {
            // Last position, can point anywhere
            arrows = vec![Direction::East];
        }
        
        // Create tile with randomized rotation/reversal for difficulty
        tiles.push(Tile {
            cells: vec![(x, y)],
            arrows,
            rotation: random_rotation(),
            reversed: random_bool(),
        });
    }

    // Add additional tiles for difficulty (if level > 2)
    if size > 2 {
        for _ in 0..(size-2) {
            // Find an unoccupied position for a distraction tile
            let mut x = 0;
            let mut y = 0;
            let mut found = false;
            
            for attempts in 0..10 {
                x = (Math::floor(Math::random() * size as f64) as usize).min(size - 1);
                y = (Math::floor(Math::random() * size as f64) as usize).min(size - 1);
                
                if !occupied[y][x] {
                    found = true;
                    break;
                }
            }
            
            if found {
                occupied[y][x] = true;
                tiles.push(Tile {
                    cells: vec![(x, y)],
                    arrows: vec![Direction::East],
                    rotation: random_rotation(),
                    reversed: random_bool(),
                });
            }
        }
    }

    tiles
}
