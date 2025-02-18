use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event, HtmlElement};

mod expression;
mod level;
mod state;

use expression::Expression;
use level::Level;
use state::GameState;

#[wasm_bindgen]
pub struct Numeracy {
    state: Rc<RefCell<GameState>>, // Changed to Rc<RefCell<>> for shared ownership
    document: Document,
    container: HtmlElement,
}

#[wasm_bindgen]
impl Numeracy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Numeracy, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document
            .get_element_by_id("game-container")
            .unwrap()
            .dyn_into::<HtmlElement>()?;

        let game = Numeracy {
            state: Rc::new(RefCell::new(GameState::new())),
            document: document.clone(),
            container,
        };

        // Set up visibility change handler
        let state = game.state.clone();
        let doc = document.clone();
        let visibility_callback = Closure::wrap(Box::new(move || {
            let hidden = doc.hidden();
            let mut game_state = state.borrow_mut();
            game_state.is_visible = !hidden;
            
            if !hidden {
                // When becoming visible, restart the current level
                game_state.start_round();
            }
        }) as Box<dyn FnMut()>);

        document.set_onvisibilitychange(Some(visibility_callback.as_ref().unchecked_ref()));
        visibility_callback.forget();

        Ok(game)
    }

    fn render_bubbles(&self) -> Result<(), JsValue> {
        let state_ref = self.state.borrow();
        let children = self.container.children();

        for i in 0..children.length() {
            if let Some(bubble) = children.item(i) {
                let bubble: Element = bubble.dyn_into::<Element>()?;

                if let Some(expr) = state_ref.expressions.get(i as usize) {
                    bubble.set_text_content(Some(&expr.text));

                    let class = if state_ref.selected_indices.contains(&(i as usize)) {
                        "bubble selected"
                    } else {
                        "bubble"
                    };
                    bubble.set_attribute("class", class)?;

                    let state = self.state.clone();
                    let bubble_ref = bubble.clone();
                    let i = i as usize;
                    let handler = Closure::wrap(Box::new(move |_event: Event| {
                        let mut state = state.borrow_mut();
                        if state.toggle_selection(i) {
                            let is_selected = state.selected_indices.contains(&i);
                            let class = if is_selected {
                                "bubble selected"
                            } else {
                                "bubble"
                            };
                            bubble_ref.set_attribute("class", class).unwrap();

                            if state.selected_indices.len() == 3 {
                                let round_success = state.check_current_round();
                                state.update_score(round_success);
                                state.start_round();
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    bubble.add_event_listener_with_callback(
                        "click",
                        handler.as_ref().unchecked_ref(),
                    )?;
                    handler.forget();
                }
            }
        }
        Ok(())
    }

    fn update_stats(&self) -> Result<(), JsValue> {
        let state = self.state.borrow();

        if let Some(level_elem) = self.document.get_element_by_id("level") {
            level_elem.set_text_content(Some(&state.level.number.to_string()));
        }

        // Removed score element update
        Ok(())
    }

    fn update_timer(&self) -> Result<(), JsValue> {
        let state = self.state.borrow();

        if let Some(timer_elem) = self.document.get_element_by_id("timer") {
            if state.is_visible {
                if let Some(remaining) = state.get_round_time_remaining() {
                    let seconds = (remaining / 1000.0) as u32;
                    let text = format!("{}:{:02}", seconds / 60, seconds % 60);
                    timer_elem.set_text_content(Some(&text));
                }
            } else {
                timer_elem.set_text_content(Some("Paused"));
            }
        }
        Ok(())
    }

    fn start_timer(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let state = self.state.clone();
        let document = self.document.clone();
        let container = self.container.clone();

        let closure = Closure::wrap(Box::new(move || {
            let this = Numeracy {
                state: state.clone(),
                document: document.clone(),
                container: container.clone(),
            };
            this.update_timer().unwrap();
            this.check_time_limits().unwrap();
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )?;

        closure.forget();
        Ok(())
    }

    fn check_time_limits(&self) -> Result<(), JsValue> {
        {
            let mut state = self.state.borrow_mut();
            
            // Only check time limits if the tab is visible
            if state.is_visible {
                if let Some(remaining) = state.get_round_time_remaining() {
                    if remaining <= 0.0 {
                        state.update_score(false);
                        state.start_round();
                    }
                }
            }
        }
        self.render_bubbles()?;
        self.update_stats()?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn start(&self) -> Result<(), JsValue> {
        self.state.borrow_mut().start_level();
        self.render_bubbles()?;
        self.update_stats()?;
        self.start_timer()?;
        Ok(())
    }
}
