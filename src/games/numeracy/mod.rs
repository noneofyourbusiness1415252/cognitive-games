use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Event};

mod expression;
mod level;
mod state;

use expression::Expression;
use level::Level;
use state::GameState;

#[wasm_bindgen]
pub struct Numeracy {
    state: GameState,
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

        Ok(Numeracy {
            state: GameState::new(),
            document,
            container,
        })
    }

    fn render_bubbles(&self) -> Result<(), JsValue> {
        self.container.set_inner_html("");
        
        for (i, expr) in self.state.expressions.iter().enumerate() {
            let bubble = self.document.create_element("div")?;
            bubble.set_class_name("bubble");
            bubble.set_text_content(Some(&expr.text));
            
            let index = i.to_string();
            bubble.set_attribute("data-index", &index)?;
            
            if self.state.selected_indices.contains(&i) {
                bubble.set_attribute("class", "bubble selected")?;
            }
            
            let handler = Closure::wrap(Box::new(move |_event: Event| {
                // Selection logic will be handled here
            }) as Box<dyn FnMut(_)>);
            
            bubble.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
            handler.forget();
            
            self.container.append_child(&bubble)?;
        }
        Ok(())
    }

    fn update_stats(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        if let Some(level_elem) = document.get_element_by_id("level") {
            level_elem.set_text_content(Some(&self.state.level.number.to_string()));
        }
        
        if let Some(score_elem) = document.get_element_by_id("score") {
            score_elem.set_text_content(Some(&self.state.score.to_string()));
        }
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        self.state.start_level();
        self.render_bubbles()?;
        self.update_stats()?;
        Ok(())
    }
}