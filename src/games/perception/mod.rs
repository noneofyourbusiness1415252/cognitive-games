mod state;
mod render;
mod maze;
mod movement;

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
                            let new_game = Self::create_maze(game.size, game.document.clone());
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
                            timer_el.set_text_content(Some(&format!("{}:{:02}", minutes, seconds)));
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