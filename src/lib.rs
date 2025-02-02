use js_sys::Math;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, HtmlElement};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let game = MazeGame::new()?;
    game.render()?;
    Ok(())
}
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct MazeGame {
    // Game state
    size: usize,
    level: usize,
    #[serde(default)]
    mazes_completed: usize,

    // Maze elements
    walls: Vec<bool>,
    current_position: (usize, usize),
    key_position: (usize, usize),
    door_position: (usize, usize),
    visited: HashSet<(usize, usize)>,
    has_key: bool,

    // Timer state
    time_remaining: i32,
    last_tick: f64,

    #[serde(skip, default = "get_document")]
    document: Document,
}

fn get_document() -> Document {
    web_sys::window()
        .expect("no global window exists")
        .document()
        .expect("no document exists")
}
#[wasm_bindgen]
impl MazeGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<MazeGame, JsValue> {
        let document = get_document();
        let storage = web_sys::window()
            .expect("no global window exists")
            .local_storage()?
            .expect("no local storage");

        let mut game = if let Some(state) = storage.get_item("maze_state")? {
            let last_save = storage
                .get_item("maze_time")?
                .unwrap_or_else(|| "0".to_string())
                .parse::<f64>()
                .unwrap_or(0.0);

            if js_sys::Date::now() - last_save > 300000.0 {
                MazeGame::create_maze(2, document)
            } else {
                serde_wasm_bindgen::from_value(js_sys::JSON::parse(&state)?)?
            }
        } else {
            MazeGame::create_maze(2, document)
        };

