use js_sys::Math;
use std::collections::HashSet;
use web_sys::Document;

use super::Perception;

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        let mut walls = vec![true; size * size * 4]; // Initialize all walls as true
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        // Helper function to get random int in range
        let random_int = |max: usize| -> usize { (Math::random() * max as f64).floor() as usize };

        // Generate random start position
        let start_x = random_int(size);
        let start_y = random_int(size);
        let start_pos = (start_x, start_y);

        visited.insert(start_pos);
        stack.push(start_pos);

        // Generate maze using iterative DFS
        while let Some(&current) = stack.last() {
            let (x, y) = current;
            let mut neighbors = Vec::new();

            // Check all possible neighbors
            if x > 0 && !visited.contains(&(x - 1, y)) {
                neighbors.push((x - 1, y, 3));
            }
            if x < size - 1 && !visited.contains(&(x + 1, y)) {
                neighbors.push((x + 1, y, 1));
            }
            if y > 0 && !visited.contains(&(x, y - 1)) {
                neighbors.push((x, y - 1, 0));
            }
            if y < size - 1 && !visited.contains(&(x, y + 1)) {
                neighbors.push((x, y + 1, 2));
            }

            if neighbors.is_empty() {
                stack.pop();
            } else {
                // Choose random unvisited neighbor
                let (next_x, next_y, wall_dir) = neighbors[random_int(neighbors.len())];

                // Remove wall between current and chosen cell
                let cell_walls = 4;
                let current_idx = (y * size + x) * cell_walls;
                walls[current_idx + wall_dir] = false;

                // Remove opposite wall of neighbor
                let opposite_wall = match wall_dir {
                    0 => 2, // top -> bottom
                    1 => 3, // right -> left
                    2 => 0, // bottom -> top
                    3 => 1, // left -> right
                    _ => unreachable!(),
                };
                let neighbor_idx = (next_y * size + next_x) * cell_walls;
                walls[neighbor_idx + opposite_wall] = false;

                visited.insert((next_x, next_y));
                stack.push((next_x, next_y));
            }
        }

        // Generate key and door positions (ensuring they're different from start and each other)
        let mut available_positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|y| (0..size).map(move |x| (x, y)))
            .filter(|&pos| pos != start_pos)
            .collect();

        let key_idx = random_int(available_positions.len());
        let key_position = available_positions.remove(key_idx);

        let door_idx = random_int(available_positions.len());
        let door_position = available_positions[door_idx];

        // Create initial visited set with just start position
        let mut initial_visited = HashSet::new();
        initial_visited.insert(start_pos);

        Perception {
            size,
            level: 1,
            mazes_completed: 0,
            walls,
            current_position: start_pos,
            start_position: start_pos,
            key_position,
            door_position,
            visited: initial_visited,
            has_key: false,
            time_remaining: 300,
            last_tick: js_sys::Date::now() / 1000.0,
            document,
        }
    }
}
