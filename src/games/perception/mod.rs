mod state;
mod render;
mod maze;
mod movement;
mod timer;

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element};

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

#[wasm_bindgen]
impl Perception {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
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
                Self::create_maze(2, document)
            } else {
                serde_wasm_bindgen::from_value(js_sys::JSON::parse(&state)?)?
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
        Self::setup_timer(game_state)?;
        
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
        self.current_position = (0, 0);
        self.visited.clear();
        self.visited.insert((0, 0));
        self.has_key = false;
        self.render().expect("Failed to render position reset");
    }
}