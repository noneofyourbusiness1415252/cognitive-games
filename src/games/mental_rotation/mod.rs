mod tile;
mod tile_generator;

use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, MouseEvent};
use std::collections::HashSet;
use web_sys::{Document, HtmlElement, Window};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
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
        let grid_size = level;  // Grid size is now equal to level
        Self {
            level,
            tiles: tile_generator::generate_initial_tiles(level),  // Pass level to generator
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
        self.check_win();
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
        // Basic implementation - will be expanded later
        false
    }

    pub fn start(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        self.setup_grid(&document)?;
        self.setup_timer(&window)?;
        self.load_state();
        
        Ok(())
    }

    fn setup_grid(&self, document: &Document) -> Result<(), JsValue> {
        let grid = document.get_element_by_id("grid").unwrap();
        grid.set_attribute("style", &format!("grid-template-columns: repeat({}, 3rem)", self.grid_size))?;
        
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let cell = document.create_element("div")?;
                cell.set_class_name("cell");
                
                for (tile_idx, tile) in self.tiles.iter().enumerate() {
                    if tile.cells.contains(&(x, y)) {
                        cell.set_class_name("cell tile");
                        let arrow = document.create_element("span")?;
                        arrow.set_class_name(&self.get_arrow_classes(tile));
                        arrow.set_text_content(Some("➔"));
                        cell.append_child(&arrow)?;
                        cell.set_attribute("data-tile", &tile_idx.to_string())?;
                        break;
                    }
                }
                
                if x == self.start_pos.0 && y == self.start_pos.1 {
                    let arrow = document.create_element("span")?;
                    arrow.set_class_name("arrow");
                    arrow.set_text_content(Some("➔"));
                    cell.append_child(&arrow)?;
                }
                
                grid.append_child(&cell)?;
            }
        }

        // Add click handlers
        let grid_clone = grid.clone();
        let click_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(cell) = target.dyn_ref::<Element>() {
                    if let Some(tile_idx) = cell.get_attribute("data-tile") {
                        if let Ok(idx) = tile_idx.parse::<usize>() {
                            // Get game instance and handle click
                            // TODO: Implement instance retrieval
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        
        grid_clone.add_event_listener_with_callback(
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
            
        let time = self.time_remaining;
        let closure = Closure::wrap(Box::new(move || {
            let mins = time / 60;
            let secs = time % 60;
            timer_element.set_text_content(Some(&format!("{}:{:02}", mins, secs)));
        }) as Box<dyn FnMut()>);
        
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )?;
        closure.forget();
        
        Ok(())
    }

    fn load_state(&self) {
        // TODO: Implement state loading from localStorage
    }

    fn save_state(&self) {
        // TODO: Implement state saving to localStorage
    }

    #[wasm_bindgen(getter)]
    pub fn grid_size(&self) -> usize {
        self.grid_size
    }
}
