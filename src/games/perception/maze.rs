use js_sys::Math;
use std::collections::HashSet;
use web_sys::Document;

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        let total_walls = size * size * 4;
        let mut walls = vec![true; total_walls];
        let mut visited = HashSet::new();
        
        // Generate random positions for start, key, and door
        let start_x = (Math::random() * size as f64) as usize;
        let start_y = (Math::random() * size as f64) as usize;
        let start_position = (start_x, start_y);
        
        // Place key in opposite quadrant from start
        let key_x = if start_x < size / 2 { 
            size/2 + (Math::random() * (size/2) as f64) as usize 
        } else { 
            (Math::random() * (size/2) as f64) as usize 
        };
        let key_y = if start_y < size / 2 { 
            size/2 + (Math::random() * (size/2) as f64) as usize 
        } else { 
            (Math::random() * (size/2) as f64) as usize 
        };
        let key_position = (key_x, key_y);

        // Place door far from both start and key
        let door_x = (start_x + size/2) % size;
        let door_y = (key_y + size/2) % size;
        let door_position = (door_x, door_y);

        // Wilson's algorithm for maze generation
        let mut unvisited: HashSet<(usize, usize)> = (0..size)
            .flat_map(|y| (0..size).map(move |x| (x, y)))
            .collect();
        
        // Start with initial cell
        visited.insert(start_position);
        unvisited.remove(&start_position);

        // Generate paths until all cells are connected
        while !unvisited.is_empty() {
            let mut current = *unvisited.iter().next().unwrap();
            let mut path = vec![current];
            
            // Random walk until we hit a visited cell
            while !visited.contains(&current) {
                let directions = [
                    (0, -1), (1, 0), (0, 1), (-1, 0)
                ];
                
                let valid_directions: Vec<(i32, i32)> = directions
                    .iter()
                    .filter(|(dx, dy)| {
                        let new_x = current.0 as i32 + dx;
                        let new_y = current.1 as i32 + dy;
                        new_x >= 0 && new_x < size as i32 && 
                        new_y >= 0 && new_y < size as i32
                    })
                    .cloned()
                    .collect();
                
                let (dx, dy) = valid_directions[
                    (Math::random() * valid_directions.len() as f64) as usize
                ];
                
                current = (
                    (current.0 as i32 + dx) as usize,
                    (current.1 as i32 + dy) as usize
                );
                
                path.push(current);
            }

            // Carve the path
            for window in path.windows(2) {
                let from = window[0];
                let to = window[1];
                let wall_idx = Self::get_wall_index(&Self::default(), from.0, from.1, to.0, to.1);
                walls[wall_idx] = false;
                // Remove opposite wall too
                let wall_idx = Self::get_wall_index(&Self::default(), to.0, to.1, from.0, from.1);
                walls[wall_idx] = false;
                visited.insert(from);
                unvisited.remove(&from);
            }
        }

        // Initialize with start position visited
        visited.clear();
        visited.insert(start_position);

        Self {
            size,
            walls,
            visited,
            current_position: start_position,
            start_position,
            key_position,
            door_position,
            has_key: false,
            document,
            level: 1,
            mazes_completed: 0,
            time_remaining: 300,
            last_tick: js_sys::Date::now() / 1000.0,
        }
    }
}

impl Default for Perception {
    fn default() -> Self {
        Self {
            size: 0,
            walls: Vec::new(),
            visited: HashSet::new(),
            current_position: (0, 0),
            start_position: (0, 0),
            key_position: (0, 0),
            door_position: (0, 0),
            has_key: false,
            document: web_sys::window()
                .unwrap()
                .document()
                .unwrap(),
            level: 1,
            mazes_completed: 0,
            time_remaining: 300,
            last_tick: 0.0,
        }
    }
}