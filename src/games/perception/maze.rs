use crate::Perception;
use js_sys::Date;
use js_sys::Math;
use std::collections::{HashSet, VecDeque};
use web_sys::Document;

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        // Number of cells and walls per cell (top, right, bottom, left)
        let total_cells = size * size;
        let wall_per_cell = 4;
        // Initialize all walls to true (wall exists)
        let mut walls = vec![true; total_cells * wall_per_cell];

        // Create a visited grid for DFS maze generation
        let mut visited_cells = vec![false; total_cells];

        // Utility: convert (r, c) into index
        let idx = |r: usize, c: usize| r * size + c;

        // Random start cell for DFS
        let start_row = (Math::random() * size as f64).floor() as usize;
        let start_col = (Math::random() * size as f64).floor() as usize;

        // Stack for DFS; push starting cell.
        let mut stack = vec![(start_row, start_col)];
        visited_cells[idx(start_row, start_col)] = true;

        // Directions: (dr, dc, current wall index, neighbor wall index)
        let directions = [
            (-1, 0, 0, 2), // Up: remove top for current, bottom for neighbor
            (0, 1, 1, 3),  // Right: remove right for current, left for neighbor
            (1, 0, 2, 0),  // Down: remove bottom for current, top for neighbor
            (0, -1, 3, 1), // Left: remove left for current, right for neighbor
        ];

        // Iterative DFS to generate spanning tree maze
        while let Some((r, c)) = stack.last().copied() {
            // Collect unvisited valid neighbors in a vector
            let mut neighbors = Vec::new();
            for &(dr, dc, cur_wall, nb_wall) in &directions {
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;
                if new_r >= 0 && new_r < size as isize && new_c >= 0 && new_c < size as isize {
                    let new_r = new_r as usize;
                    let new_c = new_c as usize;
                    if !visited_cells[idx(new_r, new_c)] {
                        neighbors.push((new_r, new_c, cur_wall, nb_wall));
                    }
                }
            }

            if !neighbors.is_empty() {
                // Shuffle neighbors using js_sys::Math::random
                neighbors.sort_by(|_, _| {
                    if Math::random() < 0.5 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                let (nr, nc, cur_wall, nb_wall) = neighbors[0];
                // Remove wall between current and neighbor
                let cell_base = idx(r, c) * wall_per_cell;
                walls[cell_base + cur_wall] = false;
                let nb_base = idx(nr, nc) * wall_per_cell;
                walls[nb_base + nb_wall] = false;
                // Mark neighbor as visited and push it to the stack
                visited_cells[idx(nr, nc)] = true;
                stack.push((nr, nc));
            } else {
                stack.pop();
            }
        }

        // --- BFS helper to find furthest cell and parents ---
        // Returns: (furthest_cell, parent mapping vector)
        let bfs = |start_cell: (usize, usize)| {
            let mut dist = vec![None; total_cells];
            let mut parent = vec![None; total_cells];
            let start_index = idx(start_cell.0, start_cell.1);
            let mut queue = VecDeque::new();
            dist[start_index] = Some(0);
            queue.push_back(start_index);

            while let Some(current) = queue.pop_front() {
                let r = current / size;
                let c = current % size;
                // Check all four directions
                for &(dr, dc, cur_wall, _) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < size as isize && nc >= 0 && nc < size as isize {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        let neighbor_index = idx(nr, nc);
                        // If the wall between current and neighbor is open then move there.
                        let cell_base = current * wall_per_cell;
                        if !walls[cell_base + cur_wall] && dist[neighbor_index].is_none() {
                            dist[neighbor_index] = Some(dist[current].unwrap() + 1);
                            parent[neighbor_index] = Some(current);
                            queue.push_back(neighbor_index);
                        }
                    }
                }
            }
            // Find furthest cell from start_cell
            let (furthest_index, _) = dist
                .iter()
                .enumerate()
                .filter_map(|(i, d)| d.map(|distance| (i, distance)))
                .max_by_key(|&(_, distance)| distance)
                .unwrap();
            ((furthest_index / size, furthest_index % size), parent)
        };

        // --- Find longest path (maze diameter) ---
        // First BFS from the random start chosen in DFS:
        let (cell_a, _) = bfs((start_row, start_col));
        // Second BFS starting from cell_a to find the furthest cell_b and record parents
        let (cell_b, parent_map) = bfs(cell_a);
        // Reconstruct the path from cell_b back to cell_a
        let mut path = Vec::new();
        let mut current = idx(cell_b.0, cell_b.1);
        path.push(current);
        while let Some(p) = parent_map[current] {
            path.push(p);
            current = p;
        }
        path.reverse(); // Now path is from cell_a to cell_b

        // --- Set start, door, key ---
        // start is one end and door is at the other.
        let start_cell = cell_a;
        let door_cell = cell_b;
        // Choose key from an intermediate cell in the path.
        let key_cell = if path.len() >= 3 {
            // valid random index in (0,len-1) excluding endpoints: indices 1..path.len()-1
            let key_idx = 1 + ((Math::random() * ((path.len() - 2) as f64)).floor() as usize);
            let key = path[key_idx];
            (key / size, key % size)
        } else {
            // if path too short, use start_cell (this rarely happens except in tiny labyrinths)
            start_cell
        };

        // --- Initialize remaining fields ---
        // Current position starts at start_cell and visited is initialized with it.
        let mut visited = HashSet::new();
        visited.insert(start_cell);

        Self {
            document,
            size,
            walls,
            current_position: start_cell,
            start_position: start_cell,
            key_position: key_cell,
            door_position: door_cell,
            visited,
            has_key: false,
            level: 1,
            moves: 0,
            time_remaining: 300,
            last_tick: Date::now() / 1000.0,
        }
    }
}
