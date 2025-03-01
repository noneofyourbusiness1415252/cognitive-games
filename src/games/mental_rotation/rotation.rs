use super::tile::Tile;

pub fn rotate_coordinates(cells: &[(usize, usize)], rotation: i32) -> Vec<(usize, usize)> {
    // Convert coordinates to signed ints for safe arithmetic
    let cells_i32: Vec<(i32, i32)> = cells.iter()
        .map(|&(x, y)| (x as i32, y as i32))
        .collect();
    
    // Find bounding box
    let min_x = cells_i32.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let min_y = cells_i32.iter().map(|&(_, y)| y).min().unwrap_or(0);

    // Calculate center of rotation (use top-left as anchor)
    let center_x = min_x;
    let center_y = min_y;

    // Invert angle for clockwise rotation
    let angle = -f64::from(rotation).to_radians();
    let sin = angle.sin();
    let cos = angle.cos();

    cells_i32.iter()
        .map(|&(x, y)| {
            // Translate to origin
            let dx = x - center_x;
            let dy = y - center_y;
            
            // Rotate clockwise
            let rx = (f64::from(dx) * cos - f64::from(dy) * sin).round() as i32;
            let ry = (f64::from(dx) * sin + f64::from(dy) * cos).round() as i32;
            
            // Translate back
            let new_x = (rx + center_x) as usize;
            let new_y = (ry + center_y) as usize;
            
            (new_x, new_y)
        })
        .collect()
}

// Helper function specifically for rotating tile coordinates
pub fn rotate_coordinates_for_tile(tile: &Tile) -> Option<Vec<(usize, usize)>> {
    if tile.cells.is_empty() {
        return None;
    }
    
    // Simulate the next rotation (90° clockwise)
    Some(rotate_coordinates(&tile.cells, 90))
}