        game.render()?;
        game.start()?;
        Ok(game)
    }
    fn save_state(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global window exists");
        let storage = window.local_storage()?.expect("no local storage exists");

        // Save game state
        let state = serde_wasm_bindgen::to_value(&self)?;
        let state_json = js_sys::JSON::stringify(&state)?.as_string().unwrap();
        storage.set_item("maze_state", &state_json)?;

        // Save current time
        storage.set_item("maze_time", &js_sys::Date::now().to_string())?;

        // Save level separately
        storage.set_item("maze_level", &self.level.to_string())?;

        Ok(())
    }
    fn create_maze(size: usize, document: Document) -> MazeGame {
        let mut walls = vec![false; size * size * 4]; // Start with no walls
                                                      // Add random walls
        for i in 0..walls.len() {
            walls[i] = Math::random() < 0.5;
        }

        let (waypoint1, key_position, waypoint2, door_position) = if size == 2 {
            let key_pos = loop {
                let pos = (
                    (Math::random() * 2.0).floor() as usize,
                    (Math::random() * 2.0).floor() as usize,
                );
                if pos != (0, 0) {
                    break pos;
                };
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

        let mut game = MazeGame {
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

        // Clear path function - ensures a 2-cell wide path
        fn clear_path(
            walls: &mut Vec<bool>,
            from: (usize, usize),
            to: (usize, usize),
            size: usize,
        ) {
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
                        let adj_wall_idx =
                            (current.1 * size + next_x) * 4 + if dx > 0 { 3 } else { 1 };
                        walls[adj_wall_idx] = false;

                        // Always clear an escape route (up or down)
                        let escape_dir = if current.1 > 0 { 0 } else { 2 }; // up if not at top, down otherwise
                        walls[(current.1 * size + current.0) * 4 + escape_dir] = false;
                        if escape_dir == 0 && current.1 > 0 {
                            // Clear the corresponding wall in the cell above
                            walls[((current.1 - 1) * size + current.0) * 4 + 2] = false;
                        } else if escape_dir == 2 && current.1 + 1 < size {
                            // Clear the corresponding wall in the cell below
                            walls[((current.1 + 1) * size + current.0) * 4 + 0] = false;
                        }
                    }
                    current.0 = (current.0 as i32 + dx) as usize;
                } else if dy != 0 {
                    let wall_idx = (current.1 * size + current.0) * 4 + if dy > 0 { 2 } else { 0 };
                    walls[wall_idx] = false;
                    // Clear adjacent cell's opposite wall if not at edge
                    if (dy > 0 && current.1 + 1 < size) || (dy < 0 && current.1 > 0) {
                        let next_y = (current.1 as i32 + dy) as usize;
                        let adj_wall_idx =
                            (next_y * size + current.0) * 4 + if dy > 0 { 0 } else { 2 };
                        walls[adj_wall_idx] = false;
                    }
                    current.1 = (current.1 as i32 + dy) as usize;
                }
                // Ensure escape route from the destination
                let escape_dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // up, down, left, right
                for (dx, dy) in escape_dirs.iter() {
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
        // Create path through waypoints
        clear_path(&mut game.walls, (0, 0), waypoint1, size);
        clear_path(&mut game.walls, waypoint1, key_position, size);
        clear_path(&mut game.walls, key_position, waypoint2, size);
        clear_path(&mut game.walls, waypoint2, door_position, size);
        game.visited.insert((0, 0));
        game
    }
    pub fn render(&self) -> Result<(), JsValue> {
        let maze = self.document.get_element_by_id("maze").unwrap();

        // Only regenerate grid if size changed
        if maze.children().length() as usize != self.size * self.size {
            maze.set_attribute(
                "style",
                &format!("grid-template-columns: repeat({}, 60px)", self.size),
            )?;
            maze.set_inner_html("");

            // Create cells only once
            for _ in 0..(self.size * self.size) {
                let cell = self.document.create_element("div")?;
                cell.set_class_name("cell");
                let span = self.document.create_element("span")?;
                cell.append_child(&span)?;
                maze.append_child(&cell)?;
            }
        }

        // Update existing cells
        for y in 0..self.size {
            for x in 0..self.size {
                let index = (y * self.size + x) as u32; // Convert to u32 for item() call
                if let Some(cell) = maze.children().item(index) {
                    self.update_cell_state(&cell, x, y)?;
                }
            }
        }

        // Update stats (unchanged)
        if let Some(level_el) = self.document.get_element_by_id("level") {
            level_el.set_inner_html(&self.level.to_string());
        }
        if let Some(completed_el) = self.document.get_element_by_id("completed") {
            completed_el.set_inner_html(&self.mazes_completed.to_string());
        }
        if let Some(timer_el) = self.document.get_element_by_id("timer") {
            let minutes = self.time_remaining / 60;
            let seconds = self.time_remaining % 60;
            timer_el.set_inner_html(&format!("{}:{:02}", minutes, seconds));
        }
        Ok(())
    }
fn update_cell_state(&self, cell: &Element, x: usize, y: usize) -> Result<(), JsValue> {
    // Reset base class
    cell.set_class_name("cell");
    
    // Update state classes
    if self.visited.contains(&(x, y)) {
        cell.class_list().add_1("visited")?;
    }
    if (x, y) == self.current_position {
        cell.class_list().add_1("current")?;
        // Ensure span exists for pseudo-elements
        if cell.children().length() == 0 {
            let span = self.document.create_element("span")?;
            cell.append_child(&span)?;
        }
    } else {
        // Remove span if not current position
        while cell.children().length() > 0 {
            if let Some(child) = cell.first_child() {
                cell.remove_child(&child)?;
            }
        }
    }

    // Update content only if needed
    let content = if (x, y) == self.key_position && !self.has_key {
        "🔑"
    } else if (x, y) == self.current_position && self.has_key {
        "🔑"
    } else if (x, y) == self.door_position {
        "🚪"
    } else {
        ""
    };

    if cell.inner_html() != content {
        cell.set_inner_html(content);
        // Re-add span if this is current position (since inner_html clears children)
        if (x, y) == self.current_position {
            let span = self.document.create_element("span")?;
            cell.append_child(&span)?;
        }
    }

    Ok(())
}    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        let game_state = Rc::new(RefCell::new(self.clone()));

        let click_handler = {
            let game_state = game_state.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if let Ok(mut game) = game_state.try_borrow_mut() {
                    if let Some(target) = event.target() {
                        if let Some(element) = target.dyn_ref::<Element>() {
                            if let Ok(Some(maze_el)) = element.closest("#maze") {
                                // Find clicked cell index
                                let children = maze_el.children();
                                let cell_index = (0..children.length())
                                    .find(|&i| {
                                        children
                                            .item(i)
                                            .map(|cell| cell.is_same_node(Some(element)))
                                            .unwrap_or(false)
                                    })
                                    .unwrap_or(0)
                                    as usize;

                                let size = game.size;
                                let x = cell_index % size;
                                let y = cell_index / size;

                                let result = game.try_move(x, y);
                                if result != 0 {
                                    game.render().unwrap();
                                }
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };
        // Attach single click handler to maze container
        if let Some(maze_el) = self.document.get_element_by_id("maze") {
            maze_el.add_event_listener_with_callback(
                "click",
                click_handler.as_ref().unchecked_ref(),
            )?;
            click_handler.forget();
        }
        let f = {
            let game_state = game_state.clone();
            Closure::wrap(Box::new(move || {
                if let Ok(mut game) = game_state.try_borrow_mut() {
                    let now = js_sys::Date::now() / 1000.0;
                    let delta = (now - game.last_tick) as i32;
                    if delta >= 1 {
                        game.time_remaining -= 1;
                        game.last_tick = now;

                        if game.time_remaining <= 0 {
                            let new_game = MazeGame::create_maze(game.size, game.document.clone());
                            game.walls = new_game.walls;
                            game.key_position = new_game.key_position;
                            game.door_position = new_game.door_position;
                            game.reset_position();
                            game.time_remaining = 300;
                            game.last_tick = now;
                            game.render().unwrap();
                        }

                        game.save_state().unwrap_or_else(|_| {
                            console::log_1(&"Failed to save game state".into());
                        });
                        if let Some(timer_el) = game.document.get_element_by_id("timer") {
                            let minutes = game.time_remaining / 60;
                            let seconds = game.time_remaining % 60;
                            timer_el.set_inner_html(&format!("{}:{:02}", minutes, seconds));
                        }

                        game.render().unwrap_or_else(|_| {
                            console::log_1(&"Failed to render timer update".into());
                        });

                        game.save_state().unwrap_or_else(|_| {
                            console::log_1(&"Failed to save game state".into());
                        });
                    }
                }
            }) as Box<dyn FnMut()>)
        };

        // Set up interval timer
        let window = web_sys::window().unwrap();
        console::log_1(&"Setting up interval...".into());
        let result = window.set_interval_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            1000,
        );

        match result {
            Ok(_) => console::log_1(&"Interval set up successfully".into()),
            Err(e) => console::log_2(&"Failed to set up interval:".into(), &e),
        }

        f.forget();
        console::log_1(&"Setup complete".into());
        Ok(())
    }
    fn create_cell(&self, x: usize, y: usize) -> Result<Element, JsValue> {
        let cell = self.document.create_element("div")?;
        cell.set_class_name("cell");

        // Add visited and current classes
        if self.visited.contains(&(x, y)) {
            cell.class_list().add_1("visited")?;
        }
        if (x, y) == self.current_position {
            if (x, y) == self.current_position {
                cell.class_list().add_1("current")?;
                // Add an empty span for the left/right pseudo-elements
                let span = self.document.create_element("span")?;
                cell.append_child(&span)?;
            }
        }

        // Add key and door symbols - only one instance of the key should exist
        if (x, y) == self.key_position && !self.has_key {
            cell.set_inner_html("🔑");
        } else if (x, y) == self.current_position && self.has_key {
            cell.set_inner_html("🔑");
        } else if (x, y) == self.door_position {
            cell.set_inner_html("🚪");
        }
        let x = x.clone();
        let y = y.clone();
        let click_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(maze_el) = doc.get_element_by_id("maze") {
                        let event_init = web_sys::CustomEventInit::new();
                        event_init.set_detail(&JsValue::from_str(&format!("{},{}", x, y)));
                        let event = web_sys::CustomEvent::new_with_event_init_dict(
                            "cell-click",
                            &event_init,
                        )
                        .unwrap();
                        maze_el.dispatch_event(&event).unwrap();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        let cell_element: &HtmlElement = cell.dyn_ref().unwrap();
        cell_element.set_onclick(Some(click_callback.as_ref().unchecked_ref()));
        click_callback.forget();
        Ok(cell)
    }
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
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

    fn get_wall_index(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> usize {
        let cell_walls = 4; // each cell has 4 possible walls
        let base_index = (from_y * self.size + from_x) * cell_walls;

        if to_x > from_x {
            base_index + 1 // right wall
        } else if to_x < from_x {
            base_index + 3 // left wall
        } else if to_y > from_y {
            base_index + 2 // bottom wall
        } else {
            base_index + 0 // top wall
        }
    }
    #[wasm_bindgen]
    pub fn try_move(&mut self, x: usize, y: usize) -> i32 {
        let result = self.try_move_internal(x, y);
        if result != 0 {
            self.save_state().unwrap_or_else(|_| {
                console::log_1(&"Failed to save game state".into());
            });
        }
        result
    }
    fn try_move_internal(&mut self, x: usize, y: usize) -> i32 {
        if !self.is_adjacent(x, y) {
            return 0;
        }

        let wall_idx = self.get_wall_index(self.current_position.0, self.current_position.1, x, y);

        // Block access to door position if key not collected
        if (x, y) == self.door_position && !self.has_key {
            return 0;
        }

        if self.walls[wall_idx] {
            self.reset_position();
            return -1;
        }

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
            let new_game = MazeGame::create_maze(self.size, self.document.clone());
            self.walls = new_game.walls;
            self.current_position = (0, 0);
            self.key_position = new_game.key_position;
            self.door_position = new_game.door_position;
            self.visited.clear();
            self.visited.insert((0, 0));
            self.has_key = false;
            self.time_remaining = 300;
            self.last_tick = js_sys::Date::now() / 1000.0;
            return 2;
        }
        1
    }
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        let new_game = MazeGame::create_maze(self.size, self.document.clone());
        self.walls = new_game.walls;
        self.key_position = new_game.key_position;
        self.door_position = new_game.door_position;
        self.reset_position();

        // Reset timer state completely
        self.time_remaining = 300;
        self.last_tick = js_sys::Date::now() / 1000.0;

        // Force timer display update
        if let Some(timer_el) = self.document.get_element_by_id("timer") {
            timer_el.set_inner_html("5:00");
        }

        // Update display
        self.render().expect("Failed to render reset");
    }
    fn reset_position(&mut self) {
        self.current_position = (0, 0);
        self.visited.clear();
        self.visited.insert((0, 0));
        self.has_key = false;
        self.render().expect("Failed to render position reset");
    }
}
