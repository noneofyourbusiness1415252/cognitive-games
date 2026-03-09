use std::collections::HashSet;
use super::tile::{Tile, Direction, direction_between, diagonal_for_turn};
use js_sys::Math;

fn rand(max: usize) -> usize {
    if max == 0 { 0 } else { (Math::random() * max as f64) as usize }
}

pub fn generate_level(level: usize) -> (Vec<Tile>, Vec<(usize, usize)>, (usize, usize), (usize, usize)) {
    let n = level.max(1);
    let start_y = if n > 1 { rand(n) } else { 0 };
    let end_y   = if n > 1 { rand(n) } else { 0 };
    let start = (0, start_y);
    let end   = (n - 1, end_y);

    let path = generate_path(start, end, n);
    let mut tiles = build_tiles(&path, n);
    add_obstacles(&mut tiles, &path.iter().cloned().collect(), n);
    scramble(&mut tiles, n);

    (tiles, path, start, end)
}

/// Generate a winding O(n)-length path from start to end.
fn generate_path(start: (usize, usize), end: (usize, usize), n: usize) -> Vec<(usize, usize)> {
    if n == 1 { return vec![start]; }
    let mut path = vec![start];
    let mut pos = start;
    let mut visited: HashSet<(usize, usize)> = [start].into();

    while pos.0 < end.0 {
        let remaining_x = end.0 - pos.0;
        let dy: i32 = end.1 as i32 - pos.1 as i32;
        // Sometimes step vertically (if more than 1 column to go and path not yet aligned)
        let step_vert = dy != 0 && remaining_x > 1 && Math::random() < 0.45;
        let next = if step_vert {
            let ny = (pos.1 as i32 + dy.signum()) as usize;
            let candidate = (pos.0, ny);
            if ny < n && !visited.contains(&candidate) { candidate }
            else { (pos.0 + 1, pos.1) }
        } else {
            (pos.0 + 1, pos.1)
        };
        let next = if next.0 == end.0 { (end.0, next.1) } else { next };
        visited.insert(next);
        path.push(next);
        pos = next;
    }
    // Adjust final Y to reach end
    while pos.1 != end.1 {
        let dy: i32 = if pos.1 < end.1 { 1 } else { -1 };
        let np = (pos.0, (pos.1 as i32 + dy) as usize);
        if visited.contains(&np) { break; }
        visited.insert(np);
        path.push(np);
        pos = np;
    }
    if path.last().copied() != Some(end) && !visited.contains(&end) {
        path.push(end);
    }
    path
}

/// Create tiles from the path (1-3 cells each) with per-cell arrows.
fn build_tiles(path: &[(usize, usize)], _n: usize) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut i = 0;
    while i < path.len() {
        let remaining = path.len() - i;
        let size = if remaining >= 3 { match rand(3) { 0 => 1, 1 => 2, _ => 3 } }
                   else if remaining == 2 { if rand(2) == 0 { 1 } else { 2 } }
                   else { 1 };
        let seg = &path[i..i + size];
        let arrows = assign_arrows(seg, path, i + size);
        tiles.push(Tile { cells: seg.to_vec(), arrows, is_obstacle: false });
        i += size;
    }
    tiles
}

/// Assign per-cell arrows for a tile covering path[start..start+seg.len()].
/// `next_start` is the index of the first cell of the next tile in path.
fn assign_arrows(seg: &[(usize, usize)], path: &[(usize, usize)], next_start: usize) -> Vec<Direction> {
    let len = seg.len();
    (0..len).map(|j| {
        if len == 1 {
            // Single cell: point toward next tile's first cell, or East if last
            if next_start < path.len() { direction_between(seg[0], path[next_start]) }
            else { Direction::East }
        } else if j == len - 1 {
            // Endpoint: point toward next tile or East if last in path
            if next_start < path.len() { direction_between(seg[j], path[next_start]) }
            else { Direction::East }
        } else if j == 0 {
            // Endpoint: point toward next cell in segment
            direction_between(seg[0], seg[1])
        } else {
            // Interior junction: diagonal if turning, otherwise straight
            let in_d  = direction_between(seg[j - 1], seg[j]);
            let out_d = direction_between(seg[j], seg[j + 1]);
            if in_d == out_d { out_d } else { diagonal_for_turn(in_d, out_d) }
        }
    }).collect()
}

/// Add obstacle cells (same visual as tiles, no arrows, blocks rotation).
fn add_obstacles(tiles: &mut Vec<Tile>, path_cells: &HashSet<(usize, usize)>, n: usize) {
    if n < 3 { return; }
    let mut occupied: HashSet<(usize, usize)> = path_cells.clone();
    for t in tiles.iter() { for &c in &t.cells { occupied.insert(c); } }
    for _ in 0..n / 3 {
        for _ in 0..20 {
            let x = rand(n); let y = rand(n);
            if !occupied.contains(&(x, y)) {
                occupied.insert((x, y));
                tiles.push(Tile { cells: vec![(x, y)], arrows: vec![], is_obstacle: true });
                break;
            }
        }
    }
}

/// Scramble tiles: apply 1-3 valid CW rotations and a possible reversal.
/// Uses O(n) time with HashSet for collision checking.
fn scramble(tiles: &mut Vec<Tile>, n: usize) {
    // Build occupied set
    let mut occupied: HashSet<(usize, usize)> = tiles.iter()
        .flat_map(|t| t.cells.iter().cloned()).collect();

    for idx in 0..tiles.len() {
        if tiles[idx].is_obstacle { continue; }
        let rots = rand(3) + 1; // 1, 2 or 3 rotations → not solution state
        for _ in 0..rots {
            let rotated = tiles[idx].rotated_cells();
            let ok = rotated.iter().all(|&(x, y)| x < n && y < n)
                && rotated.iter().all(|c| {
                    !tiles[..idx].iter().chain(tiles[idx+1..].iter())
                        .any(|t| t.cells.contains(c))
                });
            if ok {
                for &c in &tiles[idx].cells { occupied.remove(&c); }
                tiles[idx].rotate_cw();
                for &c in &tiles[idx].cells { occupied.insert(c); }
            }
        }
        if Math::random() < 0.5 { tiles[idx].reverse_arrows(); }
    }
}
