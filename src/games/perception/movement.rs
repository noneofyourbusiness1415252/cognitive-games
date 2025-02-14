use super::Perception;

impl Perception {
    pub(super) fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let current_x = self.current_position.0;
        let current_y = self.current_position.1;

        // Check if target position is adjacent (up, down, left, right)
        let dx = if x >= current_x {
            x - current_x
        } else {
            current_x - x
        };
        let dy = if y >= current_y {
            y - current_y
        } else {
            current_y - y
        };

        // Only one coordinate can change by 1, the other must be 0
        (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
    }

    pub(super) fn get_wall_index(
        &self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> usize {
        let cell_walls = 4; // each cell has 4 possible walls
        let base_index = (from_y * self.size + from_x) * cell_walls;

        if to_x > from_x {
            base_index + 1 // right wall
        } else if to_x < from_x {
            base_index + 3 // left wall
        } else if to_y > from_y {
            base_index + 2 // bottom wall
        } else {
            base_index // top wall
        }
    }

    fn animate_wall_hit(&self, target_x: usize, target_y: usize) -> Result<(), JsValue> {
        let maze = self.document.get_element_by_id("maze").unwrap();
        let index = self.current_position.1 * self.size + self.current_position.0;
        if let Some(cell) = maze.children().item(index as u32) {
            let dir = if target_x > self.current_position.0 {
                "right"
            } else if target_x < self.current_position.0 {
                "left"
            } else if target_y > self.current_position.1 {
                "bottom"
            } else {
                "top"
            };
            let class = format!("hit-{}", dir);
            cell.class_list().add_1(&class)?;
            let cell_clone = cell.clone();
            let class_clone = class.clone();
            let closure = Closure::wrap(Box::new(move || {
                let _ = cell_clone.class_list().remove_1(&class_clone);
            }) as Box<dyn FnMut()>);
            window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 300)?;
            closure.forget();
        }
        Ok(())
    }

    pub(super) fn try_move(&mut self, x: usize, y: usize) -> i32 {
        if !self.is_adjacent(x, y) {
            return 0;
        }

        let wall_idx = self.get_wall_index(self.current_position.0, self.current_position.1, x, y);

        // Block access to door position if key not collected
        if (x, y) == self.door_position && !self.has_key {
            return 0;
        }

        if self.walls[wall_idx] {
            // Animate the wall hit before resetting position.
            let _ = self.animate_wall_hit(x, y);
            self.reset_position();
            return -1;
        }

        // Record the move before updating the position
        self.moves += 1;  // <-- Increment move counter

        self.current_position = (x, y);
        self.visited.insert((x, y));

        if (x, y) == self.key_position {
            self.has_key = true;
            // When key is collected, make door accessible
            let door_x = self.door_position.0;
            let door_y = self.door_position.1;
            let base_idx = (door_y * self.size + door_x) * 4;
            for i in 0..4 {
                self.walls[base_idx + i] = false;
            }
        }

        if (x, y) == self.door_position && self.has_key {
            // Simplified level up - increase size immediately
            self.size += 1;
            self.level += 1;
            let new_game = Self::create_maze(self.size, self.document.clone());
            self.walls = new_game.walls;
            self.current_position = new_game.start_position; // Use start_position from new maze
            self.start_position = new_game.start_position; // Also update start_position
            self.key_position = new_game.key_position;
            self.door_position = new_game.door_position;
            self.visited.clear();
            self.visited.insert(new_game.start_position); // Insert correct start position
            self.has_key = false;
            self.moves = 0;
            self.time_remaining = 300;
            self.last_tick = js_sys::Date::now() / 1000.0;
            return 2;
        }
        1
    }
}
