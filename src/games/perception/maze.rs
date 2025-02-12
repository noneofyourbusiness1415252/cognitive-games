use js_sys::Math;
use std::collections::HashSet;
use web_sys::Document;

use super::Perception;

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                std::cmp::Ordering::Less => self.parent[root_x] = root_y,
                std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
                std::cmp::Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
        }
    }
}

impl Perception {
    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        let total_cells = size * size;
        let mut walls = vec![true; total_cells * 4];
        let mut ds = DisjointSet::new(total_cells);
        let mut edges = Vec::new();

        // Generate all possible edges
        for y in 0..size {
            for x in 0..size {
                let cell = y * size + x;
                if x < size - 1 {
                    edges.push((cell, cell + 1, cell * 4 + 1)); // right wall
                }
                if y < size - 1 {
                    edges.push((cell, cell + size, cell * 4 + 2)); // bottom wall
                }
            }
        }

        // Shuffle edges
        for i in (1..edges.len()).rev() {
            let j = (Math::random() * (i + 1) as f64) as usize;
            edges.swap(i, j);
        }

        // Create maze using Kruskal's algorithm
        for (cell1, cell2, wall_idx) in edges {
            if ds.find(cell1) != ds.find(cell2) {
                walls[wall_idx] = false;
                // Mirror wall removal (if removing right wall of cell1, remove left wall of cell2)
                if wall_idx % 4 == 1 { // right wall
                    walls[wall_idx + 2] = false; // left wall of adjacent cell
                } else if wall_idx % 4 == 2 { // bottom wall
                    walls[wall_idx + size * 4 - 2] = false; // top wall of cell below
                }
                ds.union(cell1, cell2);
            }
        }

        // Generate start position in the first third
        let start_x = (Math::random() * (size as f64 / 3.0)) as usize;
        let start_y = (Math::random() * (size as f64 / 3.0)) as usize;
        let start_position = (start_x, start_y);

        // Generate key position in the middle third
        let key_x = (Math::random() * (size as f64 / 3.0)) as usize + size / 3;
        let key_y = (Math::random() * (size as f64 / 3.0)) as usize + size / 3;
        let key_position = (key_x, key_y);

        // Generate door position in the last third
        let door_x = (Math::random() * (size as f64 / 3.0)) as usize + (2 * size) / 3;
        let door_y = (Math::random() * (size as f64 / 3.0)) as usize + (2 * size) / 3;
        let door_position = (door_x, door_y);

        // Make door position initially inaccessible by adding walls around it
        let door_cell = door_y * size + door_x;
        for i in 0..4 {
            walls[door_cell * 4 + i] = true;
        }

        Self {
            size,
            walls,
            current_position: start_position,
            start_position,
            key_position,
            door_position,
            has_key: false,
            visited: HashSet::from([start_position]),
            level: 1,
            time_remaining: 300,
            last_tick: js_sys::Date::now() / 1000.0,
            document,
        }
    }
}