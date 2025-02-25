mod input;
mod maze;
mod movement;
mod render;
mod state;
mod timer;

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document};

fn get_document() -> Document {
    web_sys::window()
        .expect("no global window exists")
        .document()
        .expect("no document exists")
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Perception {
    // Game state
    size: usize,
    level: usize,
    #[serde(default)]
    moves: usize,  // <-- New field to record moves

    // Maze elements
    walls: Vec<bool>,
    current_position: (usize, usize),
    start_position: (usize, usize),
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

#[wasm_bindgen]
impl Perception {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        let document = get_document();
        let storage = web_sys::window()
            .expect("no global window exists")
            .local_storage()?
            .expect("no local storage");

        let now = js_sys::Date::now();
        let mut game = if let Some(state) = storage.get_item("maze_state")? {
            let last_save = storage
                .get_item("maze_time")?
                .unwrap_or_else(|| "0".to_string())
                .parse::<f64>()
                .unwrap_or(0.0);

            // If more than 5 minutes (300_000 ms) have passed since last save, create a new maze.
            if now - last_save > 300_000.0 {
                Self::create_maze(2, document)
            } else {
                // Deserialize the saved state.
                let mut game: Self =
                    serde_wasm_bindgen::from_value(js_sys::JSON::parse(&state)?)?;
                let now_secs = now / 1000.0;
                let elapsed = now_secs - game.last_tick;
                if elapsed as i32 >= game.time_remaining {
                    // Timer expired: start a new maze.
                    Self::create_maze(2, document)
                } else {
                    // Otherwise adjust the time remaining based on elapsed time.
                    game.time_remaining -= elapsed as i32;
                    game.last_tick = now_secs;
                    game
                }
            }
        } else {
            Self::create_maze(2, document)
        };

        game.render()?;
        game.start()?;
        Ok(game)
    }
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        let game_state = Rc::new(RefCell::new(self.clone()));

        Self::setup_click_handler(game_state.clone())?;
        Self::setup_timer(game_state.clone())?;

        // Set up reset button handler
        if let Some(reset_btn) = self.document.get_element_by_id("reset-level") {
            let game_state = game_state.clone();
            let handler = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                if let Ok(mut game) = game_state.try_borrow_mut() {
                    game.reset_to_level_one().unwrap();
                }
            }) as Box<dyn FnMut(_)>);

            reset_btn
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
            handler.forget();
        }

        console::log_1(&"Setup complete".into());
        Ok(())
    }
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        let new_game = Self::create_maze(self.size, self.document.clone());
        self.walls = new_game.walls;
        self.key_position = new_game.key_position;
        self.door_position = new_game.door_position;
        self.reset_position();

        // Reset timer state completely
        self.moves = 0;
        self.time_remaining = 300;
        self.last_tick = js_sys::Date::now() / 1000.0;

        // Force timer display update
        if let Some(timer_el) = self.document.get_element_by_id("timer") {
            timer_el.set_text_content(Some("5:00"));
        }

        // Update display
        self.render().expect("Failed to render reset");
    }
    fn reset_position(&mut self) {
        let old_pos = self.current_position;
        self.current_position = self.start_position;
        self.visited.clear();
        self.visited.insert(self.start_position);
        self.has_key = false;

        // Update only the changed cells rather than the entire grid.
        let maze = self
            .document
            .get_element_by_id("maze")
            .expect("Maze element not found");

        // Helper closure to update a specific cell.
        let update_cell = |x: usize, y: usize| -> Result<(), JsValue> {
            let index = (y * self.size + x) as u32;
            if let Some(cell) = maze.children().item(index) {
                self.update_cell_state(&cell, x, y)?;
            }
            Ok(())
        };

        // Update the old cell if it differs from start_position.
        if old_pos != self.start_position {
            let _ = update_cell(old_pos.0, old_pos.1);
        }
        let _ = update_cell(self.start_position.0, self.start_position.1);
    }
    #[wasm_bindgen]
    pub fn reset_to_level_one(&mut self) -> Result<(), JsValue> {
        // Only reset if above level 1
        if self.size > 2 {
            self.size = 2; // Level 1 starts with size 2
            self.level = 1;

            // Create new level 1 maze
            let new_game = Self::create_maze(self.size, self.document.clone());
            self.walls = new_game.walls;
            self.key_position = new_game.key_position;
            self.door_position = new_game.door_position;

            // Reset position and timer
            self.reset_position();
            self.time_remaining = 300;
            self.last_tick = js_sys::Date::now() / 1000.0;

            // Update displays
            if let Some(level_el) = self.document.get_element_by_id("level") {
                level_el.set_text_content(Some("1"));
            }
            if let Some(el) = self.document.get_element_by_id("moves") {
                el.set_text_content(Some("0"));
            }
            if let Some(timer_el) = self.document.get_element_by_id("timer") {
                timer_el.set_text_content(Some("5:00"));
            }

            // Show/hide reset button based on level
            if let Some(reset_btn) = self.document.get_element_by_id("reset-level") {
                reset_btn.set_attribute("hidden", "")?;
            }

            // Save state
            self.save_state()?;
            self.render()?;
        }
        Ok(())
    }
}
