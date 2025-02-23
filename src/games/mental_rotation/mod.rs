mod tile;
mod tile_generator;

use serde::{Deserialize, Serialize};
use tile::Direction;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, MouseEvent};
use std::collections::HashSet;
use web_sys::{Document, HtmlElement, Window};
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
}

#[wasm_bindgen]
impl MentalRotation {
    #[wasm_bindgen(constructor)]
    pub fn new(level: usize) -> Self {
        let grid_size = level;
        Self {
            level,
            tiles: tile_generator::generate_initial_tiles(level),  // This function should now make arrows face South
            grid_size,
            start_pos: (0, grid_size/2),
            end_pos: (grid_size-1, grid_size/2),
            moves: 0,
            time_remaining: 180,
        }
    }

    fn get_arrow_classes(&self, tile: &tile::Tile) -> String {
        let rotation_class = match tile.rotation {
            0 => "rotate-0",
            90 => "rotate-90",
            180 => "rotate-180",
            270 => "rotate-270",
            _ => "rotate-0",
        };
        
        if tile.reversed {
            format!("arrow {} reverse", rotation_class)
        } else {
            format!("arrow {}", rotation_class)
        }
    }

    pub fn handle_click(&mut self, event: MouseEvent, tile_idx: usize) {
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
        if let Some(tile) = self.tiles.iter().find(|t| t.cells.contains(&self.start_pos)) {
            let effective_direction = tile.get_effective_direction();
            // Debug print to help diagnose
            web_sys::console::log_1(&format!("Direction: {:?}", effective_direction).into());
            
            // For level 1, must point east
            if self.level == 1 {
                match effective_direction {
                    Direction::East => true,
                    _ => false
                }
            } else {
                false
            }
        } else {
            false
        }
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
                                level_display.set_text_content(Some(&format!("Level {}", next_level)));
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
        
        self.setup_grid(&document)?;
        self.setup_timer(&window)?;
        
        // Update game instance last
        if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
            *lock = Some(self.clone());
        }
        
        Ok(())
    }

    fn setup_grid(&self, document: &Document) -> Result<(), JsValue> {
        let grid = document.get_element_by_id("grid").unwrap();
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
                        cell.set_attribute("data-position", &format!("{}{}", x, y))?;
                        let arrow = document.create_element("span")?;
                        arrow.set_class_name(&self.get_arrow_classes(tile));
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
            let target = event.target().unwrap();
            let element = target.dyn_ref::<Element>().unwrap();
            
            // Find the closest cell element
            let cell = if element.class_list().contains("cell") {
                element.clone()
            } else if let Some(cell) = element.closest(".cell").unwrap() {
                cell
            } else {
                return;
            };

            if let Some(tile_idx) = cell.get_attribute("data-tile") {
                if let Ok(idx) = tile_idx.parse::<usize>() {
                    if let Ok(mut lock) = GAME_INSTANCE.try_lock() {
                        if let Some(mut game) = lock.take() {
                            game.handle_click(event, idx);
                            if let Some(tile) = game.tiles.get(idx) {
                                for &(x, y) in &tile.cells {
                                    let selector = format!(".cell[data-position='{}{}'] .arrow", x, y);
                                    if let Some(arrow) = document.query_selector(&selector).ok().flatten() {
                                        arrow.set_class_name(&game.get_arrow_classes(tile));
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
        
        grid.add_event_listener_with_callback(
            "mousedown",
            click_callback.as_ref().unchecked_ref(),
        )?;
        click_callback.forget();
        
        Ok(())
    }

    fn setup_timer(&self, window: &Window) -> Result<(), JsValue> {
        let timer_element = window.document()
            .unwrap()
            .query_selector(".timer")
            .unwrap()
            .unwrap();
            
        let mut time = self.time_remaining;
        let closure = Closure::wrap(Box::new(move || {
            if time > 0 {
                time -= 1;
                let mins = time / 60;
                let secs = time % 60;
                timer_element.set_text_content(Some(&format!("{}:{:02}", mins, secs)));
            }
        }) as Box<dyn FnMut()>);
        
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )?;
        closure.forget();
        
        Ok(())
    }

    fn load_state(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(state)) = storage.get_item("mental_rotation_state") {
                    if let Ok(game) = serde_json::from_str(&state) {
                        *GAME_INSTANCE.lock().unwrap() = Some(game);
                    }
                }
            }
        }
    }

    fn save_state(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("mental_rotation_state", &serde_json::to_string(self).unwrap());
            }
        }
    }

    #[wasm_bindgen(getter)]
    pub fn grid_size(&self) -> usize {
        self.grid_size
    }
}
