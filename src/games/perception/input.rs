use super::Perception;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::Element;

impl Perception {
    pub(super) fn setup_click_handler(game_state: Rc<RefCell<Self>>) -> Result<(), JsValue> {
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
                                            .is_some_and(|cell| cell.is_same_node(Some(element)))
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
        if let Some(maze_el) = game_state.borrow().document.get_element_by_id("maze") {
            maze_el.add_event_listener_with_callback(
                "click",
                click_handler.as_ref().unchecked_ref(),
            )?;
            click_handler.forget();
        }

        Ok(())
    }
}
