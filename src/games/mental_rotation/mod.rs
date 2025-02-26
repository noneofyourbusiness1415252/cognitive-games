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
        let grid_size = level;
        Self {
            level,
            tiles: level_generator::generate_level(level),
            grid_size,
            start_pos: (0, grid_size/2),
            end_pos: (grid_size-1, grid_size/2),
            moves: 0,
            time_remaining: 180,
            last_click_time: 0.0,
        }
    }

    fn get_arrow_classes(tile: &tile::Tile) -> String {
        // Get the effective direction (already handles rotation and reversal)
        let effective_dir = tile.get_effective_direction();
        
        let direction_class = match effective_dir {
            Direction::North => "pointing-up",
            Direction::South => "pointing-down",
            Direction::East => "pointing-right",
            Direction::West => "pointing-left",
            Direction::NorthEast => "pointing-northeast",
            Direction::NorthWest => "pointing-northwest",
            Direction::SouthEast => "pointing-southeast",
            Direction::SouthWest => "pointing-southwest",
        };
        
    pub fn handle_click(&mut self, event: &MouseEvent, tile_idx: usize) {ctive_direction handles it
        web_sys::console::log_1(&format!("handle_click called for tile {}, button {}, level {}", 
            tile_idx, event.button(), self.level).into());
        
        if event.button() == 0 {f, event: &MouseEvent, tile_idx: usize) {
            self.rotate_tile(tile_idx);!("handle_click called for tile {}, button {}, level {}", 
        } else if event.button() == 2 {elf.level).into());
            self.reverse_tile(tile_idx);
        }f event.button() == 0 {
        self.moves += 1;tile(tile_idx);
        } else if event.button() == 2 {
        // Check win after every movex);
        if self.check_win() {
            self.trigger_win_animation();
        }
    }   // Check win after every move
        if self.check_win() {
    fn rotate_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
            tile.rotate();
        }
    }n rotate_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
    fn reverse_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
            tile.reverse();
        }
    }n reverse_tile(&mut self, tile_idx: usize) {
        if let Some(tile) = self.tiles.get_mut(tile_idx) {
    fn check_win(&self) -> bool {
        // Start from the start position and traverse the path
        let mut current_pos = self.start_pos;
        let mut visited_positions: Vec<(usize, usize)> = vec![current_pos];
        let mut last_direction = None;
        let mut current_tile_idx = None; and traverse the path
        let mut current_pos = self.start_pos;
        // Maximum path length to prevent infinite loops vec![current_pos];
        let max_iterations = self.grid_size * self.grid_size;
        let mut current_tile_idx = None;
        for _ in 0..max_iterations {
            // Find the current tileevent infinite loops
            let mut found_tile = false;size * self.grid_size;
            for (idx, tile) in self.tiles.iter().enumerate() {
                if tile.cells.contains(&current_pos) {
                    current_tile_idx = Some(idx);
                    found_tile = true;;
                    , tile) in self.tiles.iter().enumerate() {
                    // Get effective direction from this tile
                    let effective_direction = tile.get_effective_direction();
                    web_sys::console::log_1(&format!("At pos {:?}, direction: {:?}", current_pos, effective_direction).into());
                    
                    // Calculate next position based on the direction
                    let next_pos = match effective_direction {ve_direction();
                        Direction::North => {format!("At pos {:?}, direction: {:?}", current_pos, effective_direction).into());
                            if current_pos.1 == 0 { return false; } // Out of bounds
                            (current_pos.0, current_pos.1 - 1)rection
                        },xt_pos = match effective_direction {
                        Direction::South => {
                            if current_pos.1 >= self.grid_size - 1 { return false; } // Out of bounds
                            (current_pos.0, current_pos.1 + 1)
                        },
                        Direction::East => {{
                            if current_pos.0 >= self.grid_size - 1 { return false; } // Out of bounds
                                // Reached the right edge - check if it's the end position
                                return current_pos.1 == self.end_pos.1;
                            }tion::East => {
                            (current_pos.0 + 1, current_pos.1) - 1 { 
                        },      // Reached the right edge - check if it's the end position
                        Direction::West => {nt_pos.1 == self.end_pos.1;
                            if current_pos.0 == 0 { return false; } // Out of bounds or back to start
                            (current_pos.0 - 1, current_pos.1)
                        },
                        // Handle diagonal directions
                        Direction::NorthEast => { { return false; } // Out of bounds or back to start
                            if current_pos.1 == 0 || current_pos.0 >= self.grid_size - 1 { return false; }
                            (current_pos.0 + 1, current_pos.1 - 1)
                        }, Handle diagonal directions
                        Direction::NorthWest => {
                            if current_pos.1 == 0 || current_pos.0 == 0 { return false; }{ return false; }
                            (current_pos.0 - 1, current_pos.1 - 1)
                        },
                        Direction::SouthEast => {
                            if current_pos.1 >= self.grid_size - 1 || current_pos.0 >= self.grid_size - 1 { return false; }
                            (current_pos.0 + 1, current_pos.1 + 1)
                        },
                        Direction::SouthWest => {
                            if current_pos.1 >= self.grid_size - 1 || current_pos.0 == 0 { return false; }{ return false; }
                            (current_pos.0 - 1, current_pos.1 + 1)
                        },
                    };  Direction::SouthWest => {
                            if current_pos.1 >= self.grid_size - 1 || current_pos.0 == 0 { return false; }
                    // Check if we're going in circlest_pos.1 + 1)
                    if visited_positions.contains(&next_pos) {
                        return false;
                    }
                    // Check if we're going in circles
                    // Move to next positiontains(&next_pos) {
                    current_pos = next_pos;
                    visited_positions.push(current_pos);
                    last_direction = Some(effective_direction);
                    break;e to next position
                }   current_pos = next_pos;
            }       visited_positions.push(current_pos);
                    last_direction = Some(effective_direction);
            // If no tile contains the current position, path is broken
            if !found_tile {
                web_sys::console::log_1(&"No tile at current position".into());
                return false;
            }/ If no tile contains the current position, path is broken
            if !found_tile {
            // Check if we've reached the end positionurrent position".into());
            if current_pos.0 == self.end_pos.0 && current_pos.1 == self.end_pos.1 {
                web_sys::console::log_1(&"Reached end position!".into());into());
                return true;return true;
            }/ Check if we've reached the end position
        }   if current_pos.0 == self.end_pos.0 && current_pos.1 == self.end_pos.1 {
        
        // If we've gone through too many iterations, there might be a loop
        web_sys::console::log_1(&"Too many iterations - possible loop".into());ble loop".into());
        falserid = document.get_element_by_id("grid").unwrap();
    }   
        // Clear existing grid content
    fn trigger_win_animation(&self) {first_child() {
        if let Some(window) = web_sys::window() {e(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Add animating class to prevent interaction
                if let Some(container) = document.query_selector(".grid-container").ok().flatten() {ze))?;ainer) = document.query_selector(".grid-container").ok().flatten() {
                    let _ = container.class_list().add_1("animating");
                }ntextmenu prevention using closure
        let context_callback = Closure::wrap(Box::new(|e: Event| {        if let Some(rocket) = document.query_selector(".rocket").ok().flatten() {
        grid.add_event_listener_with_callback(g");
            "contextmenu",tion();
            context_callback.as_ref().unchecked_ref(),
        )?;  // Progress to next level after animation
        context_callback.forget();th_callback(elf.level + 1;
            "contextmenu",                let closure = Closure::once_into_js(move || {
        // Setup grid cellsk.as_ref().unchecked_ref(),Some(window) = web_sys::window() {
        for y in 0..self.grid_size {nt() {
            for x in 0..self.grid_size {n classes first
                let cell = document.create_element("div")?;
                cell.set_class_name("cell");).remove_1("animating");
                 0..self.grid_size {            }
                // Handle tilesid_size {
                for (tile_idx, tile) in self.tiles.iter().enumerate() {
                    if tile.cells.contains(&(x, y)) {w(next_level);
                        cell.set_class_name("cell tile");
                        cell.set_attribute("data-position", &format!("{x}{y}"))?;
                        let arrow = document.create_element("span")?; {ock() {
                        arrow.set_class_name(&MentalRotation::get_arrow_classes(tile));
                        arrow.set_text_content(Some("➔"));
                        cell.append_child(&arrow)?;sition", &format!("{x}{y}"))?;
                        cell.set_attribute("data-tile", &tile_idx.to_string())?;
                        break;set_class_name(&MentalRotation::get_arrow_classes(tile)); let Some(level_display) = document.query_selector(".level").ok().flatten() {
                    }   arrow.set_text_content(Some("➔"));           level_display.set_text_content(Some(&format!("Level {next_level}")));
                }       cell.append_child(&arrow)?;           }
                        cell.set_attribute("data-tile", &tile_idx.to_string())?;            
                grid.append_child(&cell)?;
            }       }               if let Ok(lock) = GAME_INSTANCE.try_lock() {
        }       }                       if let Some(game) = &*lock {
                                                    let _ = game.start();
                grid.append_child(&cell)?;
            }
        }
           }
        // Remove any existing start/end indicators
        if let Some(existing_rocket) = document.query_selector(".rocket")? {
            existing_rocket.remove();       let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        }                    closure.as_ref().unchecked_ref(),
        if let Some(existing_earth) = document.query_selector(".earth")? {
            existing_earth.remove();
        }

        // Add rocket and earth at correct positions
        let grid_container = document.query_selector(".grid-container")?.unwrap();
        let rocket = document.create_element("span")?;    pub fn start(&self) -> Result<(), JsValue> {
        rocket.set_class_name("rocket");
        rocket.set_text_content(Some("🚀"));.unwrap();
        grid_container.append_child(&rocket)?;wrap();

        let earth = document.create_element("span")?;        self.setup_grid(&document)?;
        earth.set_class_name("earth");
        earth.set_text_content(Some("🌍"));
        grid_container.append_child(&earth)?;
NSTANCE.try_lock() {
        // Clone document for use in callback);
        let document = document.clone();        }
        let click_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            // Get target element and ensure it's a tilent: &Document) -> Result<(), JsValue> {
            let target = event.target().unwrap();
            let element = target.dyn_ref::<Element>().unwrap();
            let tile_element = if element.class_list().contains("tile") {grid content
                element.clone()hild) = grid.first_child() {
            } else if let Some(parent) = element.parent_element() {ld(&child)?;
                if parent.class_list().contains("tile") {
                    parent
                } else {ute("style", &format!("grid-template-columns: repeat({}, 3rem)", self.grid_size))?;
                    return;
                }        // Add contextmenu prevention using closure
            } else {osure::wrap(Box::new(|e: Event| {
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
                                *lock = Some(game);        for y in 0..self.grid_size {
                                return;
                            }
                            game.last_click_time = now;

                            game.handle_click(&event, idx);
                            if let Some(tile) = game.tiles.get(idx) {
                                for &(x, y) in &tile.cells {tains(&(x, y)) {
                                    let selector = format!(".cell[data-position='{x}{y}'] .arrow");class_name("cell tile");
                                    if let Some(arrow) = document.query_selector(&selector).ok().flatten() {set_attribute("data-position", &format!("{x}{y}"))?;
                                        arrow.set_class_name(&MentalRotation::get_arrow_classes(tile));reate_element("span")?;
                                    }entalRotation::get_arrow_classes(tile));
                                }rrow.set_text_content(Some("➔"));
                            }   cell.append_child(&arrow)?;
                            game.save_state();       cell.set_attribute("data-tile", &tile_idx.to_string())?;
                            *lock = Some(game);           break;
                        }
                    }                }
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        // Single event listener on grid
        grid.add_event_listener_with_callback(tart/end indicators
            "mousedown",        if let Some(existing_rocket) = document.query_selector(".rocket")? {
            click_callback.as_ref().unchecked_ref(),isting_rocket.remove();
        )?;   }
        click_callback.forget();        if let Some(existing_earth) = document.query_selector(".earth")? {

        Ok(())
    }
        // Add rocket and earth at correct positions
    fn setup_timer(&self, window: &Window) -> Result<(), JsValue> { = document.query_selector(".grid-container")?.unwrap();
        timer::setup_timer(window, self.time_remaining)n")?;
    }

    fn save_state(&self) {container.append_child(&rocket)?;
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {   let earth = document.create_element("span")?;
                let _ = storage.set_item("mental_rotation_state", &serde_json::to_string(self).unwrap());        earth.set_class_name("earth");
            }ent(Some("🌍"));
        }
    }
   // Clone document for use in callback
    #[wasm_bindgen(getter)]       let document = document.clone();
    #[must_use] pub fn grid_size(&self) -> usize {        let click_callback = Closure::wrap(Box::new(move |event: MouseEvent| {




}    }        self.grid_size




























}    Ok(())        }        reset_callback.forget();        )?;            reset_callback.as_ref().unchecked_ref(),            "click",        reset_button.add_event_listener_with_callback(                }) as Box<dyn FnMut(_)>);            }                }                    }                        let _ = location.reload();                    if let Some(location) = window.location().ok() {                    // Reload the page to reset the game                                        let _ = storage.remove_item("mental_rotation_state");                    // Remove saved game state to force a fresh start                if let Some(storage) = window.local_storage().ok().flatten() {            if let Some(window) = web_sys::window() {        let reset_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {    if let Some(reset_button) = document.get_element_by_id("reset") {    // Setup reset button functionality        // ...existing code...fn setup_grid(&self, document: &Document) -> Result<(), JsValue> {            event.prevent_default();
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

    fn save_state(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("mental_rotation_state", &serde_json::to_string(self).unwrap());
            }
        }
    }

    #[wasm_bindgen(getter)]
    #[must_use] pub fn grid_size(&self) -> usize {
        self.grid_size
    }
}
