use super::Perception;
use wasm_bindgen::prelude::*;
use web_sys::Element;

impl Perception {
    pub(crate) fn render(&self) -> Result<(), JsValue> {
        let maze = self.document.get_element_by_id("maze").unwrap();
    
        // Only regenerate grid if size changed
        if maze.children().length() as usize != self.size * self.size {
            maze.set_attribute(
                "style",
                &format!("grid-template-columns: repeat({}, 60px)", self.size),
            )?;
            
            // Clear existing content safely
            while let Some(child) = maze.first_child() {
                maze.remove_child(&child)?;
            }
    
            // Create cells only once
            for _ in 0..(self.size * self.size) {
                let cell = self.document.create_element("div")?;
                cell.set_class_name("cell");
                let span = self.document.create_element("span")?;
                let content = self.document.create_text_node("");
                cell.append_child(&content)?;
                cell.append_child(&span)?;
                maze.append_child(&cell)?;
            }
        }
    
        // Update existing cells
        for y in 0..self.size {
            for x in 0..self.size {
                let index = (y * self.size + x) as u32;
                if let Some(cell) = maze.children().item(index) {
                    self.update_cell_state(&cell, x, y)?;
                }
            }
        }
    
        // Update stats
        if let Some(level_el) = self.document.get_element_by_id("level") {
            level_el.set_text_content(Some(&self.level.to_string()));
        }
        if let Some(completed_el) = self.document.get_element_by_id("completed") {
            completed_el.set_text_content(Some(&self.mazes_completed.to_string()));
        }
        if let Some(timer_el) = self.document.get_element_by_id("timer") {
            let minutes = self.time_remaining / 60;
            let seconds = self.time_remaining % 60;
            timer_el.set_text_content(Some(&format!("{}:{:02}", minutes, seconds)));
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
        }
    
        // Update content
        let content = if (x, y) == self.key_position && !self.has_key {
            "🔑"
        } else if (x, y) == self.current_position && self.has_key {
            "🔑"
        } else if (x, y) == self.door_position {
            "🚪"
        } else {
            ""
        };
    
        // Update text content if it's different
        if let Some(first_child) = cell.first_child() {
            if first_child.text_content().unwrap_or_default() != content {
                first_child.set_text_content(Some(content));
            }
        }
    
        Ok(())
    }
}