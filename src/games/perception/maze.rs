use std::collections::HashSet;

use super::Perception;
use js_sys::Math;
use web_sys::Document;

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        let walls = (0..size * size * 4)
            .map(|_| Math::random() < 0.5)
            .collect::<Vec<bool>>();
        let (waypoint1, key_position, waypoint2, door_position) = if size == 2 {
            let key_pos = loop {
                let pos = (
                    (Math::random() * 2.0).floor() as usize,
                    (Math::random() * 2.0).floor() as usize,
                );
                if pos != (0, 0) {
                    break pos;
                }
            };
            // Get random non-start, non-key position for door
            let door_pos = loop {
                let pos = (
                    (Math::random() * 2.0).floor() as usize,
                    (Math::random() * 2.0).floor() as usize,
                );
                if pos != (0, 0) && pos != key_pos {
                    break pos;
                }
            };
            (key_pos, key_pos, door_pos, door_pos)
        } else {
            let (mut key_pos, mut door_pos);
            loop {
                key_pos = (
                    ((size as f64) * 0.6 + Math::random() * (size as f64) * 0.3).floor() as usize,
                    ((size as f64) * 0.6 + Math::random() * (size as f64) * 0.3).floor() as usize,
                );

                door_pos = (
                    ((size as f64) * 0.7 + Math::random() * (size as f64) * 0.2).floor() as usize,
                    ((size as f64) * 0.7 + Math::random() * (size as f64) * 0.2).floor() as usize,
                );

                if key_pos != door_pos && key_pos != (0, 0) && door_pos != (0, 0) {
                    break;
                }
            }
            (
                (
                    (Math::random() * (size as f64)).floor() as usize,
                    (Math::random() * (size as f64)).floor() as usize,
                ),
                key_pos,
                (
                    (Math::random() * (size as f64)).floor() as usize,
                    (Math::random() * (size as f64)).floor() as usize,
                ),
                door_pos,
            )
        };

        let mut game = Self {
            size,
            walls,
            current_position: (0, 0),
            key_position,
            door_position,
            visited: HashSet::new(),
            has_key: false,
            level: 1,
            mazes_completed: 0,
            document,
            time_remaining: 300,
            last_tick: js_sys::Date::now() / 1000.0,
        };
        // Create path through waypoints
        clear_path(&mut game.walls, (0, 0), waypoint1, size);
        clear_path(&mut game.walls, waypoint1, key_position, size);
        clear_path(&mut game.walls, key_position, waypoint2, size);
        clear_path(&mut game.walls, waypoint2, door_position, size);
        game.visited.insert((0, 0));
        game
    }
}
fn clear_path(walls: &mut [bool], from: (usize, usize), to: (usize, usize), size: usize) {
    let mut current = from;
    // Calculate minimum required path length (Manhattan distance * 1.5)
    while current != to {
        let dx = (to.0 as i32 - current.0 as i32).signum();
        let dy = (to.1 as i32 - current.1 as i32).signum();
        // Clear both current cell's wall and neighbor's wall
        if dx != 0 {
            let wall_idx = (current.1 * size + current.0) * 4 + if dx > 0 { 1 } else { 3 };
            walls[wall_idx] = false;
            // Clear adjacent cell's opposite wall if not at edge
            if (dx > 0 && current.0 + 1 < size) || (dx < 0 && current.0 > 0) {
                let next_x = (current.0 as i32 + dx) as usize;
                let adj_wall_idx = (current.1 * size + next_x) * 4 + if dx > 0 { 3 } else { 1 };
                walls[adj_wall_idx] = false;

                // Always clear an escape route (up or down)
                let escape_dir = if current.1 > 0 { 0 } else { 2 }; // up if not at top, down otherwise
                walls[(current.1 * size + current.0) * 4 + escape_dir] = false;
                if escape_dir == 0 && current.1 > 0 {
                    // Clear the corresponding wall in the cell above
                    walls[((current.1 - 1) * size + current.0) * 4 + 2] = false;
                } else if escape_dir == 2 && current.1 + 1 < size {
                    // Clear the corresponding wall in the cell below
                    walls[(current.1 + 1) * size + current.0 * 4] = false;
                }
            }
            current.0 = (current.0 as i32 + dx) as usize;
        } else if dy != 0 {
            let wall_idx = (current.1 * size + current.0) * 4 + if dy > 0 { 2 } else { 0 };
            walls[wall_idx] = false;
            // Clear adjacent cell's opposite wall if not at edge
            if (dy > 0 && current.1 + 1 < size) || (dy < 0 && current.1 > 0) {
                let next_y = (current.1 as i32 + dy) as usize;
                let adj_wall_idx = (next_y * size + current.0) * 4 + if dy > 0 { 0 } else { 2 };
                walls[adj_wall_idx] = false;
            }
            current.1 = (current.1 as i32 + dy) as usize;
        }
        // Ensure escape route from the destination
        let escape_dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // up, down, left, right
        for (dx, dy) in &escape_dirs {
            let next_x = to.0 as i32 + dx;
            let next_y = to.1 as i32 + dy;
            if next_x >= 0 && next_x < size as i32 && next_y >= 0 && next_y < size as i32 {
                let wall_idx = (to.1 * size + to.0) * 4
                    + if *dy < 0 {
                        0
                    } else if *dx > 0 {
                        1
                    } else if *dy > 0 {
                        2
                    } else {
                        3
                    };
                walls[wall_idx] = false;
            }
        }
    }
}
