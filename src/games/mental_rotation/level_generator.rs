use super::tile::{Tile, Direction};
use js_sys::Math;

// Helper to generate random numbers
fn random_usize(max: usize) -> usize {
    (Math::random() * max as f64) as usize
}

// Random rotation: 0, 90, 180, or 270 degrees
fn random_rotation() -> i32 {
    let rand = random_usize(4);
    match rand {
        0 => 0,
        1 => 90,
        2 => 180,
        _ => 270,
    }
}

// Random boolean for reversing tile directions
fn random_bool() -> bool {
    Math::random() > 0.5
}

pub fn generate_level(level: usize) -> (Vec<Tile>, Vec<usize>, (usize, usize), (usize, usize)) {
    // Grid size is determined by level (no minimum size)
    let grid_size = level;
    
    // For very small grids (1-2), use fixed positions
    let (start_y, end_y) = if grid_size < 3 {
        (0, 0) // For tiny grids, just use a straight line
    } else {
        // Randomly select start and end Y positions
        (random_usize(grid_size), random_usize(grid_size))
    };
    
    // Define start and end positions (start on left edge, end on right edge)
    let start_pos = (0, start_y);
    let end_pos = (grid_size.saturating_sub(1), end_y); // Use saturating_sub for safety with tiny grids
    
    // Create a path from start to end
    let path = generate_path(start_pos, end_pos, grid_size);
    
    // Generate tiles along the path
    let (tiles, solution_path_tiles) = generate_tiles_from_path(path, grid_size);
    
    // Randomize tile rotations and reversals to increase difficulty
    let tiles = randomize_tiles(tiles, solution_path_tiles.clone());
    
    (tiles, solution_path_tiles, start_pos, end_pos)
}

fn generate_path(start: (usize, usize), end: (usize, usize), grid_size: usize) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    path.push(start);
    
    let mut current = start;
    
    // Create a direct path from left to right with some variety in Y position
    while current.0 < end.0 {
        // Always move east (right)
        let next_x = current.0 + 1;
        
        // Occasionally adjust Y position to make the path more interesting
        let mut next_y = current.1;
        
        // After the first move, potentially adjust Y to move toward the end Y position
        if current.0 > start.0 && current.0 < end.0 - 1 {
            if Math::random() < 0.3 {
                // Move Y toward end position
                if next_y < end.1 {
                    next_y += 1;
                } else if next_y > end.1 {
                    next_y -= 1;
                }
            }
        }
        
        // Make sure we stay within grid bounds
        next_y = next_y.min(grid_size - 1);
        
        let next = (next_x, next_y);
        path.push(next);
        current = next;
    }
    
    path
}

fn generate_tiles_from_path(path: Vec<(usize, usize)>, grid_size: usize) -> (Vec<Tile>, Vec<usize>) {
    let mut tiles = Vec::new();
    let mut solution_path_tiles = Vec::new();
    let mut path_cells = path.clone();
    
    // Process the path to create tiles
    while !path_cells.is_empty() {
        // Determine the size of this polyomino tile (1-3 cells)
        let tile_size = if path_cells.len() >= 3 {
            match random_usize(3) {
                0 => 1,
                1 => 2,
                _ => 3,
            }
        } else if path_cells.len() == 2 {
            random_usize(2) + 1
        } else {
            1
        };
        
        // Take cells for this tile
        let mut tile_cells = Vec::new();
        for _ in 0..tile_size {
            if let Some(cell) = path_cells.first() {
                tile_cells.push(*cell);
                path_cells.remove(0);
            } else {
                break;
            }
        }
        
        if !tile_cells.is_empty() {
            // Create a new tile
            let tile = Tile {
                cells: tile_cells,
                rotation: 0,
                reversed: false,
            };
            
            // Add to the tiles list
            tiles.push(tile);
            
            // This tile is part of the solution path
            solution_path_tiles.push(tiles.len() - 1);
        }
    }
    
    // Add some additional non-path tiles to make the puzzle more challenging
    add_distractor_tiles(&mut tiles, &path, grid_size);
    
    (tiles, solution_path_tiles)
}

fn add_distractor_tiles(tiles: &mut Vec<Tile>, path: &[(usize, usize)], grid_size: usize) {
    // Add "distractor" tiles that aren't part of the solution path
    // The number of distractors scales with level difficulty
    let num_distractors = (grid_size / 2).max(1);
    
    let mut occupied_cells: Vec<(usize, usize)> = path.to_vec();
    for tile in tiles.iter() {
        occupied_cells.extend(&tile.cells);
    }
    
    for _ in 0..num_distractors {
        // Try to place a distractor tile in an empty cell
        for _ in 0..10 { // Limit attempts to avoid infinite loops
            let x = random_usize(grid_size);
            let y = random_usize(grid_size);
            
            if !occupied_cells.contains(&(x, y)) {
                let mut tile_cells = vec![(x, y)];
                occupied_cells.push((x, y));
                
                // Add more cells to make a multi-cell distractor (50% chance)
                if Math::random() < 0.5 && x + 1 < grid_size && !occupied_cells.contains(&(x + 1, y)) {
                    tile_cells.push((x + 1, y));
                    occupied_cells.push((x + 1, y));
                }
                
                let tile = Tile {
                    cells: tile_cells,
                    rotation: 0,
                    reversed: false,
                };
                
                tiles.push(tile);
                break;
            }
        }
    }
}

fn randomize_tiles(mut tiles: Vec<Tile>, solution_path_tiles: Vec<usize>) -> Vec<Tile> {
    // Randomize rotations and reversals for maximum challenge
    for i in 0..tiles.len() {
        // Make sure solution path tiles require manipulation
        // This ensures we maximize the difference between best and worst case
        if solution_path_tiles.contains(&i) {
            // Always require some manipulation for solution tiles
            if random_bool() {
                // Apply rotation
                let rotation = match random_usize(3) {
                    0 => 90,
                    1 => 180,
                    _ => 270,
                };
                tiles[i].rotation = rotation;
            } else {
                // Apply reversal
                tiles[i].reversed = true;
            }
        } else {
            // For non-solution tiles, randomize completely
            tiles[i].rotation = random_rotation();
            tiles[i].reversed = random_bool();
        }
    }
    
    tiles
}
