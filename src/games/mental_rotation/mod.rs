mod tile;
mod level_generator;
mod grid;
mod timer;
mod rotation;

use serde::{Deserialize, Serialize};
use tile::Direction;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, MouseEvent};
use web_sys::{Document, Window};
use wasm_bindgen::JsCast;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME_INSTANCE: Mutex<Option<MentalRotation>> = Mutex::new(None);
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct MentalRotation {
    level: usize,
    tiles: Vec<tile::Tile>,
    initial_tiles: Vec<tile::Tile>, // Store initial tile configuration
    solution_path_tiles: Vec<usize>, // Add this line
    grid_size: usize,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    moves: usize,
    time_remaining: u32,
    #[serde(skip)]
    last_click_time: f64,
}

#[wasm_bindgen]
impl MentalRotation {
    #[wasm_bindgen(constructor)]
    #[must_use] pub fn new(level: usize) -> Self {
        // Try to load saved state first if level is 1 (initial loading)
        if level == 1 {
            if let Some(saved) = load_saved_game_state() {
                return saved;
            }
        }
        
        // Otherwise create a new game
        let grid_size = level; // Keep the original grid size calculation
        
        // Use proper tuple destructuring to get all values from generate_level
        let (tiles, solution_path_tiles, start_pos, end_pos) = level_generator::generate_level(level);
        let initial_tiles = tiles.clone(); // Store initial configuration
        
