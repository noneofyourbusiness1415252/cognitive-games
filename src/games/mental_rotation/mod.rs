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
        let grid_size = level;
        let tiles = level_generator::generate_level(level);
        let initial_tiles = tiles.clone(); // Store initial configuration
        
        Self {
            level,
            tiles,
            initial_tiles,
            grid_size,
            start_pos: (0, grid_size/2),
            end_pos: (grid_size-1, grid_size/2),
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
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
            tile.rotate();
        }
    }

    fn reverse_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
            tile.reverse();
        }
    }

    fn check_win(&self) -> bool {
        // Find tile at start position
        if let Some(start_tile) = self.tiles.iter().find(|t| t.cells.contains(&self.start_pos)) {
            // Begin path traversal from start position
            let mut current_pos = self.start_pos;
            let mut visited = vec![current_pos];
            
            loop {
                // Find the current tile
                if let Some(current_tile) = self.tiles.iter().find(|t| t.cells.contains(&current_pos)) {
                    // Get effective direction from the tile
                    let direction = current_tile.get_effective_direction();
                    
                    // Calculate next position based on direction
                    let next_pos = match direction {
                        Direction::North => (current_pos.0, current_pos.1.saturating_sub(1)),
                        Direction::South => (current_pos.0, current_pos.1 + 1),
                        Direction::East => (current_pos.0 + 1, current_pos.1),
                        Direction::West => (current_pos.0.saturating_sub(1), current_pos.1),
                        Direction::NorthEast => (current_pos.0 + 1, current_pos.1.saturating_sub(1)),
                        Direction::NorthWest => (current_pos.0.saturating_sub(1), current_pos.1.saturating_sub(1)),
                        Direction::SouthEast => (current_pos.0 + 1, current_pos.1 + 1),
                        Direction::SouthWest => (current_pos.0.saturating_sub(1), current_pos.1 + 1),
                    };
                    
                    // Check if we've reached the end
                    if next_pos == self.end_pos {
                        return true;
                    }
                    
                    // Check if next position is valid
                    if next_pos.0 >= self.grid_size || next_pos.1 >= self.grid_size {
                        // Out of bounds
                        break;
                    }
                    
                    // Check if we're going in circles
                    if visited.contains(&next_pos) {
                        // Cycle detected
                        break;
                    }
                    
                    // Check if there's a tile at the next position
                    if !self.tiles.iter().any(|t| t.cells.contains(&next_pos)) {
                        // No connecting tile
                        break;
                    }
                    
                    // Move to next position
                    current_pos = next_pos;
                    visited.push(current_pos);
                } else {
                    // Current position doesn't have a tile
                    break;
                }
            }
        }
        
        // No valid path found
        false
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
        let rocket = document.create_element("span")?;
        rocket.set_class_name("rocket");
        rocket.set_text_content(Some("🚀"));
        grid_container.append_child(&rocket)?;

        let earth = document.create_element("span")?;
        earth.set_class_name("earth");
        earth.set_text_content(Some("🌍"));
        grid_container.append_child(&earth)?;

        // Clone document for use in callback
        let document = document.clone();
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
                            if let Some(tile) = game.tiles.get(idx) {
                                for &(x, y) in &tile.cells {
                                    let selector = format!(".cell[data-position='{x}{y}'] .arrow");
                                    if let Some(arrow) = document.query_selector(&selector).ok().flatten() {
                                        arrow.set_class_name(&MentalRotation::get_arrow_classes(tile));
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
            // Clone document for use in callback
            let document = document.clone();
            
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
