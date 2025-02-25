use crate::Perception;
use js_sys::Date;
use js_sys::Math;
use std::collections::{HashSet, VecDeque};
use web_sys::Document;

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        // Total cells and walls per cell (top, right, bottom, left)
        let total_cells = size * size;
        let wall_per_cell = 4;
        let mut walls = vec![true; total_cells * wall_per_cell];

        // DFS setup for spanning tree generation
        let mut visited_cells = vec![false; total_cells];
        let idx = |r: usize, c: usize| r * size + c; // utility: (row, col) -> index

        // Pick a random starting cell (row, col)
        let start_row = (Math::random() * size as f64).floor() as usize;
        let start_col = (Math::random() * size as f64).floor() as usize;

        let mut stack = vec![(start_row, start_col)];
        visited_cells[idx(start_row, start_col)] = true;

        // Directions: (dr, dc, current wall index, neighbor wall index)
        // Up: (r-1, c) uses wall 0 in current and 2 in neighbor.
        // Right: (r, c+1) uses wall 1 in current and 3 in neighbor.
        // Down: (r+1, c) uses wall 2 in current and 0 in neighbor.
        // Left: (r, c-1) uses wall 3 in current and 1 in neighbor.
        let directions = [
            (-1, 0, 0, 2),
            (0, 1, 1, 3),
            (1, 0, 2, 0),
            (0, -1, 3, 1),
        ];

        // Iterative DFS: remove walls to create a spanning tree
        while let Some((r, c)) = stack.last().copied() {
            let mut neighbors = Vec::new();
            for &(dr, dc, cur_wall, nb_wall) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < size as isize && nc >= 0 && nc < size as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !visited_cells[idx(nr, nc)] {
                        neighbors.push((nr, nc, cur_wall, nb_wall));
                    }
                }
            }

            if neighbors.is_empty() {
                stack.pop();
            } else {
                // Shuffle neighbors using js_sys::Math::random
                neighbors.sort_by(|_, _| {
                    if Math::random() < 0.5 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                let (nr, nc, cur_wall, nb_wall) = neighbors[0];
                // Remove walls between current and neighbor
                let cell_base = idx(r, c) * wall_per_cell;
                walls[cell_base + cur_wall] = false;
                let nb_base = idx(nr, nc) * wall_per_cell;
                walls[nb_base + nb_wall] = false;
                visited_cells[idx(nr, nc)] = true;
                stack.push((nr, nc));
            }
        }

        // --- BFS helper: find furthest cell (and parent pointers) ---
        let bfs = |start_cell: (usize, usize)| {
            let mut dist = vec![None; total_cells];
            let mut parent = vec![None; total_cells];
            let start_idx = idx(start_cell.0, start_cell.1);
            let mut queue = VecDeque::new();
            dist[start_idx] = Some(0);
            queue.push_back(start_idx);

            while let Some(current) = queue.pop_front() {
                let r = current / size;
                let c = current % size;
                for &(dr, dc, cur_wall, _) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < size as isize && nc >= 0 && nc < size as isize {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        let neighbor_idx = idx(nr, nc);
                        // Only move if there is no wall between current and neighbor.
                        let cell_base = current * wall_per_cell;
                        if !walls[cell_base + cur_wall] && dist[neighbor_idx].is_none() {
                            dist[neighbor_idx] = Some(dist[current].unwrap() + 1);
                            parent[neighbor_idx] = Some(current);
                            queue.push_back(neighbor_idx);
                        }
                    }
                }
            }

            // Find the furthest cell from the start
            let (furthest_idx, _) = dist
                .iter()
                .enumerate()
                .filter_map(|(i, d)| d.map(|d| (i, d)))
                .max_by_key(|&(_, d)| d)
                .unwrap();
            ((furthest_idx / size, furthest_idx % size), parent)
        };

        // --- Determine maze endpoints using the diameter ---
        let (cell_a, _) = bfs((start_row, start_col));
        let (cell_b, parent_map) = bfs(cell_a);

        // Reconstruct the unique path (from cell_a to cell_b)
        let mut path = Vec::new();
        let mut current = idx(cell_b.0, cell_b.1);
        path.push(current);
        while let Some(p) = parent_map[current] {
            path.push(p);
            current = p;
        }
        path.reverse();

        // In the DFS/BFS we used (row, col) order.
        // Convert to (x, y) where x = col and y = row to match movement.rs.
        let convert = |(r, c): (usize, usize)| (c, r);
        let start_rc = cell_a;
        let door_rc = cell_b;
        let key_rc = if path.len() >= 3 {
            // Pick a random intermediate index (excluding endpoints)
            let key_idx = 1 + ((Math::random() * ((path.len() - 2) as f64)).floor() as usize);
            let cell = path[key_idx];
            (cell / size, cell % size)
        } else {
            start_rc
        };

        let start_cell = convert(start_rc);
        let door_cell = convert(door_rc);
        let key_cell  = convert(key_rc);

        // --- Initialize remaining fields ---
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