        Self {
            level,
            tiles,
            initial_tiles,
            solution_path_tiles,
            grid_size,
            start_pos,
            end_pos,
            moves: 0,
            time_remaining: 180,
            last_click_time: 0.0,
        }
    }

    fn get_arrow_classes(tile: &tile::Tile) -> String {
        let rotation_class = match tile.rotation {
            90 => "pointing-down",
            180 => "pointing-left", 
            270 => "pointing-up",
            _ => "pointing-right",
        };
        
        if tile.reversed {
            format!("arrow {rotation_class} flipped")
        } else {
            format!("arrow {rotation_class}")
        }
    }

    pub fn handle_click(&mut self, event: &MouseEvent, tile_idx: usize) {
        if event.button() == 0 {
            self.rotate_tile(tile_idx);
        } else if event.button() == 2 {
            self.reverse_tile(tile_idx);
        }
        self.moves += 1;
        
        // Check win after every move
        if self.check_win() {
            self.trigger_win_animation();
        }
    }
    
    fn rotate_tile(&mut self, tile_idx: usize) {
        // Check if rotation would cause a collision or go out of bounds
        if self.is_valid_rotation(tile_idx) {
            // Safe to rotate
            if let Some(tile) = self.tiles.get_mut(tile_idx) {
                tile.rotate();
            }
        }
        // If not valid, silently do nothing
    }

    // Helper function to check if a tile rotation would be valid
    fn is_valid_rotation(&self, tile_idx: usize) -> bool {
        if let Some(tile) = self.tiles.get(tile_idx) {
            // Create a copy of the tile with the proposed rotation
            let mut rotated_tile = tile.clone();
            rotated_tile.rotate();
            
            // Get the proposed rotated coordinates
            let rotated_coords = rotation::rotate_coordinates(&tile.cells, 90);
            
            // Check for grid boundaries
            for &(x, y) in &rotated_coords {
                if x >= self.grid_size || y >= self.grid_size {
                    return false; // Out of bounds
                }
            }
            
            // Check for collision with other tiles
            for (other_idx, other_tile) in self.tiles.iter().enumerate() {
                if other_idx != tile_idx { // Don't check against itself
                    for &coord in &rotated_coords {
                        if other_tile.cells.contains(&coord) {
                            return false; // Collision with another tile
                        }
                    }
                }
            }
            
            return true; // No collisions or bounds issues
        }
        
        false // Default to false if tile doesn't exist
    }

    fn reverse_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
            tile.reverse();
        }
    }

    fn check_win(&self) -> bool {
        // A winning position is when all tiles in the path create a continuous path
        // from start to end with arrows correctly connected
        
        // First, check that every tile in the path has the correct effective direction
        for &tile_idx in &self.solution_path_tiles {
            if let Some(tile) = self.tiles.get(tile_idx) {
                // Check the effective direction is East (there may be multiple ways to achieve this)
                if tile.get_effective_direction() != Direction::East {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // All solution tiles have the correct effective direction
        true
    }

    fn trigger_win_animation(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Add animating class to prevent interaction
                if let Some(container) = document.query_selector(".grid-container").ok().flatten() {
                    let _ = container.class_list().add_1("animating");
                }
                if let Some(rocket) = document.query_selector(".rocket").ok().flatten() {
                    let _ = rocket.class_list().add_1("moving");
                }
                
                // Progress to next level after animation
                let next_level = self.level + 1;
                let closure = Closure::once_into_js(move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            // Remove animation classes first
                            if let Some(container) = document.query_selector(".grid-container").ok().flatten() {
                                let _ = container.class_list().remove_1("animating");
                            }
                            
                            // Create new game instance first
                            let next_game = MentalRotation::new(next_level);
                            
                            // Update game instance
                            if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
                                *lock = Some(next_game);
                            }
                            
                            // Update level display
                            if let Some(level_display) = document.query_selector(".level").ok().flatten() {
                                level_display.set_text_content(Some(&format!("Level {next_level}")));
                            }
                            
                            // Start new level
                            if let Ok(lock) = GAME_INSTANCE.try_lock() {
                                if let Some(game) = &*lock {
                                    let _ = game.start();
                                }
                            }
                        }
                    }
                });
                
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    3000 // Match animation duration
                );
            }
        }
    }

    pub fn start(&self) -> Result<(), JsValue> {
        // First check if another instance is already running and clean it up
        if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
            if let Some(old_game) = lock.take() {
                old_game.clear_game_state();
            }
        }
        
        // Setup grid and timer first
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // Update level display
        if let Some(level_display) = document.query_selector(".level").ok().flatten() {
            level_display.set_text_content(Some(&format!("Level {}", self.level)));
        }
        
        self.setup_grid(&document)?;
        self.setup_timer(&window)?;
        self.setup_reset_button(&document)?;
        
        // Update game instance last
        if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
            *lock = Some(self.clone());
            // Save state when starting a level
            self.save_state();
        }
        
        Ok(())
    }

    fn setup_grid(&self, document: &Document) -> Result<(), JsValue> {
        let grid = document.get_element_by_id("grid").unwrap();
        
        // Clear existing grid content
        while let Some(child) = grid.first_child() {
            grid.remove_child(&child)?;
        }
        
        // Use the stored grid_size directly (which should match level)
        grid.set_attribute("style", &format!("grid-template-columns: repeat({}, 3rem)", self.grid_size))?;
        
        // Add contextmenu prevention using closure
        let context_callback = Closure::wrap(Box::new(|e: Event| {
            e.prevent_default();
            e.stop_propagation();
        }) as Box<dyn FnMut(Event)>);
        
        grid.add_event_listener_with_callback(
            "contextmenu",
            context_callback.as_ref().unchecked_ref(),
        )?;
        context_callback.forget();

        // Setup grid cells
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let cell = document.create_element("div")?;
                cell.set_class_name("cell");
                
                // Handle tiles
                for (tile_idx, tile) in self.tiles.iter().enumerate() {
                    if tile.cells.contains(&(x, y)) {
                        cell.set_class_name("cell tile");
                        cell.set_attribute("data-position", &format!("{x}{y}"))?;
                        let arrow = document.create_element("span")?;
                        arrow.set_class_name(&MentalRotation::get_arrow_classes(tile));
                        arrow.set_text_content(Some("➔"));
                        cell.append_child(&arrow)?;
                        cell.set_attribute("data-tile", &tile_idx.to_string())?;
                        break;
                    }
                }
                
                grid.append_child(&cell)?;
            }
        }

        // Remove any existing start/end indicators
        if let Some(existing_rocket) = document.query_selector(".rocket")? {
            existing_rocket.remove();
        }
        if let Some(existing_earth) = document.query_selector(".earth")? {
            existing_earth.remove();
        }

        // Add rocket and earth at correct positions
        let grid_container = document.query_selector(".grid-container")?.unwrap();
        
        // Create rocket element
        let rocket = document.create_element("span")?;
        rocket.set_class_name("rocket");
        rocket.set_text_content(Some("🚀"));
        
        // Adjust rocket vertical position to align with start_pos if necessary
        if self.grid_size > 1 {
            let start_y_percent = (self.start_pos.1 as f64 / (self.grid_size - 1) as f64) * 100.0;
            rocket.set_attribute("style", &format!("top: {}%;", start_y_percent))?;
        }
        
        grid_container.append_child(&rocket)?;

        // Create earth element
        let earth = document.create_element("span")?;
        earth.set_class_name("earth");
        earth.set_text_content(Some("🌍"));
        
        // Adjust earth vertical position to align with end_pos if necessary
        if self.grid_size > 1 {
            let end_y_percent = (self.end_pos.1 as f64 / (self.grid_size - 1) as f64) * 100.0;
            earth.set_attribute("style", &format!("top: {}%;", end_y_percent))?;
        }
        
        grid_container.append_child(&earth)?;

        // Create a click handler that doesn't capture the document reference
        // Instead, we'll use window.document() inside the closure
        let click_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            // Get target element and ensure it's a tile
            let target = event.target().unwrap();
            let element = target.dyn_ref::<Element>().unwrap();
            let tile_element = if element.class_list().contains("tile") {
                element.clone()
            } else if let Some(parent) = element.parent_element() {
                if parent.class_list().contains("tile") {
                    parent
                } else {
                    return;
                }
            } else {
                return;
            };

            // Process tile click
            if let Some(tile_idx) = tile_element.get_attribute("data-tile") {
                if let Ok(idx) = tile_idx.parse::<usize>() {
                    if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
                        if let Some(mut game) = lock.take() {
                            // Debounce using last click time
                            let now = js_sys::Date::now();
                            if now - game.last_click_time < 100.0 {
                                *lock = Some(game);
                                return;
                            }
                            game.last_click_time = now;

                            game.handle_click(&event, idx);
                            
                            // Get document from window inside the closure
                            if let Some(window) = web_sys::window() {
                                if let Some(document) = window.document() {
                                    if let Some(tile) = game.tiles.get(idx) {
                                        for &(x, y) in &tile.cells {
                                            let selector = format!(".cell[data-position='{x}{y}'] .arrow");
                                            if let Some(arrow) = document.query_selector(&selector).ok().flatten() {
                                                arrow.set_class_name(&MentalRotation::get_arrow_classes(tile));
                                            }
                                        }
                                    }
                                }
                            }
                            
                            game.save_state();
                            *lock = Some(game);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        // Single event listener on grid
        grid.add_event_listener_with_callback(
            "mousedown",
            click_callback.as_ref().unchecked_ref(),
        )?;
        click_callback.forget();

        Ok(())
    }

    fn setup_timer(&self, window: &Window) -> Result<(), JsValue> {
        timer::setup_timer(window, self.time_remaining)
    }

    fn setup_reset_button(&self, document: &Document) -> Result<(), JsValue> {
        if let Some(reset_button) = document.get_element_by_id("reset") {
            // Use window.document() inside closure instead of capturing the document parameter
            let reset_callback = Closure::wrap(Box::new(move |_: Event| {
                if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
                    if let Some(mut game) = lock.take() {
                        // Reset tiles to initial configuration without affecting moves or timer
                        game.tiles = game.initial_tiles.clone();
                        
                        // Update game instance first
                        *lock = Some(game.clone());
                        
                        // Only redraw the grid, not the timer
                        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                            let _ = game.setup_grid(&document);
                        }
                        
                        // Save state with reset tiles
                        game.save_state();
                    }
                }
            }) as Box<dyn FnMut(Event)>);
            
            reset_button.add_event_listener_with_callback(
                "click",
                reset_callback.as_ref().unchecked_ref(),
            )?;
            reset_callback.forget();
        }
        
        Ok(())
    }

    fn save_state(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(json_state) = serde_json::to_string(self) {
                    let _ = storage.set_item("mental_rotation_state", &json_state);
                }
            }
        }
    }

    pub fn clear_game_state(&self) {
        // Clear any saved state to prevent loading it immediately again
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.remove_item("mental_rotation_state");
            }
            
            // Clear any existing timer to prevent multiple timers running simultaneously
            if let Some(handle) = unsafe { timer::TIMER_HANDLE } {
                window.clear_interval_with_handle(handle);
                unsafe { timer::TIMER_HANDLE = None; }
            }
        }
    }

    #[wasm_bindgen(getter)]
    #[must_use] pub fn grid_size(&self) -> usize {
        self.grid_size
    }
}

fn load_saved_game_state() -> Option<MentalRotation> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(saved_state)) = storage.get_item("mental_rotation_state") {
                match serde_json::from_str::<MentalRotation>(&saved_state) {
                    Ok(game) => {
                        return Some(game);
                    },
                    Err(_) => {
                        // Silently fail and return None
                    }
                }
            }
        }
    }
    None
}
