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
    fn random_position(size: usize, sector: usize) -> (usize, usize) {
        let sector_size = size / 3;
        let offset = sector * sector_size;
        (
            (Math::random() * sector_size as f64) as usize + offset,
            (Math::random() * sector_size as f64) as usize + offset,
        )
    }

    pub(super) fn create_maze(size: usize, document: Document) -> Self {
        let total_cells = size * size;
        let mut walls = vec![true; total_cells * 4];
        let mut ds = DisjointSet::new(total_cells);
        let mut edges = Vec::with_capacity(2 * size * (size - 1));

        // Generate all possible edges
        for cell in 0..total_cells {
            let x = cell % size;
            let y = cell / size;
            if x < size - 1 {
                edges.push((cell, cell + 1, cell * 4 + 1));
            }
            if y < size - 1 {
                edges.push((cell, cell + size, cell * 4 + 2));
            }
        }

        // Fisher-Yates shuffle
        for i in (1..edges.len()).rev() {
            edges.swap(i, (Math::random() * (i + 1) as f64) as usize);
        }

        // Generate positions in different sectors
        let start_position = Self::random_position(size, 0);
        let key_position = Self::random_position(size, 1);
        let door_position = Self::random_position(size, 2);

        // Create maze using Kruskal's algorithm
        for (cell1, cell2, wall_idx) in edges {
            if ds.find(cell1) != ds.find(cell2) {
                walls[wall_idx] = false;
                // Mirror wall removal
                walls[wall_idx + if wall_idx % 4 == 1 { 2 } else { size * 4 - 2 }] = false;
                ds.union(cell1, cell2);
            }
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
            mazes_completed: 0,
        }
    }
}